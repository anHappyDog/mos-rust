use core::{fmt::{Debug, Display}, ptr};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Trapframe {
    pub regs: [usize; 32],
    pub status: usize,
    pub hi: usize,
    pub lo: usize,
    pub badvaddr: usize,
    pub cause: usize,
    pub epc: usize,
}

impl Display for Trapframe {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Trapframe {{ regs: [")?;
        for i in 0..32 {
            write!(f, "{:#x}, ", self.regs[i])?;
        }
        write!(
            f,
            "], status: {:#x}, hi: {:#x}, lo: {:#x}, badvaddr: {:#x}, cause: {:#x}, epc: {:#x} }}",
            self.status, self.hi, self.lo, self.badvaddr, self.cause, self.epc
        )
    }
}

impl Trapframe {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            regs: [0; 32],
            status: 0,
            hi: 0,
            lo: 0,
            badvaddr: 0,
            cause: 0,
            epc: 0,
        }
    }
    #[inline(always)]
    pub fn get_arg0(&self) -> usize {
        self.regs[5]
    }
    #[inline(always)]
    pub fn get_arg1(&self) -> usize {
        self.regs[6]
    }
    #[inline(always)]
    pub fn get_arg2(&self) -> usize {
        self.regs[7]
    }
    #[inline(always)]
    pub fn get_arg3(&self) -> usize {
        unsafe { ptr::read_unaligned((self.regs[29] as *const usize).offset(4)) }
    }
    #[inline(always)]
    pub fn get_arg4(&self) -> usize {
        unsafe { ptr::read_unaligned((self.regs[29] as *const usize).offset(5)) }
    }
    #[inline(always)]
    pub fn get_cause(&self) -> usize {
        self.cause
    }
    #[inline(always)]
    pub fn set_status(&mut self, status: usize) {
        self.status = status;
    }
    #[inline(always)]
    pub fn set_epc(&mut self, epc: usize) {
        self.epc = epc;
    }
}
