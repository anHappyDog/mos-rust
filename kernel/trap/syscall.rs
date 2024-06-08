use super::trapframe::Trapframe;
use super::E_IPC_NOT_RECV;
use crate::dev::uart::{Uart, NS16550A};
use crate::mm::addr::{pa_to_kva, PhysAddr, VirtAddr};
use crate::mm::page::{get_page_index_by_kvaddr, page_alloc, page_incref, stack_end};
use crate::mm::pgtable::Permssion;
use crate::mm::UTOP;
use crate::mm::{KSEG1, UTEMP};
use crate::proc::sched::schedule;
use crate::proc::{
    get_idx_by_envid, Env, EnvIndex, EnvStatus, CUR_ENV, ENV_FREE_LIST, ENV_LIST, ENV_SCHED_LIST,
};
use crate::trap::E_INVAL;
use crate::{dev, print};
use crate::{println, proc};
use core::mem::size_of;
use core::ops::Add;

const SYS_PUTCHAR: usize = 0;
const SYS_PRINT_CONS: usize = 1;
const SYS_GETENVID: usize = 2;
const SYS_YIELD: usize = 3;
const SYS_ENV_DESTROY: usize = 4;
const SYS_SET_TLB_MOD_ENTRY: usize = 5;
const SYS_MEM_ALLOC: usize = 6;
const SYS_MEM_MAP: usize = 7;
const SYS_MEM_UNMAP: usize = 8;
const SYS_EXOFORK: usize = 9;
const SYS_SET_ENV_STATUS: usize = 10;
const SYS_SET_TRAPFRAME: usize = 11;
const SYS_PANIC: usize = 12;
const SYS_IPC_TRY_SEND: usize = 13;
const SYS_IPC_RECV: usize = 14;
const SYS_CGETC: usize = 15;
const SYS_WRITE_DEV: usize = 16;
const SYS_READ_DEV: usize = 17;

fn sys_putchar(c: u32) -> i32 {
    NS16550A.putchar(c);
    0
}

fn sys_print_cons(s: VirtAddr, num: usize) -> i32 {
    for i in 0..num {
        NS16550A.putchar(s.add(i).read::<u8>().into());
    }
    0
}

extern "C" fn sys_write_dev(va: VirtAddr, pa: PhysAddr, len: usize) -> i32 {
    if is_illegal_va_range(va, len) || is_illegal_dev_range(pa, len) || va % len != 0 {
        return -E_INVAL;
    }
    if len == 4 {
        pa.add(KSEG1.raw).write_volatile::<u32>(va.read::<u32>());
    } else if len == 2 {
        pa.add(KSEG1.raw).write_volatile::<u16>(va.read::<u16>());
    } else if len == 1 {
        pa.add(KSEG1.raw).write_volatile::<u8>(va.read::<u8>());
    } else {
        return -E_INVAL;
    }
    0
}

extern "C" fn sys_read_dev(va: VirtAddr, pa: PhysAddr, len: usize) -> i32 {
    if is_illegal_va_range(va, len) || is_illegal_dev_range(pa, len) || va % len != 0 {
        return -E_INVAL;
    }
    if len == 4 {
        va.write::<u32>(pa.add(KSEG1.raw).read_volatile::<u32>());
    } else if len == 2 {
        va.write::<u16>(pa.add(KSEG1.raw).read_volatile::<u16>());
    } else if len == 1 {
        va.write::<u8>(pa.add(KSEG1.raw).read_volatile::<u8>());
    } else {
        return -E_INVAL;
    }
    0
}

fn sys_getenvid() -> i32 {
    let mut envid: usize = 0;
    let locked_cur_env_idx = CUR_ENV.lock();
    let locked_envs = ENV_LIST.lock();
    if let Some(env_idx) = *locked_cur_env_idx {
        envid = locked_envs[env_idx].get_envid();
    }
    envid as i32
}

fn sys_yield() -> ! {
    schedule(true);
    unreachable!("sys_yield");
}

fn sys_env_destroy(envid: usize) -> i32 {
    let idx = get_idx_by_envid(envid);
    let mut env_list = ENV_LIST.lock();
    let env: &mut Env = &mut env_list[idx];
    env.env_status = EnvStatus::Free;
    ENV_SCHED_LIST.lock().remove(env.env_sched_link.clone());
    ENV_FREE_LIST.lock().push(env.env_link.clone());
    let mut curenv = CUR_ENV.lock();
    if let Some(curidx) = *curenv {
        if curidx == idx {
            curenv.take().unwrap();
            drop(curenv);
            drop(env_list);
            schedule(true);
        }
    }
    0
}

fn sys_set_tlb_mod_entry(envid: usize, func: usize) -> i32 {
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    envs[idx].env_user_tlb_mod_entry = func;
    0
}

fn sys_mem_alloc(envid: usize, va: VirtAddr, perm: Permssion) -> i32 {
    if is_illegal_va(va) {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    let env = &mut envs[idx];
    let (idx, page_pa) = page_alloc().unwrap();
    let result = env
        .env_pgdir
        .map_va_to_pa(va, page_pa, env.env_asid, 1, &perm, false);
    if result.is_ok() {
        0
    } else {
        -E_INVAL
    }
}

fn sys_mem_map(
    srcid: usize,
    srcva: VirtAddr,
    dstid: usize,
    dstva: VirtAddr,
    flags: Permssion,
) -> i32 {
    if is_illegal_va(srcva) || is_illegal_va(dstva) {
        return -E_INVAL;
    }
    let srcidx = get_idx_by_envid(srcid);
    let dstidx = get_idx_by_envid(dstid);
    let mut envs = ENV_LIST.lock();
    let pa = {
        let srcenv = &envs[srcidx];
        match srcenv.env_pgdir.va_to_pa(srcva) {
            Some((_, pa)) => pa,
            _ => return -E_INVAL,
        }
    };
    let dstenv = &mut envs[dstidx];
    let result = dstenv
        .env_pgdir
        .map_va_to_pa(dstva, pa, dstenv.env_asid, 1, &flags, true);
    let idx = get_page_index_by_kvaddr(pa_to_kva(pa)).unwrap();
    page_incref(idx);
    if result.is_ok() {
        0
    } else {
        -E_INVAL
    }
}

fn sys_mem_unmap(envid: EnvIndex, va: VirtAddr) -> i32 {
    if is_illegal_va(va) {
        return -E_INVAL;
    }

    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    let env = &mut envs[idx];
    if env.env_pgdir.unmap_va(va, env.env_asid).is_ok() {
        0
    } else {
        -E_INVAL
    }
}

fn sys_exofork() -> i32 {
    let mut locked_env_list = ENV_LIST.lock();
    let locked_curenv_idx = CUR_ENV.lock();
    let curenv_idx = if let Some(idx) = *locked_curenv_idx {
        idx
    } else {
        panic!("sys_exofork: no curenv");
    };
    let curenv = &locked_env_list[curenv_idx];
    let parent_env_id = curenv.env_id;
    let parent_pri = curenv.env_pri;
    let idx = proc::env_alloc(&mut locked_env_list, Some(parent_env_id), parent_pri)
        .expect("sys_exofork: env_alloc failed");
    let new_env = &mut locked_env_list[idx];
    unsafe {
        core::ptr::copy(
            (&stack_end as *const usize as usize - size_of::<Trapframe>()) as *const Trapframe,
            &mut new_env.env_tf as *mut Trapframe,
            1,
        );
    }
    new_env.env_tf.regs[2] = 0;
    new_env.env_status = EnvStatus::NotRunnable;
    new_env.env_id as i32
}

fn sys_set_env_status(envid: usize, status: EnvStatus) -> i32 {
    if status != EnvStatus::Free
        && status != EnvStatus::Runnable
        && status != EnvStatus::NotRunnable
    {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();

    if envs[idx].env_status != EnvStatus::Runnable && status == EnvStatus::Runnable {
        let mut env_sched_list = ENV_SCHED_LIST.lock();
        env_sched_list.push(envs[idx].env_sched_link.clone());
    } else if envs[idx].env_status == EnvStatus::Runnable && status != EnvStatus::Runnable {
        let mut env_sched_list = ENV_SCHED_LIST.lock();
        env_sched_list.remove(envs[idx].env_sched_link.clone());
    }
    envs[idx].env_status = status;
    0
}

fn sys_set_trapframe(envid: usize, tf: *const Trapframe) -> i32 {
    if is_illegal_va_range((tf as usize).into(), size_of::<Trapframe>()) {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    let curenv_idx = CUR_ENV.lock();
    unsafe {
        if let Some(curidx) = *curenv_idx {
            if curidx != idx {
                envs[idx].env_tf = *tf;
            } else {
                VirtAddr::from(&stack_end as *const usize as usize - size_of::<Trapframe>())
                    .write::<Trapframe>(*tf);
                return (*tf).regs[2] as i32;
            }
        } else {
            panic!("sys_set_trapframe: no curenv");
        }
    }

    0
}

fn sys_panic(msg: VirtAddr) -> ! {
    let mut ch: u32;
    let mut i = 0;
    print!("user panic: ");
    loop {
        ch = msg.add(i).read::<u8>().into();
        if ch == 0 {
            break;
        }
        NS16550A.putchar(ch);
        i += 1;
    }
    dev::halt();
}

fn sys_ipc_recv(dstva: VirtAddr) -> i32 {
    if dstva != VirtAddr::zero() && is_illegal_va(dstva) {
        return -E_INVAL;
    }

    let mut locked_env_list = ENV_LIST.lock();
    let locked_curenv_id = CUR_ENV.lock();
    if let Some(curidx) = *locked_curenv_id {
        let curenv = &mut locked_env_list[curidx];
        curenv.env_ipc_dstva = dstva;
        curenv.env_ipc_recving = 1;
        curenv.env_status = EnvStatus::NotRunnable;
        drop(locked_curenv_id);
        drop(locked_env_list);

        let tf = unsafe {
            ((&stack_end as *const usize as usize - size_of::<Trapframe>()) as *mut Trapframe)
                .as_mut()
                .unwrap()
        };
        tf.regs[2] = 0;
        schedule(true);
    } else {
        panic!("sys_ipc_recv: no curenv");
    }
    0
}

fn sys_ipc_try_send(envid: usize, val: usize, srcva: VirtAddr, perm: Permssion) -> i32 {
    if srcva != VirtAddr::zero() && is_illegal_va(srcva) {
        return -E_INVAL;
    }
    let mut locked_env_list = ENV_LIST.lock();
    let mut locked_env_sched_list = ENV_SCHED_LIST.lock();
    let locked_curenv_id = CUR_ENV.lock();

    let (curenv_id, pa) = if let Some(idx) = *locked_curenv_id {
        let pa = if srcva != VirtAddr::zero() {
            if let Some((_, page_pa)) = locked_env_list[idx].env_pgdir.va_to_pa(srcva) {
                page_pa
            } else {
                return -E_INVAL;
            }
        } else {
            PhysAddr::zero()
        };
        (locked_env_list[idx].env_id, pa)
    } else {
        panic!("sys_ipc_try_send: no curenv");
    };

    let idx = get_idx_by_envid(envid);
    let env: &mut Env = &mut locked_env_list[idx];
    if env.env_ipc_recving == 0 {
        return -E_IPC_NOT_RECV;
    }

    env.env_ipc_value = val;
    env.env_ipc_from = curenv_id;
    env.env_ipc_recving = 0;
    env.env_status = EnvStatus::Runnable;
    locked_env_sched_list.insert_to_tail(env.env_sched_link.clone());
    if srcva == VirtAddr::zero() {
        return 0;
    }
    env.env_ipc_perm = perm | Permssion::PTE_V;
    let result = env
        .env_pgdir
        .map_va_to_pa(env.env_ipc_dstva, pa, env.env_asid, 1, &perm, false);
    page_incref(get_page_index_by_kvaddr(pa_to_kva(pa)).expect("sys_ipc_try_send: page_incref"));
    if result.is_ok() {
        0
    } else {
        -E_INVAL
    }
}

fn sys_cgetc() -> i32 {
    let mut ch: u32;
    loop {
        ch = NS16550A.getchar();
        if ch != 0 {
            return ch as i32;
        }
    }
}

#[inline(always)]
fn is_illegal_va(va: VirtAddr) -> bool {
    va < UTEMP || va >= UTOP
}

const VALID_ADDR_SPACE_NUM: usize = 2;
const VALID_ADDR_START: [PhysAddr; VALID_ADDR_SPACE_NUM] =
    [PhysAddr::new(0x180003f8), PhysAddr::new(0x180001f0)];
const VALID_ADDR_END: [PhysAddr; VALID_ADDR_SPACE_NUM] =
    [PhysAddr::new(0x180003f8 + 0x20), PhysAddr::new(0x180001f8)];

fn is_illegal_dev_range(pa: PhysAddr, len: usize) -> bool {
    if (pa % 4 != 0 && len != 1 && len != 2) || (pa % 2 != 0 && len != 1) {
        return true;
    }
    let target_start: PhysAddr = pa;
    let target_end: PhysAddr = pa + len;
    for i in 0..VALID_ADDR_SPACE_NUM {
        if target_start >= VALID_ADDR_START[i] && target_end <= VALID_ADDR_END[i] {
            return false;
        }
    }
    true
}

#[inline(always)]
fn is_illegal_va_range(va: VirtAddr, len: usize) -> bool {
    if len == 0 {
        return false;
    }
    (va + len) < va || va < UTEMP || (va + len) > UTOP
}

pub fn do_syscall(trapframe: &mut Trapframe) {
    trapframe.epc += 4;
    let ret: i32 = match trapframe.regs[4] {
        SYS_CGETC => sys_cgetc(),
        SYS_PUTCHAR => sys_putchar(trapframe.get_arg0() as u32),
        SYS_PRINT_CONS => sys_print_cons(trapframe.get_arg0().into(), trapframe.get_arg1()),
        SYS_GETENVID => sys_getenvid(),
        SYS_YIELD => sys_yield(),
        SYS_ENV_DESTROY => sys_env_destroy(trapframe.get_arg0()),
        SYS_SET_TLB_MOD_ENTRY => sys_set_tlb_mod_entry(trapframe.get_arg0(), trapframe.get_arg1()),
        SYS_MEM_ALLOC => sys_mem_alloc(
            trapframe.get_arg0(),
            trapframe.get_arg1().into(),
            trapframe.get_arg2().into(),
        ),
        SYS_MEM_MAP => sys_mem_map(
            trapframe.get_arg0(),
            trapframe.get_arg1().into(),
            trapframe.get_arg2(),
            trapframe.get_arg3().into(),
            trapframe.get_arg4().into(),
        ),
        SYS_MEM_UNMAP => sys_mem_unmap(trapframe.get_arg0(), trapframe.get_arg1().into()),
        SYS_EXOFORK => sys_exofork(),
        SYS_SET_ENV_STATUS => sys_set_env_status(trapframe.get_arg0(), trapframe.get_arg1().into()),
        SYS_SET_TRAPFRAME => {
            sys_set_trapframe(trapframe.get_arg0(), trapframe.get_arg1() as *mut Trapframe)
        }
        SYS_PANIC => sys_panic(trapframe.get_arg0().into()),
        SYS_IPC_RECV => sys_ipc_recv(trapframe.get_arg0().into()),
        SYS_IPC_TRY_SEND => sys_ipc_try_send(
            trapframe.get_arg0(),
            trapframe.get_arg1(),
            trapframe.get_arg2().into(),
            trapframe.get_arg3().into(),
        ),
        SYS_WRITE_DEV => sys_write_dev(
            trapframe.get_arg0().into(),
            trapframe.get_arg1().into(),
            trapframe.get_arg2(),
        ),
        SYS_READ_DEV => sys_read_dev(
            trapframe.get_arg0().into(),
            trapframe.get_arg1().into(),
            trapframe.get_arg2(),
        ),
        _ => -E_INVAL,
    };
    trapframe.regs[2] = ret as usize;
}
