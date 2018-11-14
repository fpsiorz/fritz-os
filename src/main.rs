#![feature(lang_items)]
#![feature(const_fn)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
#[macro_use] extern crate lazy_static;
extern crate spin;
extern crate bootloader;


use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn _start() -> ! {
    println!("Fritz OS");
    println!("Schöne Grüße! ☺");
    loop {}
}

#[panic_handler]
pub extern fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}
