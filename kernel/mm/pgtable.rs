use super::addr::{PhysAddr, VirtAddr};
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

const PTE_HARDFLAG_SHIFT: usize = 0x6;

bitflags::bitflags! {
    struct Permssion: u32 {
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
    pub fn new() -> Self {
        PgtableEntry { raw_entry: 0 }
    }

    pub fn set(&mut self, ppn: usize, flags: usize) {
        self.raw_entry = (ppn << 10) | flags;
    }

    pub fn get(&self) -> usize {
        self.raw_entry
    }
}

impl Pgtable {
    pub fn new() -> Self {
        Pgtable {
            entries: [PgtableEntry::new(); 1024],
        }
    }

    pub fn get_entry(&self, index: usize) -> &PgtableEntry {
        &self.entries[index]
    }

    pub fn get_entry_mut(&mut self, index: usize) -> &mut PgtableEntry {
        &mut self.entries[index]
    }
    pub fn map_va_to_pa(
        &mut self,
        va: VirtAddr,
        pa: PhysAddr,
        count: usize,
        flags: Permssion,
        reset: bool,
    ) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn unmap_va(&mut self, va: VirtAddr) -> Result<(), &'static str> {
        let vpn = va.get_vpn();
        self.entries[vpn].set(0, 0);
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
