use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use gdt;
use pic8259_simple::ChainedPics;
use spin;

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = PIC1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = 
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET)});

const TIMER_INTERRUPT_ID: u8 = PIC1_OFFSET + 0;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[TIMER_INTERRUPT_ID as usize].set_handler_fn(timer_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    unsafe{
        PICS.lock().initialize();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("Breakpoint Exception\n{:?}", stack_frame);
    serial_println!("Breakpoint Exception\n{:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    println!("Double Fault (error code {})\n{:?}", error_code, stack_frame);
    serial_println!("Double Fault (error code {})\n{:?}", error_code, stack_frame);
    ::halt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: &mut ExceptionStackFrame) {
    print!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    }
}