use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use gdt;
use pic8259_simple::ChainedPics;
use interrupt_lock::Mutex;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = 
    Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET)});

const TIMER_INTERRUPT_ID: u8 = PIC1_OFFSET + 0;
const KEYBOARD_INTERRUPT_ID: u8 = PIC1_OFFSET + 1;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[TIMER_INTERRUPT_ID as usize].set_handler_fn(timer_interrupt_handler);
        idt[KEYBOARD_INTERRUPT_ID as usize].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    unsafe{
        PICS.lock().initialize();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("Breakpoint Exception\n{:?}", stack_frame);
    serial_println!("Breakpoint Exception\n{:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: u64) {
    println!("Double Fault (error code {})\n{:?}", error_code, stack_frame);
    serial_println!("Double Fault (error code {})\n{:?}", error_code, stack_frame);
    ::halt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
    use interrupt_lock::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut kb = KEYBOARD.lock();
    let port = Port::new(0x60);
    let scancode: u8 = unsafe {port.read()};
    
    if let Ok(Some(key_event)) = kb.add_byte(scancode) {
        if let Some(key) = kb.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(c) => print!("{}", c),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID);
    }
}

use x86_64::structures::idt::PageFaultErrorCode;

extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    println!("PAGE FAULT at address {:?}", Cr2::read());
    println!("{:?}", stack_frame);
    ::halt_loop();
}