#[repr(C)]
pub struct Trapframe {
    pub regs: [usize; 32],
    pub status: usize,
    pub hi: usize,
    pub lo: usize,
    pub badvaddr: usize,
    pub cause: usize,
    pub epc: usize,
}

impl Trapframe {
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
        unsafe { *(self.regs[29] as *const usize).offset(4) }
    }
    #[inline(always)]
    pub fn get_arg4(&self) -> usize {
        unsafe { *(self.regs[29] as *const usize).offset(5) }
    }
    #[inline(always)]
    pub fn get_epc(&self) -> usize {
        self.epc
    }
    #[inline(always)]
    pub fn get_cause(&self) -> usize {
        self.cause
    }
    #[inline(always)]
    pub fn get_status(&self) -> usize {
        self.status
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
