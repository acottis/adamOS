#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

// This prevents the kernal panicing
#[panic_handler]
extern "C" fn panic(_info: &PanicInfo) -> ! {
        loop {}
}

// This stops  g
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}


// --------------------

mod vga_buffer;


// Entry point
#[no_mangle]
pub extern "C" fn kmain() -> ! {

        vga_buffer::print_something();

        loop {}
}
