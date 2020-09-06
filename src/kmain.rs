#![no_std]
#![feature(lang_items)]

mod vga_buffer; // VGA Driver
mod saftey;     // Panic Kernel Panic Stuff

// Entry point
#[no_mangle]
pub extern "C" fn kmain() -> ! {

        println!("Hello world{}", "!");
        println!("Whats up");
        loop {}
}
