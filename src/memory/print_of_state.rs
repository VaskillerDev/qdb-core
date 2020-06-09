use qdb_ast::ast::types::DataType;

#[derive(Debug, PartialOrd, PartialEq)]
struct PrintOfState {
    name: String,
    values: Vec<DataType>,
}

impl PrintOfState {
    fn new(name: &String, values: Vec<DataType>) -> Self {
        PrintOfState {
            name: name.to_string(),
            values,
        }
    }
}
