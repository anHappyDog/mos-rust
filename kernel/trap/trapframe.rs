
#[repr(C)]
pub struct Trapframe {
    pub regs : [usize; 32],
    pub status : usize,
    pub hi : usize,
    pub lo : usize,
    pub badvaddr : usize,
    pub cause : usize,
    pub epc : usize,
}


