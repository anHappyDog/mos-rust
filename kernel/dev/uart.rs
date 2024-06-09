use core::ops::Add;

use lazy_static::lazy_static;

use crate::mm::{addr::VirtAddr, KSEG1};

pub const MALTA_SERIAL_BASE: usize = 0x18000000 + 0x3f8;
const MALTA_SERIAL_DATA: usize = 0x0;
const MALTA_SERIAL_LSR: usize = 0x5;
const MALTA_SERIAL_DATA_READY: u8 = 0x1;
const MALTA_SERIAL_THR_EMPTY: u8 = 0x20;

pub trait Uart {
    #[allow(unused)]
    fn init(&mut self, base: VirtAddr, size: usize);
    fn putchar(&self, c: u32);
    fn getchar(&self) -> u32;
}

pub struct Ns16550a {
    base: VirtAddr,
    size: usize,
}

impl Ns16550a {
    pub const fn new(base: VirtAddr, size: usize) -> Self {
        Ns16550a { base, size }
    }
}

impl Uart for Ns16550a {
    fn init(&mut self, base: VirtAddr, size: usize) {
        self.base = base;
        self.size = size;
    }
    fn putchar(&self, c: u32) {
        while self.base.add(MALTA_SERIAL_LSR).read_volatile::<u8>() & MALTA_SERIAL_THR_EMPTY == 0 {}
        self.base.add(MALTA_SERIAL_DATA).write_volatile(c as u8);
    }

    fn getchar(&self) -> u32 {
        while self.base.add(MALTA_SERIAL_LSR).read_volatile::<u8>() & MALTA_SERIAL_DATA_READY == 0 {
        }
        self.base.add(MALTA_SERIAL_DATA).read_volatile()
    }
}

lazy_static! {
    pub static ref NS16550A: Ns16550a = Ns16550a::new(KSEG1.add(MALTA_SERIAL_BASE), 0);
}
