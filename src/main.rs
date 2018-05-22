use dealer::deal_with_it;
use std::io;
use std::io::Write;

mod dealer;
mod table;
mod tokenizer;

pub use table::Table;

fn main() {
    let mut db = Table { row: Vec::new() };
    repl(&mut db);
}

fn repl(db: &mut Table) {
    let mut checker = true;
    let mut counter = 0;

    while checker {
        let input = read_line();
        if input.trim() == ".exit" {
            format!("Bye!\n");
            checker = false;
            continue;
        }

        let (input, write) = deal_with_it(input, db);

        if write {
            counter += 1;
            let serialized = tokenizer::run(&input);

            db.insert(
                counter,
                serialized[1].to_string(),
                serialized[2].to_string(),
            );
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

#[test]
#[should_panic]
fn check_panic() {
    pub use table::Row;

    let mut db = Table { row: Vec::new() };
    let input = String::from("select");
    let serialized = tokenizer::run(&input);
    db.insert(1, serialized[1].to_string(), serialized[2].to_string());
}
