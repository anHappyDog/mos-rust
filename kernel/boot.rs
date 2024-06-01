#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(naked_functions)]
#![no_std]
#![no_main]

extern crate core;

mod dev;
mod mm;
mod proc;
mod trap;

use core::{arch, panic};

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _start() -> ! {
    unsafe {
        arch::asm!(
            "\tla $sp,stack_end\r\n
             \tjal   _init\r\n",
            options(noreturn)
        );
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _init(mem_sz: usize) -> ! {
    unreachable!("This sentence will never be printed.");
}

#[panic_handler]
fn panic(_info: &panic::PanicInfo) -> ! {
    unreachable!("This sentence will never be printed.");
}
