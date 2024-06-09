use crate::{
    mm::{
        addr::VirtAddr,
        page::page_alloc,
        pgtable::{Permssion, PgtableEntry},
        KSEG0, PTMAP, UENVS, UPAGES, USTACKTOP, UTEMP, UTOP, UVPT, UXSTACKTOP,
    },
    println,
    proc::{Env, CUR_ENV, ENV_LIST},
    trap::trapframe,
};
use core::{mem::size_of, ops::Add};
use mips32::{cp0, Reg};

pub(super) fn do_tlb_mod(trapframe: &mut trapframe::Trapframe) {
    let tmp_tf = VirtAddr::from(trapframe as *const trapframe::Trapframe as usize)
        .read::<trapframe::Trapframe>();
    if trapframe.regs[29] < USTACKTOP.raw || trapframe.regs[29] >= UXSTACKTOP.raw {
        trapframe.regs[29] = UXSTACKTOP.raw;
    }
    trapframe.regs[29] -= size_of::<trapframe::Trapframe>();

    VirtAddr::from(trapframe.regs[29]).write(tmp_tf);
    let curenv = {
        match CUR_ENV.lock().as_mut() {
            Some(idx) => &ENV_LIST.lock()[*idx],
            None => panic!("do_tlb_mod: no env to run"),
        }
    };
    if curenv.env_user_tlb_mod_entry == 0 {
        panic!("TLB Mod but no user handler registered.");
    }
    // println!("tf is {},the param tf is {:#x}",tmp_tf,trapframe as *const trapframe::Trapframe as usize);
    trapframe.regs[4] = trapframe.regs[29];
    trapframe.regs[29] -= size_of::<usize>();
    trapframe.epc = curenv.env_user_tlb_mod_entry;
}

pub(super) fn do_tlbs(trapframe: &mut trapframe::Trapframe) {
    do_tlb_refill(trapframe);
}

pub(super) fn do_tlbl(trapframe: &mut trapframe::Trapframe) {
    do_tlb_refill(trapframe);
}

fn passive_alloc(env: &mut Env, va: VirtAddr) -> Result<(), &'static str> {
    if va < UTEMP {
        return Err("passive_alloc: va < UTEMP");
    }
    if va >= USTACKTOP && va < USTACKTOP.add(PTMAP) {
        return Err("passive_alloc: va >= USTACKTOP && va < UXSTACKTOP");
    }
    if va >= UENVS && va < UPAGES {
        return Err("passive_alloc: va >= UENVS && va < UPAGES");
    }
    if va >= UPAGES && va < UVPT {
        return Err("passive_alloc: va >= UPAGES && va < UVPT");
    }
    if va >= KSEG0 {
        return Err("passive_alloc: va >= KSEG0");
    }
    let (_, page_pa) = page_alloc().ok_or("No more pages")?;
    let mut perm = Permssion::empty();
    if va < UVPT {
        perm = Permssion::PTE_D;
    }
    env.env_pgdir
        .map_va_to_pa(va, page_pa, env.env_asid, 1, &perm, false)
}

fn do_tlb_refill(trapframe: &mut trapframe::Trapframe) {
    let badvaddr = trapframe.badvaddr;
    let asid = trapframe.hi & 0xff;
    mips32::tlb::tlb_invalidate(badvaddr, asid);
    let cur_env_idx = CUR_ENV.lock();
    let curenvidx = match *cur_env_idx {
        Some(idx) => idx,
        None => panic!("do_tlb_refill: no env to run"),
    };
    let curenv = &mut ENV_LIST.lock()[curenvidx];
    let ptentry: &PgtableEntry;
    loop {
        match curenv.env_pgdir.va_to_pa(VirtAddr::from(badvaddr)) {
            Some((p, _)) => {
                ptentry = p;
                break;
            }
            None => {
                passive_alloc(curenv, VirtAddr::from(badvaddr))
                    .expect("do_tlb_refill: passive_alloc failed");
            }
        };
    }
    let ppte = ptentry as *const PgtableEntry as usize & !0x7;
    let lo0 = VirtAddr::from(ppte).read::<usize>() >> 6;
    let lo1 = VirtAddr::from(ppte + size_of::<PgtableEntry>()).read::<usize>() >> 6;
    cp0::entrylo0::write(lo0);
    cp0::entrylo1::write(lo1);
    mips32::tlb::tlbwr();
}
