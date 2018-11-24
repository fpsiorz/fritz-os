use x86_64::structures::idt::{InterruptDescriptorTable, ExceptionStackFrame};
use gdt;

lazy_static!{
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
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



