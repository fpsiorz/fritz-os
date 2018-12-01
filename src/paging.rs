use x86_64::structures::paging::{PageTable, RecursivePageTable, PageTableFlags};


pub fn print_table() {
    let recursive_address = 0o177777_777_777_777_777_0000usize;
    print_table_recursive(recursive_address, 0);
}

fn print_table_recursive(address: usize, depth: usize) {
    let table = unsafe {
        &mut *(address as *mut PageTable)
    };
    for i in 0..512 {
        if table[i].flags().contains(PageTableFlags::PRESENT){
            serial_println!("{:width$}: {:?}",i, table[i], width = 4 * depth);
            if depth < 3 && !table[i].flags().contains(PageTableFlags::HUGE_PAGE){
                let address = (address << 9) | i<<12;
                print_table_recursive(address, depth + 1);
            }      
        }
    }

}