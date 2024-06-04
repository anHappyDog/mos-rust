use core::ops::{Add, BitAnd};

use super::{
    addr::{PhysAddr, VirtAddr},
    page::page_alloc,
    KSEG0,
};
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pgtable {
    entries: [PgtableEntry; 1024],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PgtableEntry {
    raw_entry: usize,
}

impl BitAnd<Permssion> for PgtableEntry {
    type Output = bool;
    fn bitand(self, rhs: Permssion) -> bool {
        self.raw_entry & (rhs.bits() as usize) != 0
    }
}

const PTE_HARDFLAG_SHIFT: usize = 0x6;

bitflags::bitflags! {
    pub struct Permssion: usize {
        const PTE_COW =  0x0001;
        const PTE_LIBRARY = 0x0002;
        const PTE_C_CACHEABLE = 0x0018 << PTE_HARDFLAG_SHIFT;
        const PTE_C_UNCACHEABLE = 0x0010 << PTE_HARDFLAG_SHIFT;
        const PTE_D = 0x0004 << PTE_HARDFLAG_SHIFT;
        const PTE_V = 0x0002 << PTE_HARDFLAG_SHIFT;
        const PTE_G = 0x0001 << PTE_HARDFLAG_SHIFT;
    }
}

impl PgtableEntry {
    pub const fn new() -> Self {
        PgtableEntry { raw_entry: 0 }
    }

    pub fn set(&mut self, ppn: PhysAddr, flags: &Permssion) {
        self.raw_entry = ppn | flags.bits();
    }

    pub fn get(&self) -> usize {
        self.raw_entry
    }
}

impl Pgtable {
    pub const fn new() -> Self {
        Pgtable {
            entries: [PgtableEntry::new(); 1024],
        }
    }
    pub fn map_va_to_pa(
        &mut self,
        va: VirtAddr,
        pa: PhysAddr,
        count: usize,
        flags: Permssion,
        reset: bool,
    ) -> Result<(), &'static str> {
        for i in 0..count {
            let vpn1 = va.add(i << 12).get_vpn() >> 12;
            if !(self.entries[vpn1] & Permssion::PTE_V) {
                let (pno, pa) = page_alloc().ok_or("No more pages")?;
                self.entries[vpn1].set(pa, &flags);
            }
        }
        Ok(())
    }
    pub fn unmap_va(&mut self, va: VirtAddr) -> Result<(), &'static str> {
        let vpn = va.get_vpn();
        // self.entries[vpn].set(0, &Permssion::empty());
        Ok(())
    }

    pub fn va_to_pa(&self, va: VirtAddr) -> Result<PhysAddr, &'static str> {
        let vpn = va.get_vpn();
        let ppn = self.entries[vpn].get() >> 10;
        if ppn == 0 {
            Err("Page not found")
        } else {
            Ok(PhysAddr::new(ppn << 12))
        }
    }
    pub fn pa_to_va(&self, pa: PhysAddr) -> Result<VirtAddr, &'static str> {
        let ppn = pa.get_ppn();
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.get() >> 10 == ppn {
                return Ok(VirtAddr::new(i << 12));
            }
        }
        Err("Page not found")
    }
}
