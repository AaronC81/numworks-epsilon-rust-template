#![no_std]

use eadk::{display::{self, Rect, Color, Font, Point}, input};

pub mod eadk;

#[no_mangle]
pub extern "C" fn rs_main() {
    unsafe {
        display::fill(
            Rect { x: 10, y: 10, width: 10, height: 10 },
            Color(0xe426),
        );
        display::write_string_null_terminated(
            b"Hello from Rust!\0",
            Point { x: 50, y: 50 },
            Font::Large,
            Color(0),
            Color(0xFFFF)
        );
        
        loop {
            let colour =
                if (input::keyboard_scan() & 0b1) > 0 {
                    Color(0)
                } else {
                    Color(0xFFFF)
                };
            display::fill(
                Rect { x: 50, y: 100, width: 20, height: 20 },
                colour,
            );
        }
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
