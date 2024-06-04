use core::ops::Add;
use core::ptr::read_volatile;

use super::trapframe::Trapframe;
use crate::dev::uart::{Uart, NS16550A};
use crate::mm::addr::{PhysAddr, VirtAddr};
use crate::mm::UTOP;
use crate::mm::{KSEG1, UTEMP};
use crate::proc::sched::schedule;
use crate::trap::E_INVAL;

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
    0
}

fn sys_yield() -> ! {
    schedule(true);
    unreachable!("sys_yield");
}

fn sys_env_destroy(envid: usize) -> i32 {
    0
}

fn sys_set_tlb_mod_entry(envid: usize, func: usize) -> i32 {
    0
}

fn sys_mem_alloc(envid: usize, va: usize, perm: usize) -> i32 {
    0
}

fn sys_mem_map(envid: usize) -> i32 {
    0
}

fn sys_mem_unmap(envid: usize, va: usize) -> i32 {
    0
}

fn sys_exofork() -> i32 {
    0
}

fn sys_set_env_status(envid: usize, status: usize) -> i32 {
    0
}

fn sys_set_trapframe(envid: usize, tf: &mut Trapframe) -> i32 {
    0
}

fn sys_panic(msg: *const u8) {
    panic!("{}", 1);
}

fn sys_ipc_recv(dstva: usize) -> i32 {
    0
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
        _ => 0,
    };
    trapframe.regs[2] = ret as usize;
}
