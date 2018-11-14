#![feature(lang_items)]
#![feature(const_fn)]

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate rlibc;
extern crate volatile;
#[macro_use] extern crate lazy_static;
extern crate spin;
extern crate bootloader;
extern crate array_init;
#[cfg(test)] extern crate std;
extern crate uart_16550;
extern crate x86_64;


use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;
#[macro_use]
mod serial;

#[cfg(not(test))]
#[no_mangle]
pub extern fn _start() -> ! {
    println!("Fritz OS");
    println!("Schöne Grüße! ☺");
    serial_println!("Auch seriell, schöne Grüße ♥");
    unsafe{ exit_qemu() };
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
pub extern fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
	
}
