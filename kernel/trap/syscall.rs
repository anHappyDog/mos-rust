use super::trapframe::Trapframe;

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
    0
}

fn sys_print_cons(s: *const u8, num: u32) -> i32 {
    0
}

fn sys_write_dev(va: usize, pa: usize, len: usize) {}

fn sys_read_dev(va: usize, pa: usize, len: usize) -> i32 {
    0
}

fn sys_getenvid() -> i32 {
    0
}

fn sys_yield() -> ! {
    unreachable!("sad");
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

fn sys_panic(msg: *const u8) {}

fn sys_ipc_recv(dstva: usize) -> i32 {
    0
}

fn sys_ipc_try_send(envid: usize, val: usize, srcva: usize, perm: usize) -> i32 {
    0
}

fn sys_cgetc() -> i32 {
    0
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
