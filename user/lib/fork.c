#include <env.h>
#include <lib.h>
#include <mmu.h>

static void __attribute__((noreturn)) cow_entry(struct Trapframe *tf) {
	u_int va = tf->cp0_badvaddr;
	u_int perm;


	perm = PTE_FLAGS(vpt[VPN(va)]);
	if ((perm & PTE_COW) == 0) {
		user_panic("PTE_COW not found, va=%08x, perm=%08x", va, perm);
	}

	perm = (perm & ~PTE_COW) | PTE_D;

	syscall_mem_alloc(0, (void *)UCOW, perm);

	memcpy((void *)UCOW, (void *)ROUNDDOWN(va, PAGE_SIZE), PAGE_SIZE);

	syscall_mem_map(0, (void *)UCOW, 0, (void *)va, perm);

	syscall_mem_unmap(0, (void *)UCOW);

	int r = syscall_set_trapframe(0, tf);
	user_panic("syscall_set_trapframe returned %d", r);
}

static void duppage(u_int envid, u_int vpn) {
	int r;
	u_int addr;
	u_int perm;

	addr = vpn << PGSHIFT;
	perm = vpt[vpn] & ((1 << PGSHIFT) - 1);

	perm &= ~PTE_SWAP;

	if ((perm & PTE_D) == 0 || (perm & PTE_LIBRARY) || (perm & PTE_COW)) {
		if ((r = syscall_mem_map(0, (void *)addr, envid, (void *)addr, perm)) < 0) {
			user_panic("user panic mem map error: %d", r);
		}
	} else {
		if ((r = syscall_mem_map(0, (void *)addr, envid, (void *)addr,
					 (perm & ~PTE_D) | PTE_COW)) < 0) {
			user_panic("user panic mem map error: %d", r);
		}
		if ((r = syscall_mem_map(0, (void *)addr, 0, (void *)addr,
					 (perm & ~PTE_D) | PTE_COW)) < 0) {
			user_panic("user panic mem map error: %d", r);
		}
	}

}

int fork(void) {
	u_int child;
	u_int i;

	if (env->env_user_tlb_mod_entry != (u_int)cow_entry) {
		try(syscall_set_tlb_mod_entry(0, cow_entry));
	}


	child = syscall_exofork();
	if (child == 0) {
		env = envs + ENVX(syscall_getenvid());
		return 0;
	}


	for (i = 0; i < PDX(UXSTACKTOP); i++) {
		if (vpd[i] & PTE_V) {
			for (u_int j = 0; j < PAGE_SIZE / sizeof(Pte); j++) {
				u_long va = (i * (PAGE_SIZE / sizeof(Pte)) + j) << PGSHIFT;
				if (va >= USTACKTOP) {
					break;
				}
				if (vpt[VPN(va)] & (PTE_V | PTE_SWAP)) {
					duppage(child, VPN(va));
				}
			}
		}
	}

	syscall_set_tlb_mod_entry(child, cow_entry);
	syscall_set_env_status(child, ENV_RUNNABLE);

	return child;
}
