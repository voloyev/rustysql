#[derive(Debug)]
pub struct Row {
    id: i32,
    name: String,
    email: String,
}

pub struct Table {
    pub row: Vec<Row>,
}

impl Table {
    pub fn insert(&mut self, id: i32, name: String, email: String) {
        self.row.push(Row {
            id: id,
            name: name,
            email: email,
        })
    }

    pub fn count(&self) -> usize {
        self.row.len()
    }
}
