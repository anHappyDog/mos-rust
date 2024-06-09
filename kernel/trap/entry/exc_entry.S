
.macro SAVE_ALL
.set noat
.set noreorder
	mfc0    $k0, $12
	andi    $k0, 0x0010
	beqz    $k0, 1f
	move    $k0, $sp
	la      $sp, stack_end
1:
	subu    $sp, $sp, 152
	sw      $k0, 116($sp)
	mfc0    $k0, $12
	sw      $k0, 128($sp)
	mfc0    $k0, $13
	sw      $k0, 144($sp)
	mfc0    $k0, $14
	sw      $k0, 148($sp)
	mfc0    $k0, $8
	sw      $k0, 140($sp)
	mfhi    $k0
	sw      $k0, 132($sp)
	mflo    $k0
	sw      $k0, 136($sp)
	sw      $0, 0($sp)
	sw      $1, 4($sp)
	sw      $2, 8($sp)
	sw      $3, 12($sp)
	sw      $4, 16($sp)
	sw      $5, 20($sp)
	sw      $6, 24($sp)
	sw      $7, 28($sp)
	sw      $8, 32($sp)
	sw      $9, 36($sp)
	sw      $10, 40($sp)
	sw      $11, 44($sp)
	sw      $12, 48($sp)
	sw      $13, 52($sp)
	sw      $14, 56($sp)
	sw      $15, 60($sp)
	sw      $16, 64($sp)
	sw      $17, 68($sp)
	sw      $18, 72($sp)
	sw      $19, 76($sp)
	sw      $20, 80($sp)
	sw      $21, 84($sp)
	sw      $22, 88($sp)
	sw      $23, 92($sp)
	sw      $24, 96($sp)
	sw      $25, 100($sp)
	sw      $26, 104($sp)
	sw      $27, 108($sp)
	sw      $28, 112($sp)
	sw      $30, 120($sp)
	sw      $31, 124($sp)
.set at
.set reorder
.endm


.macro RESTORE_ALL
.set noreorder
.set noat
	lw      $v0, 128($sp)
	mtc0    $v0, $12
	lw      $v1, 136($sp)
	mtlo    $v1
	lw      $v0, 132($sp)
	lw      $v1, 148($sp)
	mthi    $v0
	mtc0    $v1, $14
	lw      $31, 124($sp)
	lw      $30, 120($sp)
	lw      $28, 112($sp)
	lw      $25, 100($sp)
	lw      $24, 96($sp)
	lw      $23, 92($sp)
	lw      $22, 88($sp)
	lw      $21, 84($sp)
	lw      $20, 80($sp)
	lw      $19, 76($sp)
	lw      $18, 72($sp)
	lw      $17, 68($sp)
	lw      $16, 64($sp)
	lw      $15, 60($sp)
	lw      $14, 56($sp)
	lw      $13, 52($sp)
	lw      $12, 48($sp)
	lw      $11, 44($sp)
	lw      $10, 40($sp)
	lw      $9, 36($sp)
	lw      $8, 32($sp)
	lw      $7, 28($sp)
	lw      $6, 24($sp)
	lw      $5, 20($sp)
	lw      $4, 16($sp)
	lw      $3, 12($sp)
	lw      $2, 8($sp)
	lw      $1, 4($sp)
	lw      $sp, 116($sp) /* Deallocate stack */
.set at
.set reorder
.endm

.section .text.tlb_miss_entry
.globl tlb_miss_entry;
.align 2;
.type tlb_miss_entry,@function;
.ent tlb_miss_entry;
tlb_miss_entry:
	j       exc_gen_entry
.end tlb_miss_entry;

.section .text.exc_gen_entry
.globl exc_gen_entry;
.align 2;
.type exc_gen_entry,@function;
.ent exc_gen_entry;
exc_gen_entry:
	SAVE_ALL
	mfc0    $t0, $12
	and     $t0, $t0, ~(0x0010 | 0x0002 | 0x0001)
	mtc0    $t0, $12
    move    $a0,$sp
    jal   trap_handler
	j   ret_from_exception

.end exc_gen_entry;

.globl ret_from_exception;
.type ret_from_exception,@function;
ret_from_exception:
	RESTORE_ALL
	eret

.macro RESET_KCLOCK
	li 	$t0, 2000000
	mtc0	$zero, $9
	mtc0 	$t0, $11
.endm

.section .text,"ax",%progbits
.globl env_pop_tf;
.type env_pop_tf,@function;
.ent env_pop_tf;
env_pop_tf:
.set reorder
.set at
	mtc0    $a1, $10
	move    $sp, $a0
	RESET_KCLOCK
	j       ret_from_exception
.end env_pop_tf;