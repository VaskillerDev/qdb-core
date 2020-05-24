use qdb_ast::ast::types::DataType;
use std::borrow::{BorrowMut, Cow};
use std::collections::{BTreeMap, HashSet};
use std::ops::RangeInclusive;
use rbtree::RBTree;

#[derive(Debug)]
pub struct MemoryMachine {
    mem: RBTree<DataType, Vec<RangeInclusive<i64>>>,
    logic_time: i64,
}

impl MemoryMachine {
    pub fn init() -> Self {
        MemoryMachine {
            mem: RBTree::new(),
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
    pub fn get(&self, data_type: &DataType) -> Option<Vec<RangeInclusive<i64>>> {
        Some(self.mem.get(&data_type).unwrap().clone())
    }

    pub fn get_values_by_range_inclusive(
        &self,
        range_inclusive: &Vec<RangeInclusive<i64>>,
    ) -> Vec<DataType> {
        let mut vec: Vec<DataType> = Vec::new();
        for (value, key) in self.mem.iter() {
            if Vec::intersect(range_inclusive, key) {
                vec.push(value.clone());
            }
        }
        return vec;
    }
}

trait Intersection {
    fn intersect(left: &Self, right: &Self) -> bool;
}

impl Intersection for Vec<RangeInclusive<i64>> {
    // intersection in each element in vec
    // if a ∈ A && a ∈ B => A ⋂ B
    fn intersect(left: &Vec<RangeInclusive<i64>>, right: &Vec<RangeInclusive<i64>>) -> bool {
        let left_vec_len = left.len() - 1;
        let right_vec_len = right.len() - 1;

        if left_vec_len + 1 > 0 && right_vec_len + 1 > 0 {
            let left_range_start = left.get(0).unwrap();
            let left_range_end = left.get(left_vec_len).unwrap();

            let right_range_start = right.get(0).unwrap();
            let right_range_end = right.get(right_vec_len).unwrap();

            /*println!("lrs {:?} : lre {:?}",left_range_start,left_range_end);
            println!("rrs {:?} : rre {:?}",right_range_start,right_range_end);*/

            return RangeInclusive::intersect(left_range_start, right_range_start)
                || RangeInclusive::intersect(left_range_end, right_range_end);
        }
        false
    }
}

impl Intersection for RangeInclusive<i64> {
    // A ⋂ B
    fn intersect(_self: &RangeInclusive<i64>, other: &RangeInclusive<i64>) -> bool {
        if _self.contains(other.start()) || _self.contains(other.end()) {
            return true;
        }
        if other.contains(_self.start()) || other.contains(_self.end()) {
            return true;
        }

        false
    }
}

mod test {
    use crate::memory::memory_machine::{Intersection, MemoryMachine};
    use qdb_ast::ast::types::DataType;
    use std::ops::RangeInclusive;

    #[test]
    fn test_memory_machine() -> Result<(), ()> {
        let mut memory_machine = MemoryMachine::init();

        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);

        memory_machine.insert(DataType::Real(32.0));
        memory_machine.insert(DataType::Real(64.0));

        memory_machine.insert(DataType::Null);

        debug_assert_eq!(
            vec![0..=2, 5..=5],
            memory_machine.get(&DataType::Null).unwrap()
        );
        debug_assert_eq!(
            vec![3..=3],
            memory_machine.get(&DataType::Real(32.0)).unwrap()
        );
        debug_assert_eq!(
            vec![4..=4],
            memory_machine.get(&DataType::Real(64.0)).unwrap()
        );

        Ok(())
    }

    #[test]
    fn test_memory_machine_get_first_by_range_inclusive() {
        let mut memory_machine = MemoryMachine::init();

        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Real(32.0));
        memory_machine.insert(DataType::Real(32.01));
        memory_machine.insert(DataType::Real(32.06));
        memory_machine.insert(DataType::Real(32.07));
        memory_machine.insert(DataType::Text("my text".to_string()));
        memory_machine.insert(DataType::Real(32.09));
        memory_machine.insert(DataType::Real(32.0));
        memory_machine.insert(DataType::Null);

        let result_a =
            memory_machine.get_values_by_range_inclusive(&vec![RangeInclusive::new(0, 5)]);
        let result_b =
            memory_machine.get_values_by_range_inclusive(&vec![RangeInclusive::new(5, 8)]);
        println!("{:?}", result_a);
        println!("{:?}", result_b);
    }

    #[test]
    fn test_range_intersection() {
        let a_range = RangeInclusive::new(1, 5);
        let b_range = RangeInclusive::new(3, 4);
        let result = RangeInclusive::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range = RangeInclusive::new(1, 5);
        let b_range = RangeInclusive::new(0, 2);
        let result = RangeInclusive::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range = RangeInclusive::new(6, 9);
        let b_range = RangeInclusive::new(1, 2);
        let result = RangeInclusive::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);

        let a_range = RangeInclusive::new(6, 9);
        let b_range = RangeInclusive::new(10, 12);
        let result = RangeInclusive::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);
    }

    #[test]
    fn test_vec_range_intersection() {
        let a_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(0, 2), RangeInclusive::new(4, 6)];
        let b_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(0, 1), RangeInclusive::new(2, 3)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(2, 4), RangeInclusive::new(5, 8)];
        let b_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(3, 5), RangeInclusive::new(8, 10)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Vec<RangeInclusive<i64>> = vec![RangeInclusive::new(0, 1)];
        let b_range: Vec<RangeInclusive<i64>> = vec![RangeInclusive::new(0, 0)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(32, 55), RangeInclusive::new(58, 93)];
        let b_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(0, 2), RangeInclusive::new(8, 10)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);

        let a_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(32, 55), RangeInclusive::new(58, 93)];
        let b_range: Vec<RangeInclusive<i64>> =
            vec![RangeInclusive::new(93, 108), RangeInclusive::new(110, 120)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);
    }
}
