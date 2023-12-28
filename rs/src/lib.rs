#![no_std]

extern "C" {
    fn eadk_bridge__display_push_rect_uniform(x: u16, y: u16, width: u16, height: u16, color: u16);
    fn eadk_bridge__display_draw_string(str: *const u8, x: u16, y: u16, large_font: u8, text_color: u16, bg_color: u16);
}

#[no_mangle]
pub extern "C" fn rs_main() {
    unsafe {
        eadk_bridge__display_push_rect_uniform(10, 10, 10, 10, 0xe426);
        eadk_bridge__display_draw_string(b"Hello from Rust!\0".as_ptr(), 50, 50, 0, 0, 0xFFFF);
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
