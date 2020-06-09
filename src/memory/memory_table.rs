use crate::memory::compared::Compared;
use crate::memory::memory_machine::MemoryMachine;
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
    // hidden function
    fn get_symbol(data_type: &DataType) -> Option<&String> {
        return match data_type {
            DataType::Symbol(val) => Some(val),
            _ => None,
        };
    }

    fn get_operator(operator: &str) -> fn(&Ordering) -> bool {
        return match operator {
            "==" => <DataType as Compared>::eq,
            "!=" => <DataType as Compared>::neq,
            ">=" => <DataType as Compared>::eq_or_gr,
            ">" => <DataType as Compared>::gr,
            "<=" => <DataType as Compared>::eq_or_le,
            "<" => <DataType as Compared>::le,
            _ => panic!("Error in operator detection"),
        };
    }

    fn resolve_symbol_operator_get_values_by_range_inclusive(
        &self,
        maybe_left_symbol: Option<&String>,
        right: &DataType,
        operator: &str,
    ) -> Vec<PrintOfState> {
        let mut vec: Vec<PrintOfState> = Vec::new();

        if maybe_left_symbol.is_some() {
            let value = maybe_left_symbol.unwrap();

            let maybe_mem_machine = self.mem.get(value);

            if maybe_mem_machine.is_some() {
                let mem_machine = maybe_mem_machine.unwrap();
                let predicate = Self::get_operator(operator);

                let indexes = mem_machine.get_by_compare_with(right, |this, other| {
                    DataType::comparing(this, other, predicate)
                });

                let indexes = *indexes.get(0).unwrap(); // why .get(0) ?, what wrong ?
                for (key, mem) in self.mem.iter() {
                    let data_types = mem.get_values_by_range_inclusive(indexes);
                    vec.push(PrintOfState::new(key, data_types));
                }
            }
        }
        return vec;
    }

    // public function
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
    pub fn find_by_predicate(&self, binary_expr: &BinaryExpr) -> Option<Vec<PrintOfState>> {
        let (left, right, operator) = binary_expr.get();

        {
            let maybe_l_value = Self::get_symbol(left);
            let maybe_r_value = Self::get_symbol(right);

            if maybe_l_value.is_some() && maybe_r_value.is_some() {
                return None;
            }

            let result_from_l = self.resolve_symbol_operator_get_values_by_range_inclusive(
                maybe_l_value,
                right,
                operator,
            );
            let result_from_r = self.resolve_symbol_operator_get_values_by_range_inclusive(
                maybe_r_value,
                left,
                operator,
            );

            if !result_from_l.is_empty() {
                return Some(result_from_l);
            }
            if !result_from_r.is_empty() {
                return Some(result_from_r);
            }
            return None;
        }

        None
    }
}

mod test {
    use crate::memory::memory_table::{MemoryTable, PrintOfState};
    use qdb_ast::ast::types::{BinaryExpr, DataType, DataVar};

    #[test]
    fn test_memory_table_insert() {
        let mut memory_table = MemoryTable::init();
        memory_table.insert("A", DataType::Text("mytext".to_string()));
        memory_table.insert("A", DataType::Null);
        memory_table.insert("B", DataType::Null);
        memory_table.insert("A", DataType::Real(56.01));
        memory_table.insert("A", DataType::Int(32));
        memory_table.insert("B", DataType::Null);

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
        mem_table.insert("my_val", DataType::Int(101));
        mem_table.insert("my_val", DataType::Int(101));
        mem_table.insert("my_val2", DataType::Int(64));
        mem_table.insert("my_val2", DataType::Int(32));
        mem_table.insert("my_val3", DataType::Int(32));

        let binary_expr = BinaryExpr::new(
            DataType::Int(101),
            DataType::Symbol("my_val".to_string()),
            "==".to_string(),
        );

        let vec_print_of_state = mem_table.find_by_predicate(&binary_expr).unwrap();

        debug_assert_eq!(
            true,
            vec_print_of_state.contains(&PrintOfState::new(
                &"my_val".to_string(),
                vec![DataType::Int(101)]
            ))
        );
        debug_assert_eq!(
            true,
            vec_print_of_state.contains(&PrintOfState::new(
                &"my_val2".to_string(),
                vec![DataType::Int(32), DataType::Int(64)]
            ))
        );
        debug_assert_eq!(
            true,
            vec_print_of_state.contains(&PrintOfState::new(
                &"my_val3".to_string(),
                vec![DataType::Int(32)]
            ))
        );
    }
}
