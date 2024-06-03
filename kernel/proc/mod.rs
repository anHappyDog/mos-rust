
use crate::trap::trapframe;
use crate::mm::pgtable;
use alloc::boxed::Box;

#[repr(C)]
struct Env {
    env_tf : trapframe::Trapframe,
    //padding for env_link
    
    env_id : usize,
    env_asid : usize,
    env_status : usize,
    env_pgdir : Box<pgtable::Pgtable>,
    // padding for env_sched_link

    env_pri : usize,
    env_ipc_value : usize,
    env_ipc_from : usize,
    env_ipc_recving : usize,
    env_ipc_dstva : usize,
    env_ipc_perm : usize,
    env_user_tlb_mod_entry : usize,
    env_runs : usize
}




impl Env {
    pub fn create(elf_data : &[u8]) {

    }
    pub fn destroy() {
        
    }
}