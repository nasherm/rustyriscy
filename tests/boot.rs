#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rustyriscy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustyriscy::uart_println;

#[link_section = ".startup"]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> !{
    asm!(
        "   csrr	t0, mhartid
            bnez	t0, 3f
            csrw	satp, zero
        .option push
        .option norelax
            la		gp, _global_pointer
        .option pop
            la    a0, _bss_start
            la    a1, _bss_end
            bgeu  a0, a1, 2f
        1:
            sd        zero, (a0)
            addi     a0, a0, 8
            bltu     a0, a1, 1b
        2:
            la		sp, _stack_end
            li		t0, (0b11 << 11) | (1 << 7) | (1 << 3)
            csrw	mstatus, t0
            la		t1, kernel_main
            csrw	mepc, t1
            la		t2, asm_trap_vector
            csrw	mtvec, t2
            li		t3, (1 << 3) | (1 << 7) | (1 << 11)
            csrw	mie, t3
            la		ra, 4f
            mret
        3:
        4:
        asm_trap_vector:
            wfi
            j	    4b",
        options(noreturn)
    );
}

#[no_mangle]
unsafe fn kernel_main() -> ! {
    // Kernel landed
    test_main();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rustyriscy::test_panic_handler(_info);
}

#[test_case]
fn kernel_landed() {
    ()
}
