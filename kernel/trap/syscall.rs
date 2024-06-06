use super::trapframe::Trapframe;
use crate::dev::uart::{Uart, NS16550A};
use crate::mm::addr::{PhysAddr, VirtAddr};
use crate::mm::page::stack_end;
use crate::mm::pgtable::Permssion;
use crate::mm::UTOP;
use crate::mm::{KSEG1, UTEMP};
use crate::proc::sched::schedule;
use crate::proc::{get_idx_by_envid, EnvIndex, EnvStatus, CUR_ENV, ENV_LIST};
use crate::trap::E_INVAL;
use crate::{dev, print};
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
const MAX_SYSNO: usize = 18;

fn sys_putchar(c: u32) -> i32 {
    NS16550A.putchar(c);
    return 0;
}

fn sys_print_cons(s: VirtAddr, num: usize) -> i32 {
    for i in 0..num {
        NS16550A.putchar(s.add(i).read::<u8>().into());
    }
    return 0;
}

#[inline(always)]

fn sys_write_dev(va: VirtAddr, pa: PhysAddr, len: usize) -> i32 {
    if is_illegal_va_range(va, len) || is_illegal_dev_range(pa, len) || va % len != 0 {
        return -E_INVAL;
    }
    if len == 4 {
        pa.write_volatile::<u32>(va.read());
    } else if len == 2 {
        pa.write_volatile::<u16>(va.read());
    } else if len == 1 {
        pa.write_volatile::<u8>(va.read());
    } else {
        return -E_INVAL;
    }
    return 0;
}

fn sys_read_dev(va: VirtAddr, pa: PhysAddr, len: usize) -> i32 {
    if is_illegal_va_range(va, len) || is_illegal_dev_range(pa, len) || va % len != 0 {
        return -E_INVAL;
    }
    if len == 4 {
        va.write(pa.read_volatile::<u32>());
    } else if len == 2 {
        va.write(pa.read_volatile::<u16>());
    } else if len == 1 {
        va.write(pa.read_volatile::<u8>());
    } else {
        return -E_INVAL;
    }
    return 0;
}

fn sys_getenvid() -> i32 {
    let mut envid: usize = 0;
    let locked_cur_env_idx = CUR_ENV.lock();
    let locked_envs = ENV_LIST.lock();
    if let Some(env_idx) = *locked_cur_env_idx {
        envid = locked_envs[env_idx].get_envid();
    }
    return envid as i32;
}

fn sys_yield() -> ! {
    schedule(true);
    unreachable!("sys_yield");
}

fn sys_env_destroy(envid: usize) -> i32 {
    0
}

fn sys_set_tlb_mod_entry(envid: usize, func: usize) -> i32 {
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    envs[idx].env_user_tlb_mod_entry = func;
    return 0;
}

fn sys_mem_alloc(envid: usize, va: usize, perm: usize) -> i32 {
    0
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
            Some(pa) => pa,
            _ => return -E_INVAL,
        }
    };
    let dstenv = &mut envs[dstidx];
    let result = dstenv.env_pgdir.map_va_to_pa(dstva, pa, 1, &flags, false);
    if let Ok(_) = result {
        return 0;
    } else {
        return -E_INVAL;
    }
}

fn sys_mem_unmap(envid: EnvIndex, va: VirtAddr) -> i32 {
    if is_illegal_va(va) {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    let env = &mut envs[idx];
    if let Ok(_) = env.env_pgdir.unmap_va(va) {
        return 0;
    } else {
        return -E_INVAL;
    }
}

fn sys_exofork() -> i32 {
    0
}

fn sys_set_env_status(envid: usize, status: EnvStatus) -> i32 {
    if status != EnvStatus::EnvFree
        && status != EnvStatus::EnvRunnable
        && status != EnvStatus::EnvNotRunnable
    {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    envs[idx].env_status = status;
    return 0;
}

fn sys_set_trapframe(envid: usize, tf: VirtAddr) -> i32 {
    if is_illegal_va_range(tf, size_of::<Trapframe>()) {
        return -E_INVAL;
    }
    let idx = get_idx_by_envid(envid);
    let mut envs = ENV_LIST.lock();
    let curenv_idx = CUR_ENV.lock();
    if let Some(curidx) = *curenv_idx {
        if curidx != idx {
            envs[idx].env_tf = tf.read();
        } else {
            unsafe {
                VirtAddr::new(stack_end as usize - size_of::<Trapframe>())
                    .write(tf.read::<Trapframe>());
            }
        }
    } else {
        panic!("sys_set_trapframe: no curenv");
    }
    return 0;
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
    let mut envs = ENV_LIST.lock();
    let curenv_idx = CUR_ENV.lock();
    if let Some(curidx) = *curenv_idx {
        let curenv = &mut envs[curidx];
        curenv.env_ipc_dstva = dstva;
        curenv.env_ipc_recving = 1;
        curenv.env_status = EnvStatus::EnvNotRunnable;
        schedule(true);
    } else {
        panic!("sys_ipc_recv: no curenv");
    }
    return 0;
}

fn sys_ipc_try_send(envid: usize, val: usize, srcva: usize, perm: usize) -> i32 {
    0
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
    return va < UTEMP || va >= UTOP;
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
    return true;
}

fn is_illegal_va_range(va: VirtAddr, len: usize) -> bool {
    if len == 0 {
        return false;
    }
    return (va + len) < va || va < UTEMP || (va + len) > UTOP;
}

pub fn do_syscall(trapframe: &mut Trapframe) {
    trapframe.epc += 4;
    let sysno: usize = trapframe.regs[2];

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
            trapframe.get_arg1(),
            trapframe.get_arg2(),
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
        SYS_SET_TRAPFRAME => sys_set_trapframe(trapframe.get_arg0(), trapframe.get_arg1().into()),
        SYS_PANIC => sys_panic(trapframe.get_arg0().into()),
        SYS_IPC_RECV => sys_ipc_recv(trapframe.get_arg0().into()),
        SYS_IPC_TRY_SEND => sys_ipc_try_send(
            trapframe.get_arg0(),
            trapframe.get_arg1(),
            trapframe.get_arg2(),
            trapframe.get_arg3(),
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
