    .section .text.entry
    .globl _start
_start:
    la sp,_kernel_stack_top
    call rust_main
