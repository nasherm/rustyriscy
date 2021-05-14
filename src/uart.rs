// To communicate with our device
// and print out to screen we're going
// to  use the UART (universal asynchronous reciever
// transmitter). We'll communicate with this device
// using memory mapped I/O. Memory mapped I/O allows
// us to treat a pointer to some page address as a
// proxy to communicate with our device i.e. we'll
// write to some address such as 0x10000000 which
// will map to the UART device and not the physical
// address. Our emulator Qemu emulates a NS165550A
// UART chipset.

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

#[repr(C)]
pub struct Uart {
    // buffer: &'static mut Buffer
    buffer_address: usize,
}

impl fmt::Write for Uart {
    fn write_str(&mut self, out:&str) -> Result<(), fmt::Error> {
        for c in out.bytes() {
            self.mmio_write(c);
        }
        Ok(())
    }
}

impl Uart {
    fn new() -> Self {
        Uart {
            buffer_address: 0x1000_0000,
        }
    }

    fn write_at_offset(&mut self, offset:usize, value:u8) {
        let mut ptr = self.buffer_address as *mut u8;
        unsafe {
            ptr.add(offset).write_volatile(value);
        }
    }

    fn read_at_offset(&mut self, offset:usize) -> u8 {
        let ptr = self.buffer_address as *mut u8;
        unsafe {
            ptr.add(offset).read_volatile()
        }
    }

    fn init(&mut self) {
        // We initialise the UART
        // by setting word length, enabling FIFOs
        // and enabling reciever interrupts
        // We wrap these operations in unsafe we are
        // manipulating raw pointers of which
        // Rust cannot perform checks of validity and safety
        // at compile time.
        // We first set word length. This describes
        // how many bits the transmitter and the reciever
        // buffers may contain. This is set at the line
        // control register (LCR) which has two slots.
        // If we set these both to zero, we have specifed
        // 8 bit characters.
        self.write_at_offset(3, 0b11);

        // We enable FIFO (first in/first out) which
        // allows for the storage of multiple bytes.
        // self.buffer.registers[2].write(0b1);
        self.write_at_offset(2, 0b1);

        // We enable reciever buffer interrupts. This means
        // that whenever data is added to the reciever, the
        // self.buffer.registers[1].write(0b1);
        self.write_at_offset(1, 0b1);
        // We don't set the BAUD rate as we're working
        // with an emulator.
    }

    pub fn mmio_write(&mut self, c: u8) {
        self.write_at_offset(0, c);
    }

    pub fn mmio_read(&mut self) -> Option<u8> {
        // We first check that there's data to read
        let dr_bit = self.read_at_offset(5);
        if dr_bit & 1 == 0 {
            None
        }
        else {
            Some(self.read_at_offset(0))
        }
    }
}

lazy_static! {
    pub static ref UART : Mutex<Uart> = Mutex::new( {
        let mut uart = Uart::new();
        uart.init();
        uart
    });
}

pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    UART.lock().write_fmt(args).expect("Printing failed");
}


#[macro_export]
macro_rules! uart_print
{
	  ($($arg:tt)*) => ({
        $crate::uart::_print(format_args!($($arg)*));
	  });
}
#[macro_export]
macro_rules! uart_println
{
	  () => ({
        $crate::uart_print!("\n");
	  });
	  ($fmt:expr) => ({
		    $crate::uart_print!(concat!($fmt, "\n"))
	  });
	  ($fmt:expr, $($args:tt)*) => ({
		    $crate::uart_print!(concat!($fmt, "\n"), $($args)*)
	  });
}

