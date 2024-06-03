pub mod addr;
pub mod page;
pub mod pgtable;

use alloc_macro::define_simple_allocator;

use buddy::simple;

const KERNEL_HEAP_SIZE: usize = 4096 * 128;

#[define_simple_allocator(KERNEL_HEAP_SIZE)]
static mut KERNEL_HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[allow(unused)]
pub const KUSEG: usize = 0x00000000;
#[allow(unused)]
pub const KSEG0: usize = 0x80000000;
pub const KSEG1: usize = 0xa0000000;
#[allow(unused)]
pub const KSEG2: usize = 0xc0000000;

pub const PTMAP: usize = 4096;
pub const PDMAP: usize = 4 * 1024 * 1024;
pub const NASID: usize = 256;

pub const KSTACKTOP: usize = KSEG0 + PDMAP;
pub const UVPT: usize = KSEG0 - PDMAP;
pub const UPAGES: usize = UVPT - PDMAP;
pub const UENVS: usize = UPAGES - PDMAP;
pub const UTOP: usize = UENVS;
pub const UXSTACKTOP: usize = UTOP;
pub const USTACKTOP: usize = UTOP - 2 * PTMAP;
pub const UTEXT: usize = PDMAP;
pub const UCOW: usize = UTEXT - PTMAP;
pub const UTEMP: usize = UCOW - PTMAP;

pub fn mm_init(mem_sz: usize) {}
