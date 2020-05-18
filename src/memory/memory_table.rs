use crate::memory::memory_machine::MemoryMachine;
use std::collections::HashSet;

/*
struct MemoryTable {
    mem: Hash
}*/

fn m () {
    let mut a : HashSet<i32> = HashSet::new();
    a.insert(22);
    //let b = a.
    //println!("{}",b.unwrap())
}

mod test {
    use crate::memory::memory_table::m;

    #[test]
    fn test_m() {
        m()
    }
}