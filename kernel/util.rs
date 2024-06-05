use core::cell::RefCell;

use alloc::sync::Arc;
use alloc::vec::Vec;
pub struct IndexStack {
    stack: Vec<usize>,
}

impl IndexStack {
    pub fn new() -> Self {
        IndexStack { stack: Vec::new() }
    }

    pub fn push(&mut self, index: usize) {
        self.stack.push(index);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.stack.pop()
    }
}

pub struct ListNode {
    pub next: Option<Arc<RefCell<ListNode>>>,
    pub prev: Option<Arc<RefCell<ListNode>>>,
    pub idx: usize,
}

pub struct DoubleLinkedList {
    pub head: Option<Arc<RefCell<ListNode>>>,
    pub tail: Option<Arc<RefCell<ListNode>>>,
}

impl ListNode {
    pub const fn new(idx: usize) -> Self {
        Self {
            next: None,
            prev: None,
            idx,
        }
    }
}

impl DoubleLinkedList {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    pub fn push(&mut self, node: Arc<RefCell<ListNode>>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            let tail = self.tail.take().unwrap();
            tail.borrow_mut().next = Some(node.clone());
            node.borrow_mut().prev = Some(tail.clone());
            self.tail = Some(node);
        }
    }

    pub fn pop(&mut self) -> Option<Arc<RefCell<ListNode>>> {
        if self.head.is_none() {
            return None;
        }
        let head = self.head.take().unwrap();
        if let Some(next) = head.borrow().next.clone() {
            next.borrow_mut().prev = None;
            self.head = Some(next);
        } else {
            self.tail = None;
            self.head = None;
        }
        Some(head)
    }
}


#[macro_export]
macro_rules! DEFINE_ELF_BYTES {
    ($var_name : ident,$path : literal) => {
        static $var_name: &[u8] = include_bytes!($path);
    };
}