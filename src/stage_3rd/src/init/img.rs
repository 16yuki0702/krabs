use plankton::layout::{INITRD_START, KERNEL_SIZE};
use plankton::mem::MemoryRegion;

pub fn set_image(kernel_size: u32, initrd_size: u32) {
    let zero_page = MemoryRegion::new(0x000, 4096);
    zero_page.write_u32(0x218, INITRD_START);
    zero_page.write_u32(0x21C, initrd_size);
    zero_page.write_u16(0x1fc, 0x0100);
    zero_page.write_u32(KERNEL_SIZE as u64, kernel_size);
}
