OUTPUT_ARCH(mips)
ENTRY(_start)

HIDDEN(KBASE = 0x80020000);
HIDDEN(STACK_SIZE = 0x6000);
SECTIONS {
	. = 0x80000000;
	.tlb_miss_entry : {
		*(.text.tlb_miss_entry)
	}

	. = 0x80000180;
	.exc_gen_entry : {
		*(.text.exc_gen_entry)
	}
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
