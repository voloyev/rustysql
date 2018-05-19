pub fn run(query: &String) -> Vec<&str> {
    analize(query)
}

fn analize(query: &String) -> Vec<&str> {
    return query.as_str().split(" ").collect();
}

#[test]
fn test_analyze() {
    let some_str = String::from("select 1 from db");
    let result = analize(&some_str);
    assert_eq!(result, vec!["select", "1", "from", "db"]);
}
