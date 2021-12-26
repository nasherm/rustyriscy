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

// --- A brief introduction to paging ---
// Paging allows for the OS to manage
// memory that is used by processes.
// We don't currently support processes
// but this will be a necessary step. Pages
// are a type of data structure used to divde
// our RAM (physical addresses). Our OS
// will allocate pages to processes, and manage
// which allows for the processes to view their
// address space as separate and their own. From the
// perspective of a process, all memmory is visible,
// This is known as virtual memory. In reality,
// this memory is managed and allocated by the OS.
// The page data structure will fundamentally
// be the unit of granualarity by which we bisect
// our physical address space. When we initialise
// paging, we read the size of our heap, and calculate
// the number of pages based on our page size (4096).
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

// We attempt to allocate
pub fn alloc(page: usize) -> *mut u8 {
    assert!(page > 0);
    unsafe {
        let num_pages = HEAP_SIZE / PAGE_SIZE;
        let heap_start = HEAP_START as *mut Page;
        heap_start as *mut u8
    }
}
