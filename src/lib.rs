#![feature(lang_items)]
#![no_std]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
extern fn eh_personality() {
}

#[panic_handler]
extern fn panic(_info: &PanicInfo) -> ! {
    loop{} 
}

#[no_mangle]
pub extern fn kmain() -> ! {
        unsafe {
                let vga = 0xb8000 as *mut u64;

                *vga = 0x2f592f412f4b2f4f;
        }
        loop { }
}
