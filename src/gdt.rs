use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const SIZE: usize = 4096;
            static mut STACK: [u8; SIZE] = [0; SIZE];
            let stack_start = VirtAddr::from_ptr(unsafe{&STACK});
            let stack_end = stack_start + SIZE;
            stack_end
        };
        tss
    };
}

struct GdtWithSelectors {
    gdt: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: GdtWithSelectors = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        GdtWithSelectors{ gdt, code_selector, tss_selector }
    };
}

pub fn init() {
    use x86_64::instructions::{segmentation::set_cs, tables::load_tss};
    GDT.gdt.load();
    unsafe {
        set_cs(GDT.code_selector);
        load_tss(GDT.tss_selector);
    }
}