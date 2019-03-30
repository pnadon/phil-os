
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), no_std)]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    panic!("Some panic message");
}

#[cfg(not(test))] // only compile when the test flag is not set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod vga_buffer;