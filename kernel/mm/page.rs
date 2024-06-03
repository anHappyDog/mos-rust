use alloc::vec::Vec;
use core::ptr;
use lazy_static::lazy_static;
use once_cell::unsync::OnceCell;
use sync::spin::Spinlock;

use super::KSEG0;
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageList {
    prev: *mut Page,
    next: *mut Page,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Page {
    p_link: PageList,
    p_ref: u16,
}

impl Page {
    pub fn new() -> Self {
        Page {
            p_link: PageList {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            },
            p_ref: 0,
        }
    }
}



lazy_static! {
    static ref PAGE_LIST: Spinlock<PageList> = Spinlock::new(PageList {
        prev: ptr::null_mut(),
        next: ptr::null_mut(),
    });
    static ref PAGES: Spinlock<Vec<Page>> = Spinlock::new(Vec::new());
}

extern "C" {
    static stack_end: usize;
}

pub(super) fn page_init(mem_sz: usize) {
    let count = mem_sz / PAGE_SIZE;
    let mut pages = PAGES.lock();
    let mut page_list = PAGE_LIST.lock();
    let kernel_end = unsafe { &stack_end as *const usize as usize };
    for i in 0..count {
        let mut page = Page::new();
        pages.push(page);
        if (kernel_end <= i * PAGE_SIZE + KSEG0) {}
    }
}
