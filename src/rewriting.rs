use std::collections::HashMap;

/// Minimal demo-AST

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TermPattern {
    NamedNode(String),
    Literal(String),
    Variable(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    And(Box<Expression>, Box<Expression>),
    Var(String),
    Const(String),
}

#[derive(Clone, Debug)]
pub struct GraphAtom {
    pub subject: TermPattern,
    pub predicate: String,
    pub object: TermPattern,
}

#[derive(Clone, Debug)]
pub enum Atom {
    GraphAtom(GraphAtom),
    Bind { variable: String, expression: Expression },
    Filter { expr: Expression },
}

#[derive(Clone, Debug)]
pub enum BodyFormula {
    Atom { atom: Atom },
    PValues { variables: Vec<String>, bindings_parameter: String },
}

#[derive(Clone, Debug)]
pub struct DatalogRule {
    pub head: String,
    pub body: Vec<BodyFormula>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphPattern {
    Bgp { patterns: Vec<(String, String, String)> },
    Extend { inner: Box<GraphPattern>, variable: String, expression: Expression },
    Filter { inner: Box<GraphPattern>, expr: Expression },
}


/// Helper: konverter TermPattern til en streng for BGP (demo)
fn tp_to_str(tp: &TermPattern) -> String {
    match tp {
        TermPattern::NamedNode(s) => s.clone(),
        TermPattern::Literal(s) => s.clone(),
        TermPattern::Variable(v) => format!("?{}", v),
    }
}

/// Demo: sammenslå flere filtere til én AND
fn combine_filters(filters: Vec<Expression>) -> Option<Expression> {
    let mut it = filters.into_iter();
    let first = it.next()?;
    Some(it.fold(first, |acc, e| Expression::And(Box::new(acc), Box::new(e))))
}

/// -------- Dynamic rewrite (demo) --------
/// Gjør triple-mønstre parameteriserbare slik at vi kan mate inn bindings ved runtime.
/// Returnerer (ny-regel, bindings-map).
pub fn create_dynamic_cache_rewrite(rule: &DatalogRule) -> (DatalogRule, HashMap<String, (String, Option<String>, Option<String>)>) {
    let mut new_body = Vec::new();
    let mut map: HashMap<String, (String, Option<String>, Option<String>)> = HashMap::new();
    let mut i = 0;

    for bf in &rule.body {
        if let BodyFormula::Atom { atom } = bf {
            if let Atom::GraphAtom(g) = atom {
                // Finn variabler i subject/object
                let mut variables: Vec<String> = Vec::new();
                let mut subj_name: Option<String> = None;
                let mut obj_name: Option<String> = None;

                if let TermPattern::Variable(v) = &g.subject {
                    subj_name = Some(v.clone());
                    variables.push(v.clone());
                }
                if let TermPattern::Variable(v) = &g.object {
                    obj_name = Some(v.clone());
                    variables.push(v.clone());
                }

                // bindings-parameter for runtime
                let bp = format!("dynamic_{}", i);
                i += 1;

                // Erstatt triple med en PValues-node i body
                new_body.push(BodyFormula::PValues {
                    variables,
                    bindings_parameter: bp.clone(),
                });

                // Registrer hvordan man mater denne
                map.insert(bp, (g.predicate.clone(), subj_name, obj_name));
                continue;
            }
        }
        new_body.push(bf.clone());
    }

    let rewritten = DatalogRule { head: rule.head.clone(), body: new_body };
    (rewritten, map)
}

/// -------- Static rewrite (demo) --------
/// Trekker rene BIND/FILTER nærmere planen og pakker alt i en kompakt BGP/EXTEND/FILTER.
pub fn create_static_rewrite(rule: &DatalogRule) -> GraphPattern {
    let mut triple_patterns = Vec::new();
    let mut filters = Vec::new();
    let mut binds: Vec<(String, Expression)> = Vec::new();

    for bf in &rule.body {
        if let BodyFormula::Atom { atom } = bf {
            match atom {
                Atom::GraphAtom(g) => {
                    triple_patterns.push((
                        tp_to_str(&g.subject),
                        g.predicate.clone(),
                        tp_to_str(&g.object),
                    ));
                }
                Atom::Bind { variable, expression } => {
                    binds.push((variable.clone(), expression.clone()));
                }
                Atom::Filter { expr } => {
                    filters.push(expr.clone());
                }
            }
        }
    }

    let mut gp = GraphPattern::Bgp { patterns: triple_patterns };

    for (v, e) in binds {
        gp = GraphPattern::Extend {
            inner: Box::new(gp),
            variable: v,
            expression: e,
        };
    }

    if let Some(expr) = combine_filters(filters) {
        gp = GraphPattern::Filter { inner: Box::new(gp), expr };
    }

    gp
}
