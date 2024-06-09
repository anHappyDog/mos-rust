pub mod uart;
use crate::mm::addr::VirtAddr;
const MALTA_FPGA_HALT: VirtAddr = VirtAddr::new(0xbf000000 + 0x500);

pub fn halt() -> ! {
    MALTA_FPGA_HALT.write_volatile(0x42);
    unreachable!("halt failed.\n");
}
