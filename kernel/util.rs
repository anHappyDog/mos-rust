use alloc::vec::Vec;
use sync::spin::Spinlock;

pub struct IndexStack {
    stack: Vec<usize>,
}

impl IndexStack {
    pub fn new() -> Self {
        IndexStack {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, index: usize) {
        self.stack.push(index);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
}
