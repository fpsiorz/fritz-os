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
pub extern fn panic(_info: &PanicInfo) -> ! {
    //use core::fmt::Write;
    //print!("PANIC at {}:{}:{}: ", file, line, column);
    //vga_buffer::WRITER.lock().write_fmt(msg).unwrap();
    loop{}
}
