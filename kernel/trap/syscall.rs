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

fn sys_putchar(c: u32) {}

fn sys_print_cons() {}

fn sys_write_dev() {}

fn sys_read_dev() {}

fn sys_getenvid() {}

fn sys_yield() {}

fn sys_env_destroy() {}

fn sys_set_tlb_mod_entry() {}

fn sys_mem_alloc() {}

fn sys_mem_map() {}

fn sys_mem_unmap() {}

fn sys_exofork() {}

fn sys_set_env_status() {}

fn sys_set_trapframe() {}

fn sys_panic() {}

fn sys_ipc_recv() {}

fn sys_ipc_try_send() {}

fn sys_cgetc() -> u32 {
    0
}

pub fn do_syscall(trapframe: &mut Trapframe) {
    trapframe.epc += 4;
    let ret: u32 = match trapframe.regs[4] {
        SYS_CGETC => sys_cgetc(),
        _ => 0,
    };
    trapframe.regs[2] = ret as usize;
}
