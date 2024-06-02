use super::page::{PAGE_SHIFT, PAGE_SIZE};



pub struct VirtAddr {
    raw: usize,
}

pub struct PhysAddr {
    raw: usize,
}

impl VirtAddr {
    pub fn new(raw: usize) -> Self {
        VirtAddr { raw }
    }

    pub fn get_vpn(&self) -> usize {
        self.raw >> PAGE_SHIFT
    }
}

impl PhysAddr {
    pub fn new(raw: usize) -> Self {
        PhysAddr { raw }
    }

    pub fn get_ppn(&self) -> usize {
        self.raw >> PAGE_SHIFT
    }
}

impl From<usize> for VirtAddr {
    fn from(raw: usize) -> Self {
        VirtAddr::new(raw)
    }
}

impl From<usize> for PhysAddr {
    fn from(raw: usize) -> Self {
        PhysAddr::new(raw)
    }
}

impl From<VirtAddr> for usize {
    fn from(va: VirtAddr) -> Self {
        va.raw
    }
}

impl From<PhysAddr> for usize {
    fn from(pa: PhysAddr) -> Self {
        pa.raw
    }
}
