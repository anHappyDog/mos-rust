pub mod int;
pub mod syscall;
pub mod tlb;
pub mod trapframe;
use core::{
    arch::{self, global_asm},
    usize,
};
use int::{enable_interrupt, enable_timer_interrupt, TIME_INTERVAL};

pub const E_UNSPECIFIED: i32 = 1;
pub const E_BAD_ENV: i32 = 2;
pub const E_INVAL: i32 = 3;
pub const E_NO_MEM: i32 = 4;
pub const E_NO_SYS: i32 = 5;
pub const E_NO_FREE_ENV: i32 = 6;
pub const E_IPC_NOT_RECV: i32 = 7;
pub const E_NO_DISK: i32 = 8;
pub const E_MAX_OPEN: i32 = 9;
pub const E_NOT_FOUND: i32 = 10;
pub const E_BAD_PATH: i32 = 11;
pub const E_FILE_EXISTS: i32 = 12;
pub const E_NOT_EXEC: i32 = 13;

const EXC_CODE_INT: usize = 0;
const EXC_CODE_MOD: usize = 1;
const EXC_CODE_TLBL: usize = 2;
const EXC_CODE_TLBS: usize = 3;
const EXC_CODE_ADEL: usize = 4;
const EXC_CODE_ADES: usize = 5;
const EXC_CODE_IBE: usize = 6;
const EXC_CODE_DBE: usize = 7;
const EXC_CODE_SYS: usize = 8;
const EXC_CODE_BP: usize = 9;
const EXC_CODE_RI: usize = 10;
const EXC_CODE_CPU: usize = 11;
const EXC_CODE_OV: usize = 12;
const EXC_CODE_TRAP: usize = 13;
const EXC_CODE_FPE: usize = 15;
const EXC_CODE_C2E: usize = 18;

#[no_mangle]
pub fn trap_init() {
    enable_interrupt();
    enable_timer_interrupt();
}

fn do_c2e(trapframe: &mut trapframe::Trapframe) {
    panic!("Coprocessor 2 exception: {:x}", trapframe.epc);
}

fn do_fpe(trapframe: &mut trapframe::Trapframe) {
    panic!("Floating point exception: {:x}", trapframe.epc);
}

fn do_trap(trapframe: &mut trapframe::Trapframe) {
    panic!("Trap exception: {:x}", trapframe.epc);
}

fn do_ov(trapframe: &mut trapframe::Trapframe) {
    panic!("Overflow exception: {:x}", trapframe.epc);
}

fn do_cpu(trapframe: &mut trapframe::Trapframe) {
    panic!("Coprocessor unusable exception: {:x}", trapframe.epc);
}

fn do_ri(trapframe: &mut trapframe::Trapframe) {
    panic!("Reserved instruction exception: {:x}", trapframe.epc);
}

fn do_bp(trapframe: &mut trapframe::Trapframe) {
    panic!("Breakpoint exception: {:x}", trapframe.epc);
}

fn do_dbe(trapframe: &mut trapframe::Trapframe) {
    panic!("Data bus error exception: {:x}", trapframe.badvaddr);
}

fn do_ibe(trapframe: &mut trapframe::Trapframe) {
    panic!("Instruction bus error exception: {:x}", trapframe.badvaddr);
}

fn do_ades(trapframe: &mut trapframe::Trapframe) {
    panic!("Address error exception: {:x}", trapframe.badvaddr);
}

fn do_adel(trapframe: &mut trapframe::Trapframe) {
    panic!("Address error exception: {:x}", trapframe.badvaddr);
}

fn do_unknown_exception(trapframe: &mut trapframe::Trapframe) {
    panic!("Unknown exception: {:x}", trapframe.cause);
}

extern "C" {
    fn exc_gen_entry();
    fn tlb_miss_entry();
}

#[no_mangle]
extern "C" fn trap_handler(trapframe: &mut trapframe::Trapframe) {
    let cause = (trapframe.cause >> 2) & 0x1f;
    match cause {
        EXC_CODE_INT => int::do_interrupt(trapframe),
        EXC_CODE_MOD => tlb::do_tlb_mod(trapframe),
        EXC_CODE_TLBL => tlb::do_tlbl(trapframe),
        EXC_CODE_TLBS => tlb::do_tlbs(trapframe),
        EXC_CODE_ADEL => do_adel(trapframe),
        EXC_CODE_ADES => do_ades(trapframe),
        EXC_CODE_IBE => do_ibe(trapframe),
        EXC_CODE_DBE => do_dbe(trapframe),
        EXC_CODE_SYS => syscall::do_syscall(trapframe),
        EXC_CODE_BP => do_bp(trapframe),
        EXC_CODE_RI => do_ri(trapframe),
        EXC_CODE_CPU => do_cpu(trapframe),
        EXC_CODE_OV => do_ov(trapframe),
        EXC_CODE_TRAP => do_trap(trapframe),
        EXC_CODE_FPE => do_fpe(trapframe),
        EXC_CODE_C2E => do_c2e(trapframe),
        _ => do_unknown_exception(trapframe),
    }
}

// acutally this function will never be called, just to avoid the compiler optimization
// fk rust optimization
pub fn ___avoid_fk_compiler_optimization() {
    //! just to call it to avoid the compiler optimization
    unsafe {
        trap_handler(&mut trapframe::Trapframe::new());
        exc_gen_entry();
        tlb_miss_entry();
    }
}
