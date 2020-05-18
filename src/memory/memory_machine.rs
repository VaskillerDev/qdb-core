use qdb_ast::ast::types::{DataType};
use std::borrow::{BorrowMut, Cow};
use std::collections::{BTreeMap};
use std::ops::{RangeInclusive};

#[derive(Debug)]
pub struct MemoryMachine {
    mem: BTreeMap<DataType, Vec<RangeInclusive<i64>>>,
    logic_time: i64,
}

impl MemoryMachine {
    pub fn init() -> Self {
        MemoryMachine {
            mem: BTreeMap::new(),
            logic_time: 0,
        }
    }
    pub fn insert(&mut self, data_type: DataType) {
        let key = self.mem.get(&data_type);

        if key.is_none() {
            self.mem.insert(
                data_type,
                vec![RangeInclusive::new(self.logic_time, self.logic_time)],
            );
        } else {
            let logic_time = self.logic_time.clone();
            let vec = self.mem.get_mut(&data_type).unwrap();
            let maybe_range = vec.iter_mut().find(|e| e.contains(&(logic_time - 1)));

            if maybe_range.is_some() {
                let range = maybe_range.unwrap();
                *range = RangeInclusive::new(*range.start(), self.logic_time);
            } else {
                vec.push(RangeInclusive::new(logic_time, logic_time));
            }
        }
        self.logic_time += 1;
    }
    pub fn get(&self, data_type: DataType) -> Option<Vec<RangeInclusive<i64>>> {
        Some(self.mem.get(&data_type).unwrap().clone())
    }
}

mod test {
    use qdb_ast::ast::types::DataType;
    use crate::memory::memory_machine::MemoryMachine;

    #[test]
    fn memory_machine_test() -> Result<(),()>{
        let mut memory_machine = MemoryMachine::init();

        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);

        memory_machine.insert(DataType::Real(32.0));
        memory_machine.insert(DataType::Real(64.0));

        memory_machine.insert(DataType::Null);

        debug_assert_eq!(vec![0..=2,5..=5],memory_machine.get(DataType::Null).unwrap());
        debug_assert_eq!(vec![3..=3],memory_machine.get(DataType::Real(32.0)).unwrap());
        debug_assert_eq!(vec![4..=4],memory_machine.get(DataType::Real(64.0)).unwrap());

        Ok(())
    }
}
