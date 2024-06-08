use crate::{ proc::sched::schedule, trap::trapframe::Trapframe};
use mips32::{
    cp0::{sr, ST_IE, ST_IM7},
    Reg,
};

pub(super) fn do_interrupt(trapframe: &mut Trapframe) {
    let cause = trapframe.get_cause();
    if cause & ST_IM7 != 0 {
        schedule(false);
    } else {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } 
}

// pub fn enable_timer_interrupt() {
//     cp0::count::write(0);
//     cp0::compare::write(TIME_INTERVAL);
//     sr::write(sr::read() | ST_IM7);
// }

pub fn enable_interrupt() {
    sr::write(sr::read() | ST_IE);
}

// pub fn disable_timer_interrupt() {
//     sr::write(sr::read() & !ST_IM7);
// }
