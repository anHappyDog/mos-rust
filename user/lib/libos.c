#include <env.h>
#include <lib.h>
#include <mmu.h>

void exit(void) {
	close_all();
	syscall_env_destroy(0);
	user_panic("unreachable code");
}

const volatile struct Env *env;
extern int main(int, char **);

void libmain(int argc, char **argv) {

	env = &envs[ENVX(syscall_getenvid())];
	main(argc, argv);
	exit();
}
