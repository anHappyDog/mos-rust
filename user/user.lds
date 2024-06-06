OUTPUT_ARCH(mips)
ENTRY(_start)

SECTIONS {
	. = 0x00400000;

	.text ALIGN(4096) : {
		*(.text .text.*)
	}

	.rodata ALIGN(4096) : {
		*(.rodata .rodata.*)
	}

	.data ALIGN(4096) : {
		*(.data)
	}

	.bss ALIGN(4096) : {
		*(.bss)
	}

	end = . ;
}
