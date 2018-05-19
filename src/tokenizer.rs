pub fn tokenize(query: &String) -> Vec<&str> {
    analize(query)
}

fn analize(query: &String) -> Vec<&str> {
    return query.as_str().split(" ").collect();
}
