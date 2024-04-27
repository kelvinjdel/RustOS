//main.rs

#![no_std]
//intended for bare metal, no operating system overhead

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
loop {}
}

fn main() {
}
