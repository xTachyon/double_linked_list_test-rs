pub struct Node {
    pub next: usize,
    pub prec: usize,
    pub value: u64,
}
pub struct DoubleLinkedList {
    data: Vec<Node>,
    pub head: usize,
    pub tail: usize,
}
impl DoubleLinkedList {
    pub const INVALID_INDEX: usize = usize::MAX;
    pub fn new() -> Self {
        let mut me = Self {
            data: Vec::with_capacity(256),
            head: 0,
            tail: 0,
        };
        me.data.push(Node {
            next: DoubleLinkedList::INVALID_INDEX,
            prec: DoubleLinkedList::INVALID_INDEX,
            value: 0,
        });
        me
    }
    pub fn add(&mut self, value: u64) {
        let new_node = Node {
            next: DoubleLinkedList::INVALID_INDEX,
            prec: self.tail,
            value,
        };
        self.data.push(new_node);
        let last_index = self.data.len() - 1;
        self.data[self.tail].next = last_index; // last index
        self.tail = last_index;
    }
    #[inline(always)]
    fn get_node(&self, index: usize) -> Option<&Node> {
        if index < self.data.len() {
            Some(&self.data[index])
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
