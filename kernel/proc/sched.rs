// use super::{CUR_ENV,ENV_LIST};

use core::mem::size_of;

use super::get_idx_by_envid;
use super::{CUR_ENV, ENV_SCHED_LIST};
use crate::mm::addr::VirtAddr;
use crate::mm::page::stack_end;
use crate::proc::EnvStatus;
use crate::proc::ENV_LIST;
use crate::trap::trapframe::Trapframe;

extern "C" {
    fn env_pop_tf(tf: &Trapframe, asid: usize);
}

pub fn schedule(y: bool) {

    let mut locked_curenv_idx = CUR_ENV.lock();
    let mut locked_env_list = ENV_LIST.lock();
    let mut locked_env_sched_list = ENV_SCHED_LIST.lock();
    let env_run_idx = if let Some(curenv_idx) = &mut *locked_curenv_idx {
        let curenv = &mut locked_env_list[*curenv_idx];
        unsafe {
            curenv.env_tf =
                VirtAddr::from(&stack_end as *const usize as usize - size_of::<Trapframe>())
                    .read::<Trapframe>();
        }
        if y || curenv.env_cur_runs <= 0 || curenv.env_status != EnvStatus::Runnable {
            if curenv.env_status == EnvStatus::Runnable {
                locked_env_sched_list.insert_to_tail(curenv.env_sched_link.clone());
                curenv.env_cur_runs = curenv.env_pri as isize;
            }
            let new_sched_env_idx = locked_env_sched_list
                .pop()
                .expect("No env to run.")
                .borrow()
                .idx;
            locked_curenv_idx.replace(new_sched_env_idx);
            get_idx_by_envid(locked_env_list[new_sched_env_idx].setup_for_run())
        } else {
            curenv.setup_for_run();
            *curenv_idx
        }
    } else {
        let new_sched_env_idx = locked_env_sched_list
            .pop()
            .expect("No env to run.")
            .borrow()
            .idx;
        locked_curenv_idx.replace(new_sched_env_idx);
        get_idx_by_envid(locked_env_list[new_sched_env_idx].setup_for_run())
    };
    let env_asid = locked_env_list[env_run_idx].env_asid;
    let env_tf = locked_env_list[env_run_idx].env_tf;
    // println!("[schedule]  env_run_idx: {}", env_run_idx);
    drop(locked_curenv_idx);
    drop(locked_env_list);
    drop(locked_env_sched_list);
    unsafe {
        env_pop_tf(&env_tf, env_asid);
    }
}
