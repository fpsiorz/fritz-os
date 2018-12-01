#![no_std]
#![feature(abi_x86_interrupt)]

extern crate rlibc;
extern crate volatile;
#[macro_use] extern crate lazy_static;
extern crate bootloader;
extern crate uart_16550;
extern crate x86_64;
extern crate pic8259_simple;
extern crate interrupt_lock;
extern crate pc_keyboard;


#[cfg(test)] extern crate array_init;
#[cfg(test)] extern crate std;


#[macro_use]pub mod vga_buffer;
#[macro_use]pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod paging;

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}