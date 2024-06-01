OUTPUT_ARCH(mips)
ENTRY(_start)

HIDDEN(KBASE = 0x80010000);
HIDDEN(HEAP_SIZE = 0x20000);
HIDDEN(STACK_SIZE = 0x4000);
SECTIONS {
	. = KBASE;
	.text : {
		*(.text.boot)
		*(.text .text.*)
	}
	. = ALIGN(4K);
	.data : {
		*(.data .data.*)
	}
	. = ALIGN(4K);
	.bss  : {
		*(.bss .bss.*)
	}
	. = ALIGN(4K);
	bss_end = .;
	.rodata : {
		*(.rodata .rodata.*)
	}
	. = ALIGN(4K);

	.stack (NOLOAD) :  {
		. += STACK_SIZE;
	}
	stack_end = .;
	/DISCARD/ : {
		*(.comment)
		*(.eh_frame)
		*(.eh_frame.*)
	}
}
