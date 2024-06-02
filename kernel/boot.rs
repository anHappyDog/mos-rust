#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

extern crate core;
extern crate alloc;

mod dev;
mod mm;
mod print;
mod proc;
mod trap;

use core::arch;

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _start() -> ! {
    unsafe {
        arch::asm!(
            "\tla $sp,stack_end\r\n
             \tmove $a0,$a3\r\n
             \tjal   _init\r\n",
            options(noreturn)
        );
    }
}

#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _init(mem_sz: usize) -> ! {
    logo();
    mm::mm_init(mem_sz);
    dev::halt();
    unreachable!("This sentence will never be printed.");
}

fn logo() {
    print!(" __  __    ____     _____\n");
    print!("|  \\/  |  / __ \\   / ____|\n");
    print!("| \\  / | | |  | | | (___\n");
    print!("| |\\/| | | |  | |  \\___ \\\n");
    print!("| |  | | | |__| |  ____) |\n");
    print!("|_|  |_|  \\____/  |_____/\n");
    print!("\n");
}
