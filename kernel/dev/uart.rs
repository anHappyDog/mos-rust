use core::ptr;

pub const MALTA_SERIAL_BASE: usize = 0x18000000 + 0x3f8;
const MALTA_SERIAL_DATA: usize = 0x0;
const MALTA_SERIAL_LSR: usize = 0x5;
const MALTA_SERIAL_DATA_READY: u8 = 0x1;
const MALTA_SERIAL_THR_EMPTY: u8 = 0x20;

pub trait Uart {
    fn init(&mut self, base: usize, size: usize);
    fn putchar(&self, c: u32);
    fn getchar(&self) -> u32;
}

pub struct Ns16550a {
    base: usize,
    size: usize,
}

impl Ns16550a {
    pub const fn new(base: usize, size: usize) -> Self {
        Ns16550a { base, size }
    }
}

impl Uart for Ns16550a {
    fn init(&mut self, base: usize, size: usize) {
        self.base = base;
        self.size = size;
    }
    fn putchar(&self, c: u32) {
        unsafe {
            while ptr::read_volatile((self.base + MALTA_SERIAL_LSR) as *const u8)
                & MALTA_SERIAL_THR_EMPTY
                == 0
            {}
            ptr::write_volatile((self.base + MALTA_SERIAL_DATA) as *mut u8, c as u8);
        }
    }

    fn getchar(&self) -> u32 {
        unsafe {
            while ptr::read_volatile((self.base + MALTA_SERIAL_LSR) as *const u8)
                & MALTA_SERIAL_DATA_READY
                == 0
            {}
            ptr::read_volatile((self.base + MALTA_SERIAL_DATA) as *const u8) as u32
        }
    }
}
