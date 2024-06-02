use core::ptr;

pub const PAGE_SHIFT : usize = 12;
pub const PAGE_SIZE : usize = 1 << PAGE_SHIFT;




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
