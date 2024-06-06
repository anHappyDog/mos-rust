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

#[no_mangle]
extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = unsafe { *s1.add(i) };
        let b = unsafe { *s2.add(i) };
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    return 0;
}

#[no_mangle]
extern "C" fn memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    if dst < src as *mut u8 {
        while i < n {
            unsafe {
                *dst.add(i) = *src.add(i);
            }
            i += 1;
        }
    } else {
        i = n;
        while i > 0 {
            unsafe {
                *dst.add(i - 1) = *src.add(i - 1);
            }
            i -= 1;
        }
    }
    return dst;
}
