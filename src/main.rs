#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate rlibc;

#[no_mangle]
pub extern fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &b) in b"Hello World".iter().enumerate() {
        unsafe{
            *vga_buffer.offset(i as isize * 2) = b;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
    loop{}
}