#![feature(lang_items)]
#![feature(const_fn)]

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate fritz_os;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern fn _start() -> ! {
    println!("Fritz OS");
    println!("Schöne Grüße! ☺");
    serial_println!("Auch seriell, schöne Grüße ♥");
    unsafe{ fritz_os::exit_qemu() };
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
pub extern fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

