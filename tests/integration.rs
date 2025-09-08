use bachelor_demo::semi_naive::reachable;

#[test]
fn reachable_transitive_demo() {
    // a->b->c->d
    let edges = [("a","b"), ("b","c"), ("c","d")];
    let got = reachable(&edges);

    // vi forventer minst disse:
    assert!(got.contains(&(String::from("a"), String::from("c"))));
    assert!(got.contains(&(String::from("a"), String::from("d"))));
    assert!(got.contains(&(String::from("b"), String::from("d"))));
}
