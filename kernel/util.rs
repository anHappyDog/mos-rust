use core::cell::RefCell;

use alloc::rc::Rc;

#[repr(C)]
pub struct ListNode {
    pub next: Option<Rc<RefCell<ListNode>>>,
    pub prev: Option<Rc<RefCell<ListNode>>>,
    pub idx: usize,
}

#[repr(C)]
pub struct DoubleLinkedList {
    pub head: Option<Rc<RefCell<ListNode>>>,
    pub tail: Option<Rc<RefCell<ListNode>>>,
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
    pub fn insert_to_tail(&mut self, node: Rc<RefCell<ListNode>>) {
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
    pub fn insert_to_head(&mut self, node: Rc<RefCell<ListNode>>) {
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            let head = self.head.take().unwrap();
            head.borrow_mut().prev = Some(node.clone());
            node.borrow_mut().next = Some(head.clone());
            self.head = Some(node);
        }
    }
    pub fn push(&mut self, node: Rc<RefCell<ListNode>>) {
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
    pub fn remove(&mut self, node: Rc<RefCell<ListNode>>) {
        let prev = node.borrow().prev.clone();
        let next = node.borrow().next.clone();
        if prev.clone().is_none() && next.clone().is_none() {
            return;
        }
        if let Some(prev) = prev.clone() {
            prev.borrow_mut().next.clone_from(&next);
        } else {
            self.head.clone_from(&next);
        }
        if let Some(next) = next.clone() {
            next.borrow_mut().prev.clone_from(&prev);
        } else {
            self.tail.clone_from(&prev);
        }
        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;
    }
    pub fn pop(&mut self) -> Option<Rc<RefCell<ListNode>>> {
        self.head.as_ref()?;
        let head = self.head.take().unwrap();
        if let Some(next) = head.borrow().next.clone() {
            next.borrow_mut().prev = None;
            self.head = Some(next);
        } else {
            self.tail = None;
            self.head = None;
        }
        head.borrow_mut().next = None;
        head.borrow_mut().prev = None;
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
    0
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
    dst
}

