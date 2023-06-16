#![no_std]
// crt0 entrypoint is from std, but we don't use std
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    const WIDTH: i32 = 80;
    const HEIGHT: i32 = 25;

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let x = i + WIDTH * 2 * j;
            unsafe {
                *vga_buffer.offset(x as isize * 2) = x as u8;
                *vga_buffer.offset(x as isize * 2 + 1) = x as u8;
            }
        }
    }

    loop {}
}
