#![feature(lang_items)]
#![feature(const_fn)]

#![no_std]
#![no_main]

extern crate rlibc;
extern crate volatile;
#[macro_use] extern crate lazy_static;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn _start() -> ! {
    println!("Fritz OS");
    println!("Schöne Grüße!");
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    use core::fmt::Write;
    print!("PANIC at {}:{}:{}: ", file, line, column);
    vga_buffer::WRITER.lock().write_fmt(msg).unwrap();
    loop{}
}