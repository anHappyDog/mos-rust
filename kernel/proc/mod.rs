pub mod sched;

use crate::mm::addr::VirtAddr;
use crate::mm::pgtable::Pgtable;
use crate::trap::trapframe;
use crate::trap::trapframe::Trapframe;
use crate::util::DoubleLinkedList;
use crate::util::ListNode;
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cell::RefCell;
use core::mem::size_of;
use lazy_static::lazy_static;
use sync::spin::Spinlock;

#[repr(C)]
pub struct Env {
    pub env_tf: trapframe::Trapframe,

    //padding for env_link 8B
    env_link: Arc<RefCell<ListNode>>,
    env_padding1: [u8; 4],
    //
    env_id: usize,
    env_asid: usize,
    env_parent_id: usize,
    pub env_status: EnvStatus,
    pub env_pgdir: Box<Pgtable>,

    // padding for env_sched_link 8B
    env_sched_link: Arc<RefCell<ListNode>>,
    env_cur_runs: usize,
    env_pri: usize,
    env_ipc_value: usize,
    env_ipc_from: usize,
    env_ipc_recving: usize,
    env_ipc_dstva: usize,
    env_ipc_perm: usize,
    pub env_user_tlb_mod_entry: usize,
    env_runs: usize,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum EnvStatus {
    EnvFree = 0,
    EnvRunnable = 1,
    EnvNotRunnable = 2,
}

impl Env {
    pub fn new(idx: usize) -> Self {
        Self {
            env_tf: trapframe::Trapframe::new(),
            env_link: Arc::new(RefCell::new(ListNode::new(idx))),
            env_id: 0,
            env_asid: 0,
            env_status: EnvStatus::EnvFree,
            env_pgdir: Box::new(Pgtable::new()),
            env_sched_link: Arc::new(RefCell::new(ListNode::new(idx))),
            env_pri: 0,
            env_ipc_value: 0,
            env_ipc_from: 0,
            env_ipc_recving: 0,
            env_ipc_dstva: 0,
            env_ipc_perm: 0,
            env_user_tlb_mod_entry: 0,
            env_runs: 0,
            env_parent_id: 0,
            env_padding1: [0; 4],
            env_cur_runs: 0,
        }
    }
    pub fn create(&mut self, elf_data: &[u8]) {}
    pub fn destroy(&mut self) {}
    pub fn get_envid(&self) -> usize {
        self.env_id
    }
    pub fn run(&mut self) {}
}
pub const NASID: usize = 256;
pub const LOG2NENV: usize = 10;
pub const NENV: usize = 1 << LOG2NENV;
pub type EnvIndex = usize;

lazy_static! {
    pub static ref ENV_LIST: Spinlock<Vec<Env>> = Spinlock::new(Vec::new());
    pub static ref CUR_ENV: Spinlock<Option<EnvIndex>> = Spinlock::new(None);
    static ref ENV_FREE_LIST: Spinlock<DoubleLinkedList> = Spinlock::new(DoubleLinkedList::new());
    static ref ENV_SCHED_LIST: Spinlock<DoubleLinkedList> = Spinlock::new(DoubleLinkedList::new());
    static ref ASID_BITMAP: Spinlock<Box<[u32; NASID / 32]>> =
        Spinlock::new(Box::new([0; NASID / 32]));
    static ref NEXT_ALLOC_ENV_ID: Spinlock<usize> = Spinlock::new(0);
    static ref PRE_PGTABLE: Spinlock<Box<Pgtable>> = Spinlock::new(Box::new(Pgtable::new()));
}

pub fn env_init() {
    #[cfg(feature = "fit-cmos")]
    {
        assert!(LOG2NENV == 10);
        assert!(size_of::<Env>() == 0xdc);
        assert!(size_of::<Trapframe>() == 0x98);
    }
    let mut envs = ENV_LIST.lock();
    let mut env_free_list = ENV_FREE_LIST.lock();
    for i in 0..NENV {
        let mut env = Env::new(i);
        env.env_id = mkenvid(&env);
        env_free_list.push(env.env_link.clone());
        envs.push(env);
    }
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
    return envid & (NENV - 1);
}

fn mkenvid(e: &Env) -> usize {
    let envs = ENV_LIST.lock();
    let mut locked_next_env_id = NEXT_ALLOC_ENV_ID.lock();
    let ret = *locked_next_env_id;
    *locked_next_env_id += 1;
    let idx = envs
        .iter()
        .position(|x| x as *const Env == e as *const Env)
        .unwrap();
    return ret << (1 + LOG2NENV) | idx;
}

fn asid_alloc() -> Result<usize, &'static str> {
    let mut locked_asid_bitmap = ASID_BITMAP.lock();
    for i in 0..NASID {
        let mut index = i >> 5;
        let mut inner = i & 31;
        if locked_asid_bitmap[index] & (1 << inner) == 0 {
            locked_asid_bitmap[index] |= 1 << inner;
            return Ok(i);
        }
    }
    return Err("No more free asid.\n");
}

fn env_alloc() -> Result<EnvIndex, &'static str> {
    let mut env_free_list = ENV_FREE_LIST.lock();
    let mut envs = ENV_LIST.lock();
    if env_free_list.head.is_none() {
        return Err("No more free env.\n");
    }
    let node = env_free_list.pop().unwrap();
    let idx = node.borrow().idx;
    envs[idx].env_status = EnvStatus::EnvRunnable;
    return Ok(idx);
}

pub fn env_create(elf_data: &[u8]) {
    let env_idx = env_alloc().expect("Currently no more free env.");
    let mut envs = ENV_LIST.lock();
    envs[env_idx].create(elf_data);
}
