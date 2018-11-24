#![feature(lang_items)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]

#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate fritz_os;
extern crate x86_64;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};


static BREAKPOINT_COUNT: AtomicUsize = AtomicUsize::new(0);
lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(_: &mut ExceptionStackFrame) {
    BREAKPOINT_COUNT.fetch_add(1, Ordering::SeqCst);
}

#[cfg(not(test))]
#[no_mangle]
pub extern fn _start() -> ! {
    TEST_IDT.load();
    x86_64::instructions::int3();
    x86_64::instructions::int3();
    x86_64::instructions::int3();

    match BREAKPOINT_COUNT.load(Ordering::SeqCst) {
        0 => {
            serial_println!("failed");
            serial_println!("breakpoint handler wasn't called");
        },
        3 => {
            serial_println!("ok");
        },
        count => {
            serial_println!("failed");
            serial_println!("breakpoint handler was called {} times", count);
        },
    }

    unsafe { fritz_os::exit_qemu() };
    panic!("failed to exit qemu")
}



#[cfg(not(test))]
#[panic_handler]
pub extern fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    unsafe { fritz_os::exit_qemu() };
    fritz_os::halt_loop();
}

