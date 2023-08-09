use std::collections::LinkedList;


pub struct DoubleLinkedList {
    list: LinkedList<u64>
}

impl DoubleLinkedList {
    pub fn new() -> DoubleLinkedList {
        DoubleLinkedList {
            list: LinkedList::new()
        }
    }
    pub fn add(&mut self, value: u64) {
        self.list.push_back(value);
    }
    pub fn sum_all(&self) -> u64 {
        self.list.iter().sum()
    }
}
