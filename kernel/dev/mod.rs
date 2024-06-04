pub mod ide;
pub mod uart;
use crate::mm::KSEG1;
use core::ptr;
const MALTA_FPGA_HALT: usize = 0x1f000000 + 0x500;

pub fn halt() -> ! {
    unsafe {
        ptr::write_volatile((KSEG1 | MALTA_FPGA_HALT) as *mut u8, 0x42);
        unreachable!("halt failed.\n");
    }
}
