// use super::{CUR_ENV,ENV_LIST};

use core::arch;

use mips32::{cp0, gpr, Reg};

use super::{CUR_ENV, ENV_SCHED_LIST};
use crate::proc::EnvStatus;
use crate::proc::ENV_LIST;
use crate::trap::int::TIME_INTERVAL;
pub fn schedule(y: bool) {
    let curenv_id = CUR_ENV.lock();
    let idx = match *curenv_id {
        Some(idx) => {
            let mut envs = ENV_LIST.lock();
            if envs[idx].env_cur_runs <= 0 || y || envs[idx].env_status != EnvStatus::EnvRunnable {
                if envs[idx].env_status == EnvStatus::EnvRunnable {
                    let mut env_sched_list = ENV_SCHED_LIST.lock();
                    env_sched_list.push(envs[idx].env_sched_link.clone());
                    envs[idx].env_cur_runs = envs[idx].env_pri as isize;
                }
            }
            drop(curenv_id);
            let mut env_sched_list = ENV_SCHED_LIST.lock();
            if let Some(node) = env_sched_list.pop() {
                let idx = node.borrow().idx;
                *CUR_ENV.lock() = Some(idx);
                idx
            } else {
                panic!("No env to run.\n");
            }
        }
        None => {
            drop(curenv_id);
            let mut env_sched_list = ENV_SCHED_LIST.lock();
            if let Some(node) = env_sched_list.pop() {
                let idx = node.borrow().idx;
                *CUR_ENV.lock() = Some(idx);
                idx
            } else {
                panic!("No env to run.\n");
            }
        }
    };
    {
        let mut env_list = ENV_LIST.lock();
        env_list[idx].env_cur_runs -= 1;
        cp0::entryhi::write(env_list[idx].env_asid);
        gpr::sp::write(&env_list[idx].env_tf as *const _ as usize);
    }
    cp0::count::write(0);
    cp0::compare::write(TIME_INTERVAL);

    unsafe {
        arch::asm!("j ret_from_exception");
    }
}
