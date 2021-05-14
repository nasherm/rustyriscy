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
/* We specify in our memory section
   that we have ram that is writable,executable
   and allocatable.
*/

MEMORY
{
    ram (wxa) : ORIGIN = BASE_ADDRESS, LENGTH = 128M
}


PHDRS
{
    text PT_LOAD;
    data PT_LOAD;
    bss  PT_LOAD;
}

SECTIONS
{
    . = BASE_ADDRESS;

    /* CPU instructions */
    .text :
    {
        PROVIDE(_text_start = .);
        *(.startup) *(.text_init) *(.text *.text.*)
        PROVIDE(_text_end = .);
    /*
      Stephen Marz
      6 October 2019
      ---
      The portion after the right brace is in an odd format. However, this is telling the
      linker what memory portion to put it in. We labeled our RAM, ram, with the constraints
      that it is writeable, allocatable, and executable. The linker will make sure with this
      that we can do all of those things.

      >ram - This just tells the linker script to put this entire section (.text) into the
            ram region of memory. To my knowledge, the '>' does not mean "greater than". Instead,
        it is a symbol to let the linker know we want to put this in ram.

      AT>ram - This sets the LMA (load memory address) region to the same thing. LMA is the final
              translation of a VMA (virtual memory address). With this linker script, we're loading
          everything into its physical location. We'll let the kernel copy and sort out the
          virtual memory. That's why >ram and AT>ram are continually the same thing.

      :text  - This tells the linker script to put this into the :text program header. We've only
              defined three: text, data, and bss. In this case, we're telling the linker script
          to go into the text section.
      ---
    */
    } >ram AT>ram :text

    PROVIDE(_global_pointer = . );

    /* Global static data */
    .rodata :
    {
        PROVIDE(_rodata_start = .);
        *(.rodata .rodata.*)
        PROVIDE(_rodata_end = .);
    } >ram AT>ram :text


    /* Global initialized variables.
       We align to 4096 (0x1000) bytes to correspond with a
       a 4Kib paging resolution
    */
    .data ALIGN(0x1000):
    {
        PROVIDE(_data_start = .);
        *(.sdata .sdata.*) *(.data .data.*)
        PROVIDE(_data_end = .);
    } >ram AT>ram :data

    /* Global unitialized variables */
    .bss :
    {
        PROVIDE(_bss_start = .);
        *(.sbss .sbss.*) *(.bss .bss.*)
        PROVIDE(_bss_end = .);
    } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_stack = _bss_end + 0x80000);
  PROVIDE(_memory_end =   ORIGIN(ram) + LENGTH(ram));
  PROVIDE(_heap_start = _stack);
  PROVIDE(_heap_size = _memory_end - _stack);
}
