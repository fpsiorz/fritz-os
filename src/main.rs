#![feature(lang_items)]
#![feature(const_fn)]

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate fritz_os;
extern crate bootloader;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[no_mangle]
pub extern fn _start(info: &bootloader::bootinfo::BootInfo) -> ! {
    println!("Fritz OS");
    println!("Schöne Grüße! ☺");
    serial_println!("Auch seriell, schöne Grüße ♥");
    fritz_os::gdt::init();
    fritz_os::interrupts::init_idt();

    println!("{:?}", info);
    x86_64::instructions::interrupts::enable();
    println!("No crash");
    fritz_os::paging::print_table();
    fritz_os::halt_loop();
}

#[cfg(not(test))]
#[panic_handler]
pub extern fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fritz_os::halt_loop();
}

