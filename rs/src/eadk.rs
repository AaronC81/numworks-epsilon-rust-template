// Links against the C-provided `eadk_bridge`.
//
// Targets the `eadk_bridge` for `nwlink` 0.0.17.

extern "C" {
    // Keyboard and Events
    fn eadk_bridge__keyboard_scan() -> u64;

    // Display
    fn eadk_bridge__display_push_rect(x: u16, y: u16, width: u16, height: u16, pixels: *const u16);
    fn eadk_bridge__display_push_rect_uniform(x: u16, y: u16, width: u16, height: u16, color: u16);
    fn eadk_bridge__display_pull_rect(x: u16, y: u16, width: u16, height: u16, pixels: *mut u16);
    fn eadk_bridge__display_wait_for_vblank() -> u8;
    fn eadk_bridge__display_draw_string(str: *const u8, x: u16, y: u16, large_font: u8, text_color: u16, bg_color: u16);
    
    // Timing
    fn eadk_bridge__timing_usleep(us: u32);
    fn eadk_bridge__timing_msleep(ms: u32);
    fn eadk_bridge__timing_millis() -> u64;
}

pub mod input {
    /// Scans the keyboard and returns its state.
    pub fn keyboard_scan() -> u64 {
        unsafe { super::eadk_bridge__keyboard_scan() }
    }
}

pub mod display {
    /// A rectangle on the display.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Rect {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
    }
    
    /// A collection of sized pixel data which can be drawn to the screen as an image.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Bitmap<'d> {
        pub width: u16,
        pub height: u16,

        // Expected to have size (width * height)
        pub data: &'d [Color],
    }

    /// A point on the display.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: u16,
        pub y: u16,
    }

    /// An RGB565 color.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[repr(C)]
    pub struct Color(pub u16);
    impl Color {
        pub const WHITE: Color = Color(0xFFFF);
        pub const BLACK: Color = Color(0);
        pub const RED:   Color = Color(0xF800);
        pub const GREEN: Color = Color(0x07E0);
        pub const BLUE:  Color = Color(0x001F);
    }

    /// The fonts available for drawing text.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum Font {
        Small,
        Large,
    }

    impl Font {
        fn to_large_font_arg(self) -> u8 {
            if self == Font::Large { 1 } else { 0 }
        }
    }

    /// Fills a [Rect] with a [Color].
    pub fn fill(rect: Rect, color: Color) {
        unsafe {
            super::eadk_bridge__display_push_rect_uniform(rect.x, rect.y, rect.width, rect.height, color.0)
        }
    }

    /// Draws a [Bitmap] to a given [Point] on the display.
    pub fn draw_bitmap(pt: Point, bitmap: Bitmap) {
        // TODO: validate dimensions
        unsafe {
            super::eadk_bridge__display_push_rect(pt.x, pt.y, bitmap.width, bitmap.height, bitmap.data.as_ptr() as *const u16)
        }
    }

    /// Writes a null-terminated sequence of ASCII bytes to the display.
    /// 
    /// It is the caller's responsibility to ensure that the given slice contains a NUL (0).
    /// Undefined Behaviour will occur if it doesn't.
    pub unsafe fn write_string_null_terminated(str: &[u8], pt: Point, font: Font, text_color: Color, bg_color: Color) {
        super::eadk_bridge__display_draw_string(str.as_ptr(), pt.x, pt.y, font.to_large_font_arg(), text_color.0, bg_color.0)
    }

    /// Waits for display VBLANK.
    /// 
    /// Honestly, I don't know what this does, but I'll bridge it anyway in case it's important for
    /// something.
    pub fn wait_for_vblank() -> bool {
        unsafe {
            super::eadk_bridge__display_wait_for_vblank() > 0
        }
    }
}
