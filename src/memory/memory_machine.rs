use crate::memory::compared::Compared;
use crate::memory::intersection::Intersection;
use qdb_ast::ast::types::DataType;
use rbtree::{Iter, RBTree};
use std::borrow::{BorrowMut, Cow};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::ops::RangeInclusive;

pub type Indexes = Vec<RangeInclusive<i64>>;

#[derive(Debug)]
pub struct MemoryMachine {
    mem: RBTree<DataType, Indexes>,
    logic_time: i64,
}

impl MemoryMachine {
    // To initialize empty MemoryMachine with clear tree map
    // and logic time equal 0 (original number).
    pub fn init() -> Self {
        MemoryMachine {
            mem: RBTree::new(),
            logic_time: 0,
        }
    }

    // To insert data_type in tree map. Where key - data_type value, value - vec indexes.
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

    // To get indexes by data_type value key from tree map.
    pub fn get(&self, data_type: &DataType) -> Option<Indexes> {
        Some(self.mem.get(&data_type).unwrap().clone())
    }

    // To get data_type by indexes from tree map.
    pub fn get_values_by_range_inclusive(&self, range_inclusive: &Indexes) -> Vec<DataType> {
        let mut vec: Vec<DataType> = Vec::new();
        for (value, key) in self.mem.iter() {
            if Vec::intersect(range_inclusive, key) {
                vec.push(value.clone());
            }
        }
        return vec;
    }

    // To get last value from memory machine
    pub fn get_last_value(&self) -> Option<&DataType> {
        let last_index = self.logic_time - 1;
        for (symbol, indexes) in self.mem.iter() {
            for range in indexes {
                if range.contains(&last_index) {
                    return Some(symbol);
                }
            }
        }
        None
    }

    // To get vector of indexes filter by predicate from tree map
    // where f(a,b) := a x b, where x ∃ {==,!=,>=,>,<=,<}
    // then ∀a ∈ A
    pub fn get_by_compare_with<F: Fn(&DataType, &DataType) -> bool>(
        &self,
        other: &DataType,
        predicate: F,
    ) -> Vec<&Indexes> {
        let mut vec: Vec<&Indexes> = vec![];
        for i in self.mem.iter() {
            if predicate(i.0, other) {
                vec.push(i.1);
            }
        }
        vec
    }
}

impl Intersection for Indexes {
    // intersection in each element in vec
    // if a ∈ A && a ∈ B => A ⋂ B
    fn intersect(left: &Indexes, right: &Indexes) -> bool {
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

impl Compared for DataType {
    fn compare_with_override(&self, other: Self) -> Option<Ordering> {
        unimplemented!()
    }
}

impl Compared for &DataType {
    fn compare_with_override(&self, other: Self) -> Option<Ordering> {
        self.compare_with(&other)
    }
}

mod test {
    use crate::memory::compared::Compared;
    use crate::memory::intersection::Intersection;
    use crate::memory::memory_machine::{Indexes, MemoryMachine};
    use qdb_ast::ast::types::DataType;
    use std::cmp::Ordering;
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
    fn test_get_last_value() {
        let mut memory_machine = MemoryMachine::init();

        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);

        memory_machine.insert(DataType::Real(32.0));

        debug_assert_eq!(
            &DataType::Real(32.0),
            memory_machine.get_last_value().unwrap()
        );
    }

    #[test]
    fn test_memory_machine_get_compare_with() {
        let mut memory_machine = MemoryMachine::init();

        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Null);
        memory_machine.insert(DataType::Real(35.0));
        memory_machine.insert(DataType::Real(35.01));

        let result = memory_machine.get_by_compare_with(&DataType::Real(35.0), |this, other| {
            DataType::comparing(this, other, <DataType as Compared>::eq)
        });

        //println!("{:?}",result)
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
        let a_range: Indexes = vec![RangeInclusive::new(0, 2), RangeInclusive::new(4, 6)];
        let b_range: Indexes = vec![RangeInclusive::new(0, 1), RangeInclusive::new(2, 3)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Indexes = vec![RangeInclusive::new(2, 4), RangeInclusive::new(5, 8)];
        let b_range: Indexes = vec![RangeInclusive::new(3, 5), RangeInclusive::new(8, 10)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Indexes = vec![RangeInclusive::new(0, 1)];
        let b_range: Indexes = vec![RangeInclusive::new(0, 0)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(true, result);

        let a_range: Indexes = vec![RangeInclusive::new(32, 55), RangeInclusive::new(58, 93)];
        let b_range: Indexes = vec![RangeInclusive::new(0, 2), RangeInclusive::new(8, 10)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);

        let a_range: Indexes = vec![RangeInclusive::new(32, 55), RangeInclusive::new(58, 93)];
        let b_range: Indexes = vec![RangeInclusive::new(93, 108), RangeInclusive::new(110, 120)];
        let result = Vec::intersect(&a_range, &b_range);
        debug_assert_eq!(false, result);
    }
}
