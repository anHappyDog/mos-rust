use core::ops::{Add, BitAnd};

use mips32::tlb::tlb_invalidate;

use crate::println;

use super::{
    addr::{PhysAddr, VirtAddr},
    page::{get_page_index_by_kvaddr, page_alloc, page_decref, PAGE_SHIFT, PAGE_SIZE},
    KSEG0,
};
#[repr(C)]
#[derive(Clone, Copy)]
#[repr(align(4096))]
pub struct Pgtable {
    pub entries: [PgtableEntry; 1024],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PgtableEntry {
    pub raw_entry: usize,
}

impl BitAnd<Permssion> for PgtableEntry {
    type Output = bool;
    fn bitand(self, rhs: Permssion) -> bool {
        self.get() & rhs.bits() != 0
    }
}

const PTE_HARDFLAG_SHIFT: usize = 0x6;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
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
impl From<Permssion> for usize {
    fn from(value: Permssion) -> Self {
        value.bits()
    }
}

impl From<usize> for Permssion {
    fn from(value: usize) -> Self {
        Permssion::from_bits_truncate(value)
    }
}

impl PgtableEntry {
    pub const fn new() -> Self {
        PgtableEntry { raw_entry: 0 }
    }
    pub fn kva(&self) -> VirtAddr {
        VirtAddr::from((self.raw_entry & 0xfffff000) + KSEG0.raw)
    }

    pub fn set(&mut self, pa: PhysAddr, flags: &Permssion) {
        assert!(pa % PAGE_SIZE == 0);
        self.raw_entry = pa | flags.bits();
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
        asid: usize,
        count: usize,
        flags: &Permssion,
        reset: bool,
    ) -> Result<(), &'static str> {
        for i in 0..count {
            let tva = va.add(i << PAGE_SHIFT);
            let tpa = pa.add(i << PAGE_SHIFT);
            let vpn = tva.get_vpn();
            if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
                let (_, page_pa) = page_alloc().ok_or("No more pages")?;
                self.entries[vpn >> 10].set(
                    page_pa,
                    &(*flags | Permssion::PTE_V | Permssion::PTE_C_CACHEABLE),
                );
            }
            let pgd: &mut Pgtable =
                unsafe { &mut *(self.entries[vpn >> 10].kva().raw as *mut Pgtable) };
            if pgd.entries[vpn & 0x3ff] & Permssion::PTE_V {
                if reset {
                    page_decref(
                        get_page_index_by_kvaddr(pgd.entries[vpn & 0x3ff].kva())
                            .expect("The mapped memory should be a page."),
                    );
                    tlb_invalidate(tva.into(), asid);
                } else {
                    return Err("Page already mapped");
                }
            }
            pgd.entries[vpn & 0x3ff].set(
                tpa.align_down(PAGE_SIZE),
                &(*flags | Permssion::PTE_V | Permssion::PTE_C_CACHEABLE),
            );
            tlb_invalidate(tva.into(), asid);
            println!("mapped va {:#x} to pa {:#x}", tva.raw, tpa.raw);
        }
        Ok(())
    }
    pub fn unmap_va(&mut self, va: VirtAddr, asid: usize) -> Result<(), &'static str> {
        let vpn = va.get_vpn();
        // self.entries[vpn].set(0, &Permssion::empty());
        if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
            return Err("the page is not mapped");
        }
        let pgd: &mut Pgtable =
            unsafe { &mut *(self.entries[vpn >> 10].kva().raw as *mut Pgtable) };
        if !(pgd.entries[vpn & 0x3ff] & Permssion::PTE_V) {
            return Err("the page is not mapped");
        }
        let pg = pgd.entries[vpn & 0x3ff].kva();
        page_decref(get_page_index_by_kvaddr(pg).expect("The mapped memory should be a page."));
        pgd.entries[vpn & 0x3ff].set(PhysAddr::new(0), &Permssion::empty());
        tlb_invalidate(va.into(), asid);
        Ok(())
    }

    pub fn va_to_pa(&self, va: VirtAddr) -> Option<(&PgtableEntry, PhysAddr)> {
        let vpn = va.get_vpn();
        if !(self.entries[vpn >> 10] & Permssion::PTE_V) {
            return None;
        }
        let pgd: &Pgtable = unsafe { &*(self.entries[vpn >> 10].kva().raw as *const Pgtable) };
        if !(pgd.entries[vpn & 0x3ff] & Permssion::PTE_V) {
            return None;
        }
        Some((
            &pgd.entries[vpn & 0x3ff],
            PhysAddr::from(
                (pgd.entries[vpn & 0x3ff].raw_entry & !(PAGE_SIZE - 1))
                    | (va.raw & (PAGE_SIZE - 1)),
            ),
        ))
    }
}
