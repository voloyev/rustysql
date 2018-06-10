//mod self::tokenizer;

pub use table::Table;

pub fn deal_with_it(input: String, db: &mut Table) -> (String, bool) {
    if input.trim() == ".help" {
        return (
            String::from(
                "Available commands are: \n\n.exit\tto exit program\n.help\tto show this help\n",
            ),
            false,
        );
    }

    if input.trim() == ".select" {
        println!("{:?}", db.row);
        return (String::from(""), false);
    }

    if input.trim() == ".count" {
        println!("{:?}", db.count());
        return (String::from(""), false);
    }

    return (input, true);
}

#[test]
fn help_input_test() {
    let mut db = Table { row: Vec::new() };

    let a = deal_with_it(String::from(".help"), &mut db);
    assert_eq!(
        a,
        (
            String::from(
                "Available commands are: \n\n.exit\tto exit program\n.help\tto show this help\n"
            ),
            false
        )
    )
}

#[test]
fn select_input_test() {
    let mut db = Table { row: Vec::new() };

    let a = deal_with_it(String::from(".select"), &mut db);
    assert_eq!(a, (String::from(""), false))
}
