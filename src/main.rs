#![no_std]
#![no_main]
#![feature(lang_items)]

#[no_mangle]
pub extern fn _start() -> ! {
    loop {}
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments, _file: &'static str, _line: u32, _column: u32) -> ! {
    loop{}
}