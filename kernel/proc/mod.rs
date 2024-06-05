pub mod sched;

use crate::mm::addr::VirtAddr;
use crate::trap::trapframe;
use lazy_static::lazy_static;
use sync::spin::Spinlock;

pub const ENV_FREE: usize = 0;
pub const ENV_RUNNABLE: usize = 1;
pub const ENV_NOT_RUNNABLE: usize = 2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Env {
    env_tf: trapframe::Trapframe,

    //padding for env_link 8B
    env_idx: usize,

    //
    env_id: usize,
    env_asid: usize,
    env_status: usize,
    env_pgdir: VirtAddr,

    // padding for env_sched_link 8B
    env_pri: usize,
    env_ipc_value: usize,
    env_ipc_from: usize,
    env_ipc_recving: usize,
    env_ipc_dstva: usize,
    env_ipc_perm: usize,
    env_user_tlb_mod_entry: usize,
    env_runs: usize,
}

impl Env {
    pub fn create(&mut self, elf_data: &[u8]) {}
    pub fn destroy(&mut self) {}
    pub fn get_envid(&self) -> usize {
        self.env_id
    }
    pub const fn new() -> Env {
        Env {
            env_tf: trapframe::Trapframe::new(),
            env_id: 0,
            env_asid: 0,
            env_status: 0,
            env_pgdir: VirtAddr::new(0),
            env_pri: 0,
            env_ipc_value: 0,
            env_ipc_from: 0,
            env_ipc_recving: 0,
            env_ipc_dstva: 0,
            env_ipc_perm: 0,
            env_user_tlb_mod_entry: 0,
            env_runs: 0,
            env_idx: 0,
        }
    }
    pub fn run(&mut self) {}
}
pub const NASID: usize = 256;
pub const LOG2NENV: usize = 10;
pub const NENV: usize = 1 << LOG2NENV;
type EnvIndex = usize;

lazy_static! {
    pub static ref ENV_LIST: Spinlock<[Env; NENV]> = Spinlock::new([Env::new(); NENV]);
    pub static ref CUR_ENV: Spinlock<Option<EnvIndex>> = Spinlock::new(None);
    static ref ASID_BITMAP: Spinlock<[u32; NASID / 32]> = Spinlock::new([0; NASID / 32]);
    static ref NEXT_ENV_ID: Spinlock<usize> = Spinlock::new(0);
}

pub fn env_init() {}

#[inline(always)]
fn get_idx_by_envid(envid: usize) -> EnvIndex {
    return envid & (NENV - 1);
}

fn mkenvid(e: &Env) -> usize {
    let mut locked_next_env_id = NEXT_ENV_ID.lock();
    let ret = *locked_next_env_id;
    *locked_next_env_id += 1;
    return ret << (1 + LOG2NENV) | e.env_idx;
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

pub fn env_create(elf_data: &[u8]) -> usize {
    0
}
