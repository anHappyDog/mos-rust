pub mod addr;
pub mod page;
pub mod pgtable;

use core::ops::Add;

use addr::VirtAddr;
use alloc_macro::define_simple_allocator;

use buddy::simple;

const KERNEL_HEAP_SIZE: usize = 4096 * 1024;

#[define_simple_allocator(KERNEL_HEAP_SIZE)]
static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[allow(unused)]
pub const KUSEG: VirtAddr = VirtAddr::new(0x0);
#[allow(unused)]
pub const KSEG0: VirtAddr = VirtAddr::new(0x80000000);
pub const KSEG1: VirtAddr = VirtAddr::new(0xa0000000);
#[allow(unused)]
pub const KSEG2: VirtAddr = VirtAddr::new(0xc0000000);

pub const PTMAP: usize = 4096;
pub const PDMAP: usize = 4 * 1024 * 1024;
pub const NASID: usize = 256;

// KSTACKTOP
pub const UVPT: VirtAddr = VirtAddr::new(0x7fc00000);
pub const UPAGES: VirtAddr = VirtAddr::new(0x7f800000);
pub const UENVS: VirtAddr = VirtAddr::new(0x7f400000);
pub const UTOP: VirtAddr = UENVS;
pub const UXSTACKTOP: VirtAddr = UTOP;
pub const USTACKTOP: VirtAddr = VirtAddr::new(0x7f3fd000);
pub const UTEXT: VirtAddr = VirtAddr::new(PDMAP);
pub const UCOW: VirtAddr = VirtAddr::new(0x3fe000);
pub const UTEMP: VirtAddr = VirtAddr::new(0x3fd000);

pub fn mem_init(mem_sz: usize) {
    page::page_init(mem_sz);
}
