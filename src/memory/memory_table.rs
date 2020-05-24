use crate::memory::memory_machine::MemoryMachine;
use qdb_ast::ast::types::{DataType, DataVar};
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};

// variable storage
#[derive(Debug)]
struct MemoryTable {
    mem: HashMap<String, MemoryMachine>,
}

impl MemoryTable {
    pub fn init() -> Self {
        MemoryTable {
            mem: HashMap::new(),
        }
    }
    pub fn insert(&mut self, name_var: &str, value: DataType) {
        let maybe_mem_machine = self.mem.get_mut(name_var);
        if maybe_mem_machine.is_some() {
            let mut mem_machine = maybe_mem_machine.unwrap();
            mem_machine.insert(value);
        } else {
            let mut mem_machine = MemoryMachine::init();
            mem_machine.insert(value);
            self.mem.insert(name_var.to_string(), mem_machine);
        }
    }
    pub fn find(&self, var: &DataVar) -> Option<Vec<DataType>> {
        let (name, value) = var.get();
        let maybe_mem_machine = self.mem.get(name.as_str());
        if maybe_mem_machine.is_some() {
            let mem_machine = maybe_mem_machine.unwrap();
            let maybe_range = mem_machine.get(value);
            if maybe_range.is_some() {
                let range = maybe_range.unwrap();
                for (_, mem) in self.mem.iter() {
                    let data_types = mem.get_values_by_range_inclusive(&range);
                    if data_types.len() > 0 {
                        return Some(data_types);
                    }
                    return None;
                }
            }
        }
        None
    }
}

mod test {
    use crate::memory::memory_table::MemoryTable;
    use qdb_ast::ast::types::{DataType, DataVar};

    #[test]
    fn test_memory_table_insert() {
        let mut memory_table = MemoryTable::init();
        memory_table.insert("my_var", DataType::Text("mytext".to_string()));
        memory_table.insert("my_var", DataType::Null);
        memory_table.insert("my_var", DataType::Null);
        memory_table.insert("my_var", DataType::Real(56.01));
        memory_table.insert("my_var", DataType::Int(32));
        memory_table.insert("my_val", DataType::Null);
        println!("{:#?}", memory_table);
    }

    #[test]
    fn test_memory_table_find() {
        let mut memory_table = MemoryTable::init();
        memory_table.insert("my_var", DataType::Text("mytext".to_string()));
        memory_table.insert("my_var", DataType::Null);
        memory_table.insert("my_var2", DataType::Null);
        let data_var = DataVar::new("my_var".to_string(), DataType::Null);
        let a = memory_table.find(&data_var);
        println!("{:?}", a);
    }
}
