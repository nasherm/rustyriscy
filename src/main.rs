#![no_main]
#![no_std]
#![feature(naked_functions)]
#![feature(asm)]

use core::panic::PanicInfo;

// As we aren't linking the standard
// library we must define a panic
// handler. This is the function
// which is called when a  panic occurs
// i.e. a serious bug in our code.
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop {}
}

// This is our OS entry point
// placed at the magic address
// 0x80000000 by our linker. The below assembly
// performs a few necessary steps for OS.
// We first read the hart-id (hardware thread id)
// using a control register command. If this isn't
// equal to zero, we jump into a loop that can
// only be interrupted by a hardware interrupt.
// Otherwise, we write zero to the supervisor
// address and translation register. This register
// holds the physical page number of the root
// page table, an address space identifier
// which facilitates address-translation fences on
// a per-address space basis, and the mode field
// which selects the current address translation scheme
//              31    30      22 21          0
// satp -> [   MODE  ][  ASID  ] [     PPN    ]
//
// We have set all these values to zero. Setting
// MODE to zero means that we have no translation or protection.
//
// After setting the satp we perform three .option operations.
// This is to modify assembler options, in particular we want
// to set the norelax option which prevents linker relaxation.
// Linker relaxation is a way of optimizing programs at
// link time. More detail of linker relaxation can be found
// at this SiFIVE blog https://www.sifive.com/blog/all-aboard-part-3-linker-relaxation-in-riscv-toolchain
// Essentially by using .option push we specify that we
// want to push an option to the option stack, in this case
// relax. This option will then be maintained for the la
// instruction which loads the global pointer into gp.
// From there, we then set the stack pointer to 0x90000000.
// We then set the machine status register to some magic
// value 0x60000. This controls and keeps track of the
// hart's current operating state. We then load
// the address of 'kernel_init' into our machine
// exception program counter (mepc). We set
// our machine trap-handler base address (mtvec) to
// asm_trap_vector essentially saying that in
// the instance of a trap, loop. Then we
// set a write a magic value into mie which is
// our machine interrupt-enable register. We
// then set our return address to infinite loop
// at asm_trap_vector.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> !{
    asm!(
        "   csrr	t0, mhartid
            bnez	t0, 4f
            csrw	satp, zero
        .option push
        .option norelax
            la		gp, _global_pointer
        .option pop
            li		sp, 0x90000000
            li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
            csrw	mstatus, t0
            la		t1, kernel_init
            csrw	mepc, t1
            la		t2, asm_trap_vector
            csrw	mtvec, t2
            li		t3, (1 << 3) | (1 << 7) | (1 << 11)
            csrw	mie, t3
            la		ra, 4f
            mret
        4:
        asm_trap_vector:
            wfi
            j	    4b",
        options(noreturn)
    );
}

#[no_mangle]
unsafe fn kernel_init() -> ! {
    loop {}
}
