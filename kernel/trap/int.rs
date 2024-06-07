use crate::{println, proc::sched::schedule, trap::trapframe::Trapframe};
use mips32::{
    cp0::{self, sr, ST_IE, ST_IM0, ST_IM1, ST_IM2, ST_IM3, ST_IM4, ST_IM5, ST_IM6, ST_IM7},
    Reg,
};

pub const TIME_INTERVAL: usize = 500000;

pub(super) fn do_interrupt(trapframe: &mut Trapframe) {
    let cause = trapframe.get_cause();
    if cause & ST_IM7 != 0 {
        schedule(false);
    } else {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } 
}

pub fn enable_timer_interrupt() {
    cp0::count::write(0);
    cp0::compare::write(TIME_INTERVAL);
    sr::write(sr::read() | ST_IM7);
}

pub fn enable_interrupt() {
    sr::write(sr::read() | ST_IE);
}

pub fn disable_timer_interrupt() {
    sr::write(sr::read() & !ST_IM7);
}
