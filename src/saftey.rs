use core::panic::PanicInfo;
use crate::println;
// This prevents the kernal panicing

#[panic_handler]
extern "C" fn panic(info: &PanicInfo) -> ! {
        // Print the panic message
        println!("{}", info);
        loop {}
}

// This stops stack unwinding
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

