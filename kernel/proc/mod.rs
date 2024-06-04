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
    pub fn destroy() {}
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
        }
    }
    pub fn run(&mut self) {}
}

pub const ENV_COUNT: usize = 1024;

lazy_static! {
    pub static ref ENV_LIST: Spinlock<[Env; ENV_COUNT]> = Spinlock::new([Env::new(); ENV_COUNT]);
}

pub fn env_init() {}
