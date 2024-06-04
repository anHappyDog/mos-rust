use crate::{println, trap::trapframe::Trapframe};
use mips32::{
    cp0::{sr, ST_IE, ST_IM0, ST_IM1, ST_IM2, ST_IM3, ST_IM4, ST_IM5, ST_IM6, ST_IM7,self},
    Reg,
};

pub const TIME_INTERVAL: usize = 0x10000000;

pub(super) fn do_interrupt(trapframe: &mut Trapframe) {
    let cause = trapframe.cause;
    if cause & ST_IM7 != 0 {
        do_timer_int(trapframe);
    } else if cause & ST_IM6 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM5 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM4 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM3 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM2 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM1 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else if cause & ST_IM0 != 0 {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    } else {
        panic!("do_interrupt: unexpected interrupt, cause: {:x}", cause);
    }
}

fn do_timer_int(trapframe: &mut Trapframe) {
    println!("do_timer_int: count: {:x}, compare: {:x}", cp0::count::read(), cp0::compare::read());
    cp0::count::write(0);
    cp0::compare::write(TIME_INTERVAL);
    // panic!("do_timer_int: not implemented");
}

pub(super) fn enable_timer_interrupt() {
    sr::write(sr::read() | ST_IM7);
    cp0::count::write(0);
    cp0::compare::write(TIME_INTERVAL);
}

pub(super) fn enable_interrupt() {
    sr::write(sr::read() | ST_IE);
}

pub(super) fn disable_timer_interrupt() {}
