pub mod int;
pub mod syscall;
pub mod trapframe;

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

pub fn trap_init() {}

fn trap_handler(trapframe: &mut trapframe::Trapframe) {
    let cause = trapframe.cause;
    match cause {
        EXC_CODE_INT => int::do_interrupt(trapframe),
        EXC_CODE_MOD => {}
        EXC_CODE_TLBL => {}
        EXC_CODE_TLBS => {}
        EXC_CODE_ADEL => {}
        EXC_CODE_ADES => {}
        EXC_CODE_IBE => {}
        EXC_CODE_DBE => {}
        EXC_CODE_SYS => syscall::do_syscall(trapframe),
        EXC_CODE_BP => {}
        EXC_CODE_RI => {}
        EXC_CODE_CPU => {}
        EXC_CODE_OV => {}
        EXC_CODE_TRAP => {}
        EXC_CODE_FPE => {}
        EXC_CODE_C2E => {}
        _ => {}
    }
}
