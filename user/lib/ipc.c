// User-level IPC library routines

#include <env.h>
#include <lib.h>
#include <mmu.h>

// Send val to whom.  This function keeps trying until
// it succeeds.  It should panic() on any error other than
// -E_IPC_NOT_RECV.
//
// Hint: use syscall_yield() to be CPU-friendly.
void ipc_send(u_int whom, u_int val, const void *srcva, u_int perm) {
	int r;
	// debugf("ipc_send start,sender is %08x\n",env->env_id);
	// debugf("ipc_send: whom:%08x, val:%08x, srcva:%08x, perm:%08x\n", whom, val, srcva, perm);
	while ((r = syscall_ipc_try_send(whom, val, srcva, perm)) == -E_IPC_NOT_RECV) {
		syscall_yield();
	}
	// debugf("ipc_send end,return is %08x\n",r);
	user_assert(r == 0);
}

// Receive a value.  Return the value and store the caller's envid
// in *whom.
//
// Hint: use env to discover the value and who sent it.
u_int ipc_recv(u_int *whom, void *dstva, u_int *perm) {
	// debugf("start ipc_recv, whom: %08x\n",env->env_id);
	int r = syscall_ipc_recv(dstva);
	if (r != 0) {
		user_panic("syscall_ipc_recv err: %08x", r);
	}
	if (whom) {
		*whom = env->env_ipc_from;
	}

	if (perm) {
		*perm = env->env_ipc_perm;
	}
	// debugf("ipc_recv: whom:%08x, val:%08x, srcva:%08x, perm:%08x\n", env->env_ipc_from, env->env_ipc_value, env->env_ipc_value, env->env_ipc_perm);
	return env->env_ipc_value;
}
