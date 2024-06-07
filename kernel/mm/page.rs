use alloc::vec::Vec;
use core::{
    ops::{Add, Sub},
    ptr,
};
use lazy_static::lazy_static;
use sync::spin::Spinlock;

use crate::util::IndexStack;

use super::{addr::{PhysAddr, VirtAddr}, KSEG0};

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;

type PageIndex = usize;

#[repr(C)]
pub struct Page {
    p_no: u32,
    p_pa: PhysAddr,
    p_ref: u16,
}

impl Page {
    pub const fn new() -> Self {
        Self {
            p_no: 0,
            p_pa: PhysAddr::new(0),
            p_ref: 0,
        }
    }
    pub fn get_pa(&self) -> PhysAddr {
        self.p_pa
    }
    pub fn get_no(&self) -> u32 {
        self.p_no
    }
}

lazy_static! {
    pub static ref PAGES: Spinlock<Vec<Page>> = Spinlock::new(Vec::new());
    static ref PAGE_LIST: Spinlock<IndexStack> = Spinlock::new(IndexStack::new());
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
        let mut page = Page::new();
        page.p_no = i as u32;
        page.p_pa = PhysAddr::new(KSEG0.raw + i * PAGE_SIZE);
        if KSEG0.add(i * PAGE_SIZE) >= kernel_end {
            page_list.push(i);
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
    let pages = PAGES.lock();
    let pno = page_list.pop()?;
    Some((pno, pages[pno].p_pa))
}

pub fn page_incref(p: PageIndex) {
    let mut pages = PAGES.lock();
    pages[p].p_ref += 1;
}

pub fn page_decref(p: PageIndex) {
    let mut pages = PAGES.lock();
    pages[p].p_ref -= 1;
    if pages[p].p_ref == 0 {
        let mut page_list = PAGE_LIST.lock();
        page_list.push(p);
    }
}
