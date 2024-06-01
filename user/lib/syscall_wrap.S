#include <asm/asm.h>

/* ----- MOS EXERCISE 4 msyscall BEGIN ----- */
LEAF(msyscall)
	// Just use 'syscall' instruction and return.

	// ----- MOS BLANK BEGIN -----
	syscall
	jr      ra
	// ----- MOS BLANK END -----
END(msyscall)
/* ----- MOS EXERCISE END ----- */
