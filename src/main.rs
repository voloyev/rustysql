enum ExecuteResult {
    ExecuteSuccess,
    ExecuteTableFull,
}

enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

enum PrepareResult {
    PrepareSuccess,
    Preparesyntaxerror,
    PrepareUnrecognizedError,
}

enum StatementType {
    StatementInsert,
    StatementSelect,
}

fn main() {}
