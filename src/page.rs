// We will have page sizes of size 4kib
#![feature(asm)]
use crate::uart_println;

extern "C" {
    static HEAP_START: usize;
    static HEAP_SIZE: usize;
}

pub const PAGE_SIZE: usize = 4096;

#[repr(u8)]
enum Flags {
    Empty,
    Taken,
    Last,
}

impl Flags {
    pub fn val(self) -> u8 {
        self as u8
    }
}

pub struct Page {
    // Page descriptors
    flags: u8,
}

impl Page {
    pub fn clear(&mut self) {
        self.flags = Flags::Empty.val();
    }
}

pub fn init_paging() {
    unsafe {
        // uart_println!("HEAP_START = {:#08x}", HEAP_START);
        // uart_println!("HEAP_SIZE = {:#08x}", HEAP_SIZE);
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        // Initialise page flags
        let heap_ptr = HEAP_START as *mut Page;
        for i in 0..num_pages {
            (*heap_ptr.add(i)).clear();
        }
    }
}

// pub fn alloc(usize)
