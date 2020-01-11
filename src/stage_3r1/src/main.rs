#![no_std]
#![no_main]

use plankton::{print, println};
use stage_3r1::{init, pm};

#[link_section = ".fisrt"]
#[no_mangle]
extern "C" fn stage3(kernel_size: u16, inird_size: u16, cmd_line: &[u8]) -> ! {
    println!("Stage3:");
    println!("  Initializing system.");
    init::setup(kernel_size, inird_size, cmd_line);
    if kernel_size > 0 {
        println!("  Decompressing kernel ...");
        pm::move_to_protect();
    } else {
        println!("  No bzip2 compressed kernel. Stopped ...");
        loop {}
    }
}