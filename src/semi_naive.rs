/// Minimal, illustrativ "semi-naiv" idé for transitiv lukning.
/// Input: kanter som par (u, v). Output: alle (x, y) der y er nåbar fra x.
pub fn reachable(edges: &[(&str, &str)]) -> Vec<(String, String)> {
    use std::collections::{HashMap, HashSet, VecDeque};

    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for (u, v) in edges {
        adj.entry(*u).or_default().push(*v);
    }

    let mut result: HashSet<(String, String)> = HashSet::new();
    for (&start, _) in &adj {
        let mut seen: HashSet<&str> = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(start);
        seen.insert(start);

        while let Some(u) = q.pop_front() {
            if let Some(nbrs) = adj.get(u) {
                for &w in nbrs {
                    if !seen.contains(w) {
                        seen.insert(w);
                        q.push_back(w);
                        result.insert((start.to_string(), w.to_string()));
                    }
                }
            }
        }
    }
    let mut out: Vec<_> = result.into_iter().collect();
    out.sort();
    out
}
