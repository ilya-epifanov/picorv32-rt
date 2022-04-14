use core::arch::global_asm;

// This assembly is based on the original asm.S file which used to be pre
// compiled into binary blobs and shipped with the repo.

// Power on program entry point.
//
// This function is placed at offset zero and calls start.
global_asm!(
    ".section .initjmp, \"ax\"",
    ".global _initjmp",

    "_initjmp:",
    // Call _start
    "jal zero, _start"
);

// Initialisation entry point.
//
// It initializes DWARF call frame information, the stack pointer, the
// frame pointer (needed for closures to work in start_rust) and the global
// pointer. Then it calls _start_rust.
global_asm!(
    ".section .init, \"ax\"",
    ".global _start",

    "_start:",
    ".cfi_startproc",
    ".cfi_undefined ra",

    ".option push",
    ".option norelax",
    "la gp, __global_pointer$",
    "addi tp, gp, 0",
    ".option pop",

    "la sp, _stack_start",

    "add s0, sp, zero",

    "jal zero, _start_rust",

    ".cfi_endproc",
    );

// Trap entry point (_start_trap) when interrupt q registers are enabled.
//
// Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
// restores caller saved registers and then returns.
//
// Callee saved registers s0..11 are not stored and restored. Callee saved
// registers are saved and restored by the callee (_start_trap_rust will save
// and restore them if used).
#[cfg(feature = "interrupts-qregs")]
global_asm!(
    ".section .trap, \"ax\"",
    ".global _start_trap",

    "_start_trap:",

    // Create space on the stack
    "addi sp, sp, -16*4",

    // Store registers on the stack
    "sw gp,   0*4(sp)",
    "sw x5,   1*4(sp)",
    "sw x6,   2*4(sp)",
    "sw x7,   3*4(sp)",
    "sw x10,  4*4(sp)",
    "sw x11,  5*4(sp)",
    "sw x12,  6*4(sp)",
    "sw x13,  7*4(sp)",
    "sw x14,  8*4(sp)",
    "sw x15,  9*4(sp)",
    "sw x16, 10*4(sp)",
    "sw x17, 11*4(sp)",
    "sw x28, 12*4(sp)",
    "sw x29, 13*4(sp)",
    "sw x30, 14*4(sp)",
    "sw x31, 15*4(sp)",

    // Store the return address (x1) and stack pointer (x2) in the q
    // registers.
    //
    // NOTE: The `.insn` requires a register name for `rd` but the compiler is
    // not aware of the `q` registers as they are picorv32 specific. To work
    // around this we use the `x` register equivalents.
    ".insn r 0b0001011, 0, 0b0000001, x2, x1, zero", // setq q2, x1
    ".insn r 0b0001011, 0, 0b0000001, x3, x2, zero", // setq q3, x2

    // Store the pointer to the stored registers in a0
    "addi a0, sp, 0",

    // Store the IRQs to be handled bitmask in a1
    //
    // NOTE: The `.insn` requires a register name for `rs` but the compiler is
    // not aware of the `q` registers as they are picorv32 specific. To work
    // around this we use the `x` register equivalents.
    ".insn r 0b0001011, 0, 0b0000000, a1, x1, zero", // getq a1, q1

    // Call _start_trap_rust. This function takes a0 and a1 as arguments
    "jal ra, _start_trap_rust",

    // Restore the return address and stack pointer from the q registers
    //
    // NOTE: The `.insn` requires a register name for `rs` but the compiler is
    // not aware of the `q` registers as they are picorv32 specific. To work
    // around this we use the `x` register equivalents.
    ".insn r 0b0001011, 0, 0b0000000, x1, x2, zero", // getq x1, q2
    ".insn r 0b0001011, 0, 0b0000000, x2, x3, zero", // getq x2, q3

    // Restore the registers from the stack
    "lw gp,   0*4(sp)",
    "lw x5,   1*4(sp)",
    "lw x6,   2*4(sp)",
    "lw x7,   3*4(sp)",
    "lw x10,  4*4(sp)",
    "lw x11,  5*4(sp)",
    "lw x12,  6*4(sp)",
    "lw x13,  7*4(sp)",
    "lw x14,  8*4(sp)",
    "lw x15,  9*4(sp)",
    "lw x16, 10*4(sp)",
    "lw x17, 11*4(sp)",
    "lw x28, 12*4(sp)",
    "lw x29, 13*4(sp)",
    "lw x30, 14*4(sp)",
    "lw x31, 15*4(sp)",

    // Undo the stack allocation
    "addi sp, sp, 16*4",

    // Return to the pre intterupt location in the program
    ".insn r 0b0001011, 0, 0b0000010, zero, zero, zero" // retirq
    );

// Trap entry point (_start_trap) when interrupts are enabled but q registers
// are not enabled.
//
// Saves caller saved registers ra, t0..6, a0..7, calls _start_trap_rust,
// restores caller saved registers and then returns.
//
// Callee saved registers s0..11 are not stored and restored. Callee saved
// registers are saved and restored by the callee (_start_trap_rust will save
// and restore them if used).
#[cfg(all(feature = "interrupts", not(feature = "interrupts-qregs")))]
global_asm!(
    ".section .trap, \"ax\"",
    ".global _start_trap",

    "_start_trap:",

    // Create space on the stack
    "addi sp, sp, -18*4",

    // Store registers on the stack
    "sw gp,   0*4(sp)",
    "sw x1,   1*4(sp)",
    "sw x2,   2*4(sp)",
    "sw x5,   3*4(sp)",
    "sw x6,   4*4(sp)",
    "sw x7,   5*4(sp)",
    "sw x10,  6*4(sp)",
    "sw x11,  7*4(sp)",
    "sw x12,  8*4(sp)",
    "sw x13,  9*4(sp)",
    "sw x14, 10*4(sp)",
    "sw x15, 11*4(sp)",
    "sw x16, 12*4(sp)",
    "sw x17, 13*4(sp)",
    "sw x28, 14*4(sp)",
    "sw x29, 15*4(sp)",
    "sw x30, 16*4(sp)",
    "sw x31, 17*4(sp)",

    // Store the pointer to the stored registers in a0
    "addi a0, sp, 0",

    // Store the IRQs to be handled bitmask in a1. When q registers are not
    // enabled, IRQs to be handled bitmask is stored in x4 (tp)
    "addi a1, tp, 0",

    // Call _start_trap_rust. This function takes a0 and a1 as arguments
    "jal ra, _start_trap_rust",

    // Restore the registers from the stack
    "lw gp,   0*4(sp)",
    "lw x1,   1*4(sp)",
    "lw x2,   2*4(sp)",
    "lw x5,   3*4(sp)",
    "lw x6,   4*4(sp)",
    "lw x7,   5*4(sp)",
    "lw x10,  6*4(sp)",
    "lw x11,  7*4(sp)",
    "lw x12,  8*4(sp)",
    "lw x13,  9*4(sp)",
    "lw x14, 10*4(sp)",
    "lw x15, 11*4(sp)",
    "lw x16, 12*4(sp)",
    "lw x17, 13*4(sp)",
    "lw x28, 14*4(sp)",
    "lw x29, 15*4(sp)",
    "lw x30, 16*4(sp)",
    "lw x31, 17*4(sp)",

    // Undo the stack allocation
    "addi sp, sp, 18*4",

    // Return to the pre intterupt location in the program
    ".insn r 0b0001011, 0, 0b0000010, zero, zero, zero" // retirq
    );

// Trap entry point (_start_trap) when interrupts are not enabled.
//
// Calls _start.
#[cfg(not(any(feature = "interrupts", feature = "interrupts-qregs")))]
global_asm!(
    ".section .trap, \"ax\"",
    ".global _start_trap",

    "_start_trap:",
    // Call _start
    "jal zero, _start"
);

// Make sure there is an abort when linking
global_asm!(
    ".section .init",
    ".global abort",

    "abort:",
    // Call _start
    "jal zero, _start"
);
