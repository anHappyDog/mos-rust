use super::page::{PAGE_SHIFT, PAGE_SIZE};
use core::{
    ops::{Add, BitOr, Rem, Sub},
    ptr,
};
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct VirtAddr {
    pub raw: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PhysAddr {
    pub raw: usize,
}


impl Sub<VirtAddr> for VirtAddr {
    type Output = VirtAddr;

    fn sub(self, rhs: VirtAddr) -> Self::Output {
        VirtAddr{raw :self.raw - rhs.raw}
    }
}

impl BitOr<usize> for PhysAddr {
    type Output = usize;

    fn bitor(self, rhs: usize) -> Self::Output {
        self.raw | rhs
    }
}

impl BitOr<usize> for VirtAddr {
    type Output = usize;
    fn bitor(self, rhs: usize) -> Self::Output {
        self.raw | rhs
    }
}

impl Rem<usize> for PhysAddr {
    type Output = usize;

    fn rem(self, rhs: usize) -> Self::Output {
        self.raw % rhs
    }
}

impl Rem<usize> for VirtAddr {
    type Output = usize;
    fn rem(self, rhs: usize) -> Self::Output {
        self.raw % rhs
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        PhysAddr {
            raw: self
                .raw
                .checked_add(other)
                .expect("physaddr add overflowed."),
        }
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;
    fn add(self, other: usize) -> Self::Output {
        VirtAddr {
            raw: self
                .raw
                .checked_add(other)
                .expect("virtaddr add overflowed."),
        }
    }
}

impl VirtAddr {
    #[inline(always)]
    pub const fn new(raw: usize) -> Self {
        VirtAddr { raw }
    }

    #[inline(always)]
    pub fn get_vpn(&self) -> usize {
        self.raw >> PAGE_SHIFT
    }
    #[inline(always)]
    pub fn write<T>(&self, src: T) {
        unsafe {
            ptr::write(self.raw as *mut T, src);
        }
    }
    #[inline(always)]
    pub fn write_volatile<T>(&self, src: T) {
        unsafe {
            ptr::write_volatile(self.raw as *mut T, src);
        }
    }
    #[inline(always)]
    pub fn read<T>(&self) -> T {
        unsafe { ptr::read(self.raw as *const T) }
    }
    #[inline(always)]
    pub fn read_volatile<T>(&self) -> T {
        unsafe { ptr::read_volatile(self.raw as *const T) }
    }
}

impl PhysAddr {
    pub const fn new(raw: usize) -> Self {
        PhysAddr { raw }
    }

    pub fn get_ppn(&self) -> usize {
        self.raw >> PAGE_SHIFT
    }
    #[inline(always)]
    pub fn write<T>(&self, src: T) {
        unsafe {
            ptr::write(self.raw as *mut T, src);
        }
    }
    #[inline(always)]
    pub fn write_volatile<T>(&self, src: T) {
        unsafe {
            ptr::write_volatile(self.raw as *mut T, src);
        }
    }
    #[inline(always)]
    pub fn read<T>(&self) -> T {
        unsafe { ptr::read(self.raw as *const T) }
    }
    #[inline(always)]
    pub fn read_volatile<T>(&self) -> T {
        unsafe { ptr::read_volatile(self.raw as *const T) }
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
