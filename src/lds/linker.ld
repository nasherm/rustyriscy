OUTPUT_ARCH("riscv")

ENTRY(_start)
/* This is the typical base address
 * of kernels running on Risc-V. It's
 * the base address that will be used
 * by Qemu
 */
BASE_ADDRESS = 0x80000000;
/* PROVIDE(_stack_size = 4K); */
/* PROVIDE(_heap_size  = 16K); */

SECTIONS
{
    . = BASE_ADDRESS;

    .text :
    {
        *(.startup)
        *(.text.*)
    }

    .rodata ALIGN(0x1000):
    {
        PROVIDE(_global_pointer = .);
        *(.rodata*)
    }

    .data ALIGN(0x1000):
    {
        *(.sdata*)
        *(.data*)
    }

    .bss ALIGN(0x1000):
    {
        bss_start = .;
        *(.sbss*)
        *(.bss*)
        bss_end = .;
    }
}
