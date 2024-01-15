#![no_std]
#![no_main]

mod console;
mod sbi;
mod mm;
use core::arch::global_asm;
use core::arch::asm;

#[no_mangle]
pub fn rust_main() {
    let i1 : i32 = 100;
    let i2 : i32 = 1000;
    let i3  = [1,2,33,3,3,3,3,3];
    for i in i3 {
        print!("{}\n",i);
    }
    unsafe {
        let sp : usize;
        asm!("mv {},sp",out(reg) sp);
        println!("sp is {:x}",sp);
    } 
    println!("{}",i1 + i2);
    print!("nihao\n");
    println!("asdasdasd");
    sbi::shutdown(false);
}
global_asm!(include_str!("entry.asm"));


