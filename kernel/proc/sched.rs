// use super::{CUR_ENV,ENV_LIST};

use core::arch;
use core::mem::size_of;

use mips32::{cp0, gpr, Reg};

use super::{CUR_ENV, ENV_SCHED_LIST};
use crate::mm::addr::VirtAddr;
use crate::mm::page::stack_end;
use crate::println;
use crate::proc::EnvStatus;
use crate::proc::ENV_LIST;
use crate::trap;
use crate::trap::trapframe::Trapframe;

pub fn schedule(y: bool) {
    println!("\t\t\t\tschedule start");
    trap::int::disable_timer_interrupt();
    let curenv_id = CUR_ENV.lock();
    let idx = match *curenv_id {
        Some(idx) => {
            let mut envs = ENV_LIST.lock();
            unsafe {
                let env = &mut envs[idx];
                env.env_tf =
                    VirtAddr::from(&stack_end as *const usize as usize - size_of::<Trapframe>())
                        .read();
                println!("the saved env is {:#x},the epc is {}", idx, env.env_tf);
            }
            if envs[idx].env_cur_runs <= 0 || y || envs[idx].env_status != EnvStatus::Runnable {
                if envs[idx].env_status == EnvStatus::Runnable {
                    let mut env_sched_list = ENV_SCHED_LIST.lock();
                    env_sched_list.push(envs[idx].env_sched_link.clone());
                    envs[idx].env_cur_runs = envs[idx].env_pri as isize;
                }
                drop(envs);
                drop(curenv_id);
                {
                    let mut env_sched_list = ENV_SCHED_LIST.lock();
                    let node = env_sched_list.pop().expect("No env to run.");
                    let idx = node.borrow().idx;
                    CUR_ENV.lock().replace(idx);
                    idx
                }
            } else {
                idx
            }
        }
        None => {
            drop(curenv_id);
            {
                let mut env_sched_list = ENV_SCHED_LIST.lock();
                let node = env_sched_list.pop().expect("No env to run.");
                let idx = node.borrow().idx;
                CUR_ENV.lock().replace(idx);
                idx
            }
        }
    };
    {
        let mut env_list = ENV_LIST.lock();
        env_list[idx].env_cur_runs -= 1;
        cp0::entryhi::write(env_list[idx].env_asid);
        let stack = &env_list[idx].env_tf as *const Trapframe as usize;
        println!(
            "\t\t\t\t\tschedule end,curenv is {},epc is {:#x},env is {}",
            idx, env_list[idx].env_tf.epc,env_list[idx].env_tf
        );
        gpr::sp::write(stack);
    }
    trap::int::enable_timer_interrupt();

    unsafe {
        arch::asm!("j ret_from_exception");
    }
}
