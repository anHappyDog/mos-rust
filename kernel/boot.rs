#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(naked_functions)]
#![no_std]
#![no_main]

extern crate core;

use core::{arch, panic};

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _start() -> ! {
    unsafe {
        arch::asm!("\tnop\r\n
        \tjal _init", options(noreturn));
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _init(mem_sz: usize) -> ! {
    unreachable!("This sentence will never be printed.");
}

#[panic_handler]
fn pp(_info: &panic::PanicInfo) -> ! {
    unreachable!("This sentence will never be printed.");
}
