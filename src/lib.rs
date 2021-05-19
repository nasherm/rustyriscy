#![no_std]
#![cfg_attr(test, no_main)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod uart;
pub mod page;

use core::panic::PanicInfo;

/// TESTING ///
use qemu_exit::QEMUExit;
const QEMU_EXIT_HANDLE: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x10_0000);

pub fn qemu_exit_fail() -> ! {
    QEMU_EXIT_HANDLE.exit_failure()
}

pub fn qemu_exit_success() -> ! {
    QEMU_EXIT_HANDLE.exit_success()
}

pub trait Testable {
    fn run(&self) -> ();
}

impl <T> Testable for T
where
    T : Fn()
{
    fn run(&self) {
        uart_print!("{} ...\t", core::any::type_name::<T>());
        self();
        uart_println!("[ok]")
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    uart_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    uart_println!();
    qemu_exit_success();
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    uart_println!("[failed]");
    uart_println!("Error: {}", info);
    qemu_exit_fail();
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    test_panic_handler(_info);
}

// This function is never called
// in reality, but acts as a hack
// for us to initialise read only
// global data.
#[link_section = ".rodata"]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn init_globals() -> ! {
    asm!(
        ".global HEAP_START
         HEAP_START: .dword _heap_start
         .global HEAP_SIZE
         HEAP_SIZE: .dword _heap_size",
        options(noreturn)
    );
}


#[cfg(test)]
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

#[cfg(test)]
#[no_mangle]
unsafe fn kernel_main() -> ! {
    test_main();
    loop {}
}
