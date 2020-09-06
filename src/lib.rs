#![no_std]
#![feature(lang_items)]

// --------------------

mod saftey;
mod vga_buffer;
use core::fmt::Write;

// Entry point
#[no_mangle]
pub extern "C" fn kmain() -> ! {

        
        //vga_buffer::print_something();

        write!(vga_buffer::WRITER.lock(), "test").unwrap();
        write!(vga_buffer::WRITER.lock(), "H\n").unwrap();
        write!(vga_buffer::WRITER.lock(), "ello \n안녕하세요").unwrap();
        write!(vga_buffer::WRITER.lock(), "Wörld! NAOMI\n").unwrap();
        write!(vga_buffer::WRITER.lock(), "The numbers are {} \nand {}", 42, 1.0 / 3.0).unwrap();

        loop {}
}
