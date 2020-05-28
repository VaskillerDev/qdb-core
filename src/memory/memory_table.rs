use crate::memory::memory_machine::MemoryMachine;
use crate::memory::compared::Compared;
use qdb_ast::ast::types::{BinaryExpr, DataType, DataVar};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
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
    pub fn find_by_predicate(&self, binary_expr: &BinaryExpr) -> Option<Vec<Vec<DataType>>> {
        fn get_symbol(data_type: &DataType) -> Option<&String> {
            return match data_type {
                DataType::Symbol(val) => Some(val),
                _ => None,
            };
        }
        fn get_operator(operator : &str) -> fn(&Ordering) -> bool {
            return match operator {
                "==" => <DataType as Compared>::eq,
                "!=" => <DataType as Compared>::neq,
                ">=" => <DataType as Compared>::eq_or_gr,
                ">" => <DataType as Compared>::gr,
                "<=" => <DataType as Compared>::eq_or_le,
                "<" => <DataType as Compared>::le,
                _ => {panic!("Error in operator detection")}
            }
        }

        let (left, right, operator) = binary_expr.get();

        {
            let maybe_l_value = get_symbol(left);
            let maybe_r_value = get_symbol(right);

            if maybe_l_value.is_some() && maybe_r_value.is_some() {
                return None;
            }

            if maybe_l_value.is_some() {
                let value = maybe_l_value.unwrap();
                let maybe_mem_machine = self.mem.get(value);
                if maybe_mem_machine.is_some() {
                    let mem_machine = maybe_mem_machine.unwrap();
                    let predicate = get_operator(operator);
                    let a = mem_machine.get_by_compare_with(right,|this,other|
                        DataType::comparing(this,other,predicate)
                    );
                  let mut vec : Vec<Vec<DataType>> = Vec::new();
                  let a =  *a.get(0).unwrap();
                    for (_,mem) in self.mem.iter() {
                        vec.push(mem.get_values_by_range_inclusive(a))
                    }
                    if vec.is_empty() {return None}
                    return Some(vec)
                }
            }
        }

        None
    }
}

mod test {
    use crate::memory::memory_table::MemoryTable;
    use qdb_ast::ast::types::{BinaryExpr, DataType, DataVar};

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

    #[test]
    fn test_memory_find_by_predicate() {
        let mut mem_table = MemoryTable::init();
        mem_table.insert("my_val", DataType::Int(10));
        mem_table.insert("my_val", DataType::Int(10));
        mem_table.insert("my_val2", DataType::Int(64));
        mem_table.insert("my_val2", DataType::Int(32));

        let binary_expr = BinaryExpr::new(
            DataType::Symbol("my_val".to_string()),
            DataType::Int(10),
            "==".to_string(),
        );
       let a = mem_table.find_by_predicate(&binary_expr);
        println!("{:?}",a);
    }
}
