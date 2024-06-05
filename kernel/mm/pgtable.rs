use core::ops::{Add, BitAnd};

use super::{
    addr::{PhysAddr, VirtAddr},
    page::{get_page_index_by_kvaddr, page_alloc, page_decref},
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

    pub fn to_addr(&self) -> PhysAddr {
        PhysAddr::new(self.raw_entry & 0xfffff000)
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
            let vpn = va.add(i << 12).get_vpn();
            if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
                let (_, pa) = page_alloc().ok_or("No more pages")?;
                self.entries[vpn >> 10].set(pa, &flags);
            }
            let pgd: &mut Pgtable =
                unsafe { &mut *(self.entries[vpn >> 10].to_addr().raw as *mut Pgtable) };
            if pgd.entries[vpn & 0x3ff] & Permssion::PTE_V {
                if reset {
                    let (_, pa) = page_alloc().ok_or("No more pages")?;
                    let pg = pgd.entries[vpn & 0x3ff].to_addr();
                    page_decref(
                        get_page_index_by_kvaddr(VirtAddr::new(pg.raw as usize))
                            .expect("The mapped memory should be a page."),
                    );
                    pgd.entries[vpn & 0x3ff].set(pa, &flags);
                } else {
                    return Err("Page already mapped");
                }
            }
            pgd.entries[vpn & 0x3ff].set(pa, &flags);
        }
        Ok(())
    }
    pub fn unmap_va(&mut self, va: VirtAddr) -> Result<(), &'static str> {
        let vpn = va.get_vpn();
        // self.entries[vpn].set(0, &Permssion::empty());
        if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
            return Err("the page is not mapped");
        }
        let pgd: &mut Pgtable =
            unsafe { &mut *(self.entries[vpn >> 10].to_addr().raw as *mut Pgtable) };
        if !(pgd.entries[vpn & 0x3ff] & Permssion::PTE_V) {
            return Err("the page is not mapped");
        }
        let pg = pgd.entries[vpn & 0x3ff].to_addr();
        page_decref(
            get_page_index_by_kvaddr(VirtAddr::new(pg.raw as usize))
                .expect("The mapped memory should be a page."),
        );
        pgd.entries[vpn & 0x3ff].set(PhysAddr::new(0), &Permssion::empty());
        Ok(())
    }

    pub fn va_to_pa(&self, va: VirtAddr) -> Option<PhysAddr> {
        let vpn = va.get_vpn();
        if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
            return None;
        }
        let pgd: &Pgtable = unsafe { &*(self.entries[vpn >> 10].to_addr().raw as *const Pgtable) };
        if !(pgd.entries[vpn & 0x3ff] & Permssion::PTE_V) {
            return None;
        }
        Some(pgd.entries[vpn & 0x3ff].to_addr())
    }
    pub fn pa_to_va(&self, pa: PhysAddr) -> Result<VirtAddr, &'static str> {
        for i in 0..1024 {
            let pgd: &Pgtable = unsafe { &*(self.entries[i].to_addr().raw as *const Pgtable) };
            for j in 0..1024 {
                if pgd.entries[j] & Permssion::PTE_V && pgd.entries[j].to_addr() == pa {
                    return Ok(VirtAddr::new((i << 22) | (j << 12)));
                }
            }
        }
        Err("The physical address is not mapped.")
    }
}
