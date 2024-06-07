pub mod sched;
use crate::mm::addr::kva_to_pa;
use crate::mm::addr::pa_to_kva;
use crate::mm::addr::PhysAddr;
use crate::mm::addr::VirtAddr;
use crate::mm::page::page_alloc;
use crate::mm::page::Page;
use crate::mm::page::PAGE_SIZE;
use crate::mm::pgtable::Permssion;
use crate::mm::pgtable::Pgtable;
use crate::mm::pgtable::PgtableEntry;
use crate::mm::KSEG0;
use crate::mm::UENVS;
use crate::mm::UPAGES;
use crate::mm::USTACKTOP;
use crate::mm::UTOP;
use crate::mm::UVPT;
use crate::println;
use crate::trap::trapframe;
use crate::util::DoubleLinkedList;
use crate::util::ListNode;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::mem::size_of;
use core::ops::Add;
use core::ptr;
use elf::ElfHeader;
use elf::ProgramHeader;
use lazy_static::lazy_static;
use mips32::cp0::ST_EXL;
use mips32::cp0::ST_IE;
use mips32::cp0::ST_IM7;
use mips32::cp0::ST_UM;
use sync::spin::Spinlock;

#[repr(C)]
pub struct Env {
    pub env_tf: trapframe::Trapframe,

    //padding for env_link 8B
    pub env_link: Rc<RefCell<ListNode>>,
    env_padding1: [u8; 4],
    //
    pub env_id: usize,
    pub env_asid: usize,
    env_parent_id: usize,
    pub env_status: EnvStatus,
    pub env_pgdir: Box<Pgtable>,

    // padding for env_sched_link 8B
    pub env_sched_link: Rc<RefCell<ListNode>>,
    pub env_cur_runs: isize,
    pub env_pri: usize,
    pub env_ipc_value: usize,
    pub env_ipc_from: usize,
    pub env_ipc_recving: usize,
    pub env_ipc_dstva: VirtAddr,
    pub env_ipc_perm: Permssion,
    pub env_user_tlb_mod_entry: usize,
    pub env_runs: usize,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum EnvStatus {
    Free = 0,
    Runnable = 1,
    NotRunnable = 2,
}

impl From<usize> for EnvStatus {
    fn from(value: usize) -> Self {
        match value {
            0 => EnvStatus::Free,
            1 => EnvStatus::Runnable,
            2 => EnvStatus::NotRunnable,
            _ => panic!("Invalid EnvStatus.\n"),
        }
    }
}

impl Env {
    pub fn init(&mut self, idx: usize) {
        self.env_tf = trapframe::Trapframe::new();

        self.env_id = 0;
        self.env_asid = 0;
        self.env_status = EnvStatus::Free;
        self.env_pgdir = Box::new(Pgtable::new());
        unsafe {
            ptr::write(
                &mut self.env_link,
                Rc::new(RefCell::new(ListNode::new(idx))),
            );
            ptr::write(
                &mut self.env_sched_link,
                Rc::new(RefCell::new(ListNode::new(idx))),
            );
        }
        self.env_pri = 0;
        self.env_ipc_value = 0;
        self.env_ipc_from = 0;
        self.env_ipc_recving = 0;
        self.env_ipc_dstva = VirtAddr::new(0);
        self.env_ipc_perm = Permssion::empty();
        self.env_user_tlb_mod_entry = 0;
        self.env_runs = 0;
        self.env_parent_id = 0;
        self.env_padding1 = [0; 4];
        self.env_cur_runs = 0;
    }
    pub fn create(&mut self, elf_data: &[u8]) {
        self.env_pri = DEFAULT_PRIO;
        let elf_ident = elf::ElfIdent::try_load(elf_data).unwrap();
        let elf_header = elf::load_elf_header::<elf::ElfHeader32>(elf_data, &elf_ident).unwrap();
        let elf_program_headers =
            elf::load_elf_program_headers::<elf::ProgramHeader32>(elf_data, &elf_header).unwrap();
        elf_program_headers.iter().for_each(|ph| {
            if ph.get_type() != elf::PT_LOAD {
                return;
            }
            let va = VirtAddr::new(ph.get_vaddr());
            let memsz = ph.get_memsz();
            let file_offset = ph.get_offset();
            let file_sz = ph.get_filesz();
            let mut perm = Permssion::empty();
            if ph.get_flags() & elf::PF_W == elf::PF_W {
                perm = Permssion::PTE_D;
            }
            for i in 0..memsz.div_ceil(PAGE_SIZE) {
                let (_, page_pa) = page_alloc().unwrap();
                self.env_pgdir
                    .as_mut()
                    .map_va_to_pa(
                        va.add(i * PAGE_SIZE),
                        page_pa,
                        self.env_asid,
                        1,
                        &perm,
                        false,
                    )
                    .unwrap();
                unsafe {
                    if i * PAGE_SIZE < file_sz {
                        core::ptr::copy(
                            elf_data.as_ptr().add(file_offset + i * PAGE_SIZE),
                            pa_to_kva(page_pa).into(),
                            core::cmp::min(PAGE_SIZE, file_sz - i * PAGE_SIZE),
                        );
                    }
                }
            }
        });
        self.env_id = mkenvid(self.env_link.borrow().idx);
        self.env_asid = asid_alloc().unwrap();
        self.env_status = EnvStatus::Runnable;
        self.env_tf.set_epc(elf_header.get_entry());
        self.env_tf.regs[29] = USTACKTOP.raw - size_of::<usize>() * 2;
        self.env_tf.set_status(ST_IM7 | ST_IE | ST_EXL | ST_UM);
    }
    #[allow(dead_code)]
    pub fn destroy(&mut self) {}
    pub fn get_envid(&self) -> usize {
        self.env_id
    }
}
pub const NASID: usize = 256;
pub const LOG2NENV: usize = 4;
pub const NENV: usize = 1 << LOG2NENV;
pub type EnvIndex = usize;

#[repr(C, align(4096))]
#[derive(Clone, Copy)]
struct PageBuffer {
    buffer: [u8; PAGE_SIZE],
}

fn init_envs() -> Vec<Env> {
    let mut t = Box::new(
        [PageBuffer {
            buffer: [0; PAGE_SIZE],
        }; (NENV * size_of::<Env>() + PAGE_SIZE - 1) / PAGE_SIZE],
    );
    assert!(t.as_ptr() as usize % PAGE_SIZE == 0);
    unsafe { Vec::from_raw_parts(t.as_mut_ptr() as usize as *mut Env, NENV, NENV) }
}

lazy_static! {
    pub static ref ENV_LIST: Spinlock<Vec<Env>> = Spinlock::new(init_envs());
    pub static ref CUR_ENV: Spinlock<Option<EnvIndex>> = Spinlock::new(None);
    pub static ref ENV_FREE_LIST: Spinlock<DoubleLinkedList> =
        Spinlock::new(DoubleLinkedList::new());
    pub static ref ENV_SCHED_LIST: Spinlock<DoubleLinkedList> =
        Spinlock::new(DoubleLinkedList::new());
    static ref ASID_BITMAP: Spinlock<Box<[u32; NASID / 32]>> =
        Spinlock::new(Box::new([0; NASID / 32]));
    static ref NEXT_ALLOC_ENV_ID: Spinlock<usize> = Spinlock::new(0);
    static ref PRE_PGTABLE: Spinlock<Box<Pgtable>> = Spinlock::new(Box::new(Pgtable::new()));
}

fn map_pre_pgdir(envs: *const Env) {
    let mut pre_pgtable = PRE_PGTABLE.lock();
    pre_pgtable
        .map_va_to_pa(
            UENVS,
            (envs as usize - KSEG0.raw).into(),
            0,
            (NENV * size_of::<Env>() + PAGE_SIZE - 1) / PAGE_SIZE,
            &Permssion::PTE_G,
            false,
        )
        .unwrap();
    let (pages_len, pages_vaddr) = {
        let pages = crate::mm::page::PAGES.lock();
        (pages.len(), pages.as_ptr() as usize)
    };
    println!(
        "envs's addr is {:x},pages' addr is {:x}",
        envs as usize, pages_vaddr
    );
    pre_pgtable
        .map_va_to_pa(
            UPAGES,
            (pages_vaddr - KSEG0.raw).into(),
            0,
            (pages_len * size_of::<Page>() + PAGE_SIZE - 1) / PAGE_SIZE,
            &Permssion::PTE_G,
            false,
        )
        .unwrap();
}

pub fn env_init() {
    let mut envs = ENV_LIST.lock();
    let mut env_free_list = ENV_FREE_LIST.lock();
    for i in 0..NENV {
        let env = &mut envs[i];
        env.init(i);
        env_free_list.push(env.env_link.clone());
    }

    map_pre_pgdir(envs.as_ptr());
}

#[inline(always)]
pub fn get_idx_by_envid(envid: usize) -> EnvIndex {
    if envid == 0 {
        let cur_env = CUR_ENV.lock();
        if let Some(idx) = *cur_env {
            return idx;
        } else {
            panic!("No current env.\n");
        }
    }
    envid & (NENV - 1)
}

fn mkenvid(idx: usize) -> usize {
    let mut locked_next_env_id = NEXT_ALLOC_ENV_ID.lock();
    let ret = *locked_next_env_id;
    *locked_next_env_id += 1;
    ret << (1 + LOG2NENV) | idx
}

fn asid_alloc() -> Result<usize, &'static str> {
    let mut locked_asid_bitmap = ASID_BITMAP.lock();
    for i in 1..NASID {
        let index = i >> 5;
        let inner = i & 31;
        if locked_asid_bitmap[index] & (1 << inner) == 0 {
            locked_asid_bitmap[index] |= 1 << inner;
            return Ok(i);
        }
    }
    Err("No more free asid.")
}

pub fn env_alloc(parent_id: Option<usize>) -> Result<EnvIndex, &'static str> {
    let node = ENV_FREE_LIST.lock().pop().expect("No more free env.");
    let idx = node.borrow().idx;
    let env = &mut ENV_LIST.lock()[idx];
    env.env_status = EnvStatus::Runnable;
    env.env_id = mkenvid(idx);
    env.env_asid = asid_alloc().unwrap();
    env.env_parent_id = parent_id.unwrap_or(0);
    env.env_runs = 0;
    let pre_table = PRE_PGTABLE.lock();
    for i in (UTOP.raw >> 22)..(UVPT.raw >> 22) {
        env.env_pgdir.entries[i] = pre_table.entries[i];
    }
    env.env_pgdir.entries[UVPT.raw >> 22].set(
        kva_to_pa(VirtAddr::from(env.env_pgdir.entries.as_ptr() as usize)),
        &Permssion::PTE_V,
    );
    Ok(idx)
}

pub const DEFAULT_PRIO: usize = 1;

pub fn env_create(elf_data: &[u8]) {
    let env_idx = env_alloc(None).unwrap();
    let mut envs = ENV_LIST.lock();
    let mut env_sched_list = ENV_SCHED_LIST.lock();
    envs[env_idx].create(elf_data);
    env_sched_list.push(envs[env_idx].env_sched_link.clone());
}
