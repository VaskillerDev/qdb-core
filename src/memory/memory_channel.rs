use crate::memory::memory_table::MemoryTable;
use rbtree::RBTree;

pub type MemoryChannel = RBTree<String, MemoryTable>;
