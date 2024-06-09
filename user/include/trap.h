#ifndef _TRAP_H_
#define _TRAP_H_

#ifndef __ASSEMBLER__

#include <types.h>

struct Trapframe
{
	/* Saved main processor registers. */
	unsigned long regs[32];

	/* Saved special registers. */
	unsigned long cp0_status;
	unsigned long hi;
	unsigned long lo;
	unsigned long cp0_badvaddr;
	unsigned long cp0_cause;
	unsigned long cp0_epc;
};

#endif /* !__ASSEMBLER__ */

#endif /* _TRAP_H_ */
