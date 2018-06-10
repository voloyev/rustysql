use std::collections::HashMap;
use std::io;
use std::io::Write;
mod tokenizer;

fn main() {
    let mut db = HashMap::new();
    repl(&mut db);
}

fn repl(db: &mut HashMap<i32, String>) {
    let mut checker = true;
    let mut counter = 0;

    while checker {
        let tmp = read_line();
        if tmp.trim() == ".exit" {
            format!("Bye!\n");
            checker = false;
            continue;
        }

        let (input, write) = check_input(tmp, db);

        if write {
            counter += 1;
            println!("{:?}", tokenizer::run(&input));
            db.insert(counter, input);
        } else {
            println!("{}", input);
        }
    }
}

fn read_line() -> String {
    print!("db_name > ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("somthing wrong");
    input
}

fn check_input(input: String, db: &mut HashMap<i32, String>) -> (String, bool) {
    if input.trim() == ".help" {
        return (
            String::from(
                "Available commands are: \n\n.exit\tto exit program\n.help\tto show this help\n",
            ),
            false,
        );
    }

    if input.trim() == ".select" {
        println!("{:?}", db);
        return (String::from(""), false);
    }

    return (input, true);
}

#[test]
fn check_help_input_test() {
    let mut db = HashMap::new();
    let a = check_input(String::from(".help"), &mut db);
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
fn check_select_input_test() {
    let mut db = HashMap::new();
    let a = check_input(String::from(".select"), &mut db);
    assert_eq!(a, (String::from(""), false))
}
