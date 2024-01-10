OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;
STACK_SIZE   = 4096 * 16;
SECTIONS {
    . = BASE_ADDRESS;
    .text : {
	*(.text.entry)
	*(.text .text.*)    
    }
    . = ALIGN(4K);
    .kernel_stack : {
	_kernel_stack_end = .;
	. = . + STACK_SIZE;
	_kernel_stack_top = .; 
    }
    . = ALIGN(4K);
    .rodata : {
	*(.rodata .rodata.*)
    }
    .srodata : {
	*(.srodata .srodata.*)
    } 
     .data : {
	*(.data .data.*)    
    }
    .sdata : {
	*(.sdata .sdata.*)
    }
    .bss : {
	*(.bss .bss.*)
    }
    .sbss : {
	*(.sbss .sbss.*)
    }
    .ebss = .;
    /DISCARD/ : {
	*(.comment)
	*(.eh_frame) 
    }
}

