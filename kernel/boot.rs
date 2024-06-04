#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

mod dev;
mod mm;
mod print;
mod proc;
mod trap;

use core::arch;

use proc::sched;

#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _init(mem_sz: usize) -> ! {
    logo();
    mm::mem_init(mem_sz);
    trap::trap_init();
    proc::env_init();
    sched::schedule(true);
}

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _start() -> ! {
    unsafe {
        arch::asm!(
            "\tla   $sp,    stack_end\r\n
             \tmove $a0,    $a3      \r\n
             \tj          _init      \r\n",
            options(noreturn)
        );
    }
}

#[inline(always)]
fn logo() {
    print!(" __  __    ____     _____\n");
    print!("|  \\/  |  / __ \\   / ____|\n");
    print!("| \\  / | | |  | | | (___\n");
    print!("| |\\/| | | |  | |  \\___ \\\n");
    print!("| |  | | | |__| |  ____) |\n");
    print!("|_|  |_|  \\____/  |_____/\n");
    print!("\n");
}
