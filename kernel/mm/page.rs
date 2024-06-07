use alloc::{boxed::Box, rc::Rc, vec::Vec};
use core::{
    cell::RefCell,
    ops::{Add, Sub},
    ptr,
};
use lazy_static::lazy_static;
use sync::spin::Spinlock;

use crate::util::{DoubleLinkedList, IndexStack, ListNode};

use super::{
    addr::{pa_to_kva, PhysAddr, VirtAddr},
    KSEG0,
};

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;

type PageIndex = usize;

#[repr(C)]
pub struct Page {
    p_link: Rc<RefCell<ListNode>>,
    p_pa: PhysAddr,
    p_ref: u16,
}

impl Page {
    pub fn new(idx: usize) -> Self {
        Self {
            p_link: Rc::new(RefCell::new(ListNode::new(idx))),
            p_pa: PhysAddr::new(0),
            p_ref: 0,
        }
    }
    #[allow(unused)]
    pub fn get_pa(&self) -> PhysAddr {
        self.p_pa
    }

    #[allow(unused)]
    pub fn get_no(&self) -> u32 {
        self.p_link.borrow().idx as u32
    }
}

lazy_static! {
    pub static ref PAGES: Spinlock<Vec<Page>> = Spinlock::new(Vec::new());
    static ref PAGE_LIST: Spinlock<DoubleLinkedList> = Spinlock::new(DoubleLinkedList::new());
}

extern "C" {
    pub static stack_end: usize;
}

pub(super) fn page_init(mem_sz: usize) {
    let count = mem_sz / PAGE_SIZE;
    let mut pages = PAGES.lock();
    let mut page_list = PAGE_LIST.lock();
    let kernel_end = unsafe { VirtAddr::new(&stack_end as *const usize as usize) };
    for i in 0..count {
        let mut page = Page::new(i);
        page.p_pa = PhysAddr::new(i * PAGE_SIZE);
        if KSEG0.add(i * PAGE_SIZE) >= kernel_end {
            page_list.push(page.p_link.clone());
        } else {
            page.p_ref = 1;
        }
        pages.push(page);
    }
}

#[inline(always)]
pub fn get_page_index_by_kvaddr(kvaddr: VirtAddr) -> Option<PageIndex> {
    kvaddr.sub(KSEG0).raw.checked_div(PAGE_SIZE)
}

pub fn page_alloc() -> Option<(PageIndex, PhysAddr)> {
    let mut page_list = PAGE_LIST.lock();
    let mut pages = PAGES.lock();
    let node = page_list.pop()?;
    let idx = node.borrow().idx;
    let page = &mut pages[idx];

    unsafe {
        ptr::write_bytes(pa_to_kva(page.p_pa).into(), 0, PAGE_SIZE);
    }
    page.p_ref = 1;
    Some((idx, page.p_pa))
}

#[allow(dead_code)]
pub fn page_incref(p: PageIndex) {
    let mut pages = PAGES.lock();
    pages[p].p_ref += 1;
}

pub fn page_decref(p: PageIndex) {
    let mut pages = PAGES.lock();
    pages[p].p_ref -= 1;
    if pages[p].p_ref == 0 {
        let mut page_list = PAGE_LIST.lock();
        page_list.push(pages[p].p_link.clone());
    }
}
