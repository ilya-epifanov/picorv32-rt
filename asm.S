/*
    Entry point of all programs (_start).

    It initializes DWARF call frame information, the stack pointer, the
    frame pointer (needed for closures to work in start_rust) and the global
    pointer. Then it calls _start_rust.
*/

#include "custom_ops.S"

.section .initjmp, "ax"
    jal zero, _start


.section .init, "ax"
.global _start

_start:
    .cfi_startproc
    .cfi_undefined ra

    .option push
    .option norelax
    la gp, __global_pointer$
    addi tp, gp, 0
    .option pop

    la sp, _stack_start

    add s0, sp, zero

    jal zero, _start_rust

    .cfi_endproc


/*
    Trap entry point (_start_trap)

    Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
    restores caller saved registers and then returns.
*/
.section .trap, "ax"
.global _start_trap

_start_trap:

#if defined(RV32RT_INTERRUPTS) || defined(RV32RT_INTERRUPTS_QREGS)

#ifdef RV32RT_INTERRUPTS_QREGS

	picorv32_setq_insn(q2, x1)
	picorv32_setq_insn(q3, x2)

    addi sp, sp, -16*4

	sw gp,   0*4(sp)
	sw x5,   1*4(sp)
	sw x6,   2*4(sp)
	sw x7,   3*4(sp)
	sw x10,  4*4(sp)
	sw x11,  5*4(sp)
	sw x12,  6*4(sp)
	sw x13,  7*4(sp)
	sw x14,  8*4(sp)
	sw x15,  9*4(sp)
	sw x16, 10*4(sp)
	sw x17, 11*4(sp)
	sw x28, 12*4(sp)
	sw x29, 13*4(sp)
	sw x30, 14*4(sp)
    sw x31, 15*4(sp)

#else

    addi sp, sp, -18*4

	sw gp,   0*4(sp)
	sw x1,   1*4(sp)
	sw x2,   2*4(sp)
	sw x5,   3*4(sp)
	sw x6,   4*4(sp)
	sw x7,   5*4(sp)
	sw x10,  6*4(sp)
	sw x11,  7*4(sp)
	sw x12,  8*4(sp)
	sw x13,  9*4(sp)
	sw x14, 10*4(sp)
	sw x15, 11*4(sp)
	sw x16, 12*4(sp)
	sw x17, 13*4(sp)
	sw x28, 14*4(sp)
	sw x29, 15*4(sp)
	sw x30, 16*4(sp)
    sw x31, 17*4(sp)

#endif

	addi a0, sp, 0
#ifdef RV32RT_INTERRUPTS_QREGS
	picorv32_getq_insn(a1, q1)
#else
	addi a1, tp, 0
#endif

    jal ra, _start_trap_rust

#ifdef RV32RT_INTERRUPTS_QREGS

	picorv32_getq_insn(x1, q2)
	picorv32_getq_insn(x2, q3)

	lw gp,   0*4(sp)
	lw x5,   1*4(sp)
	lw x6,   2*4(sp)
	lw x7,   3*4(sp)
	lw x10,  4*4(sp)
	lw x11,  5*4(sp)
	lw x12,  6*4(sp)
	lw x13,  7*4(sp)
	lw x14,  8*4(sp)
	lw x15,  9*4(sp)
	lw x16, 10*4(sp)
	lw x17, 11*4(sp)
	lw x28, 12*4(sp)
	lw x29, 13*4(sp)
	lw x30, 14*4(sp)
    lw x31, 15*4(sp)

    addi sp, sp, 16*4

#else

	lw gp,   0*4(sp)
	lw x1,   1*4(sp)
	lw x2,   2*4(sp)
	lw x5,   3*4(sp)
	lw x6,   4*4(sp)
	lw x7,   5*4(sp)
	lw x10,  6*4(sp)
	lw x11,  7*4(sp)
	lw x12,  8*4(sp)
	lw x13,  9*4(sp)
	lw x14, 10*4(sp)
	lw x15, 11*4(sp)
	lw x16, 12*4(sp)
	lw x17, 13*4(sp)
	lw x28, 14*4(sp)
	lw x29, 15*4(sp)
	lw x30, 16*4(sp)
    lw x31, 17*4(sp)

    addi sp, sp, 18*4

#endif

    picorv32_retirq_insn()

#else

    jal zero, _start

#endif

/* Make sure there is an abort when linking */
.section .init
.global abort
abort:
    jal zero, _start
