use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_HANDLE_UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Copy, Clone)]
pub struct Handle {
    index: u32,
    unique_id: u32,
}
impl Handle {
    pub const INVALID: Handle = Handle {
        index: 0xFFFFFFFF,
        unique_id: 0xFFFFFFFF,
    };
    pub fn new(index: u32) -> Handle {
        let unique_id =
            (GLOBAL_HANDLE_UNIQUE_ID.fetch_add(1, Ordering::SeqCst) as u32) % 0xFFFF_FFFE;
        Self { index, unique_id }
    }
}
pub struct Node {
    pub next: Handle,
    pub prec: Handle,
    pub value: u64,
    pub unique_id: u32,
}
pub struct DoubleLinkedList {
    data: Vec<Node>,
    pub head: Handle,
    pub tail: Handle,
}
impl DoubleLinkedList {
    pub fn new(capacity: usize) -> Self {
        let first_element_handle = Handle::new(0);
        let mut me = Self {
            data: Vec::with_capacity(capacity),
            head: first_element_handle,
            tail: first_element_handle,
        };
        me.data.push(Node {
            next: Handle::INVALID,
            prec: Handle::INVALID,
            value: 0,
            unique_id: first_element_handle.unique_id,
        });
        me
    }
    pub fn add(&mut self, value: u64) {
        let new_elem_index = self.data.len();
        let new_node_handle = Handle::new(new_elem_index as u32);
        let new_node = Node {
            next: Handle::INVALID,
            prec: self.tail,
            value,
            unique_id: new_node_handle.unique_id,
        };
        self.data.push(new_node);

        self.data[self.tail.index as usize].next = new_node_handle; // last index
        self.tail = new_node_handle;
    }
    #[inline(always)]
    fn get_node(&self, handle: Handle) -> Option<&Node> {
        let index = handle.index as usize;
        if index < self.data.len() {
            let obj = &self.data[index];
            if obj.unique_id == handle.unique_id {
                Some(&self.data[index])
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn sum_all(&self) -> u64 {
        let mut sum = 0;
        let mut current = self.head;
        while let Some(node) = self.get_node(current) {
            sum += node.value;
            current = node.next;
        }
        sum
    }
}
