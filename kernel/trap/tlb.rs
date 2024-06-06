use crate::trap::trapframe;

pub(super) fn do_tlb_mod(trapframe: &mut trapframe::Trapframe) {}

pub(super) fn do_tlbs(trapframe: &mut trapframe::Trapframe) {}

pub(super) fn do_tlbl(trapframe: &mut trapframe::Trapframe) {}
