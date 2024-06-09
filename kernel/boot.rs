#![cfg_attr(target_arch = "mips", feature(asm_experimental_arch))]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(mem_copy_fn)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate core;

mod dev;
mod mm;
mod print;
mod proc;
mod trap;
mod util;
use crate::trap::___avoid_fk_compiler_optimization;
use core::arch;
use proc::sched;

elf::DEFINE_ELF_BYTES!(USER_ICODE, "../target/user/bin/icode.b");
elf::DEFINE_ELF_BYTES!(FS_SERV, "../target/user/bin/fs.b");
elf::DEFINE_ELF_BYTES!(TEST1, "../target/user/bin/test1.b");
elf::DEFINE_ELF_BYTES!(TEST2, "../target/user/bin/test2.b");
elf::DEFINE_ELF_BYTES!(TEST3, "../target/user/bin/test3.b");
elf::DEFINE_ELF_BYTES!(FKTEST, "../target/user/bin/fktest.b");
elf::DEFINE_ELF_BYTES!(TEST4, "../target/user/bin/test4.b");
elf::DEFINE_ELF_BYTES!(FSTEST, "../target/user/bin/fstest.b");
elf::DEFINE_ELF_BYTES!(TESTBSS, "../target/user/bin/testbss.b");
elf::DEFINE_ELF_BYTES!(PINGPING, "../target/user/bin/pingpong.b");
elf::DEFINE_ELF_BYTES!(TESTPIPE, "../target/user/bin/testpipe.b");
elf::DEFINE_ELF_BYTES!(TESTARG, "../target/user/bin/testarg.b");
elf::DEFINE_ELF_BYTES!(DEVTST, "../target/user/bin/devtst.b");
elf::DEFINE_ELF_BYTES!(NUM, "../target/user/bin/num.b");
elf::DEFINE_ELF_BYTES!(TESTFDSHARING, "../target/user/bin/testfdsharing.b");
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _init(mem_sz: usize) -> ! {
    logo();
    mm::mem_init(mem_sz);
    proc::env_init();
    // println!("creating user_icode");
    proc::env_create(USER_ICODE, 1);
    // proc::env_create(FSTEST, 1);
    // println!("creating fs_serv");
    // proc::env_create(TEST4, 1);
    // proc::env_create(PINGPING, 1);
    // proc::env_create(TEST2, 1);
    // proc::env_create(TESTARG, 1);
    // proc::env_create(DEVTST, 1);
    // proc::env_create(NUM, 1);
    // proc::env_create(TESTFDSHARING, 1);
    proc::env_create(FS_SERV, 1);
    // proc::env_create(TESTBSS, 1);
    sched::schedule(true);
    // never reach here,to let cheat the compiler
    // this function "will definitely be called"
    // so she won't optimize it inner functions.
    ___avoid_fk_compiler_optimization();
    dev::halt();
}

#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
extern "C" fn _start() -> ! {
    unsafe {
        arch::asm!(
            "\tmtc0 $0 ,    $12      \r\n
             \tla   $sp,    stack_end\r\n
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
