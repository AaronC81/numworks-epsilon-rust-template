// Adapts EADK functions with "simpler" type signatures, allowing them to be called from Rust
// without problems.
//
// Targets the `eadk.h` distributed with `nwlink` 0.0.17.

#include <eadk.h>
#include <stdint.h>

void eadk_bridge__display_push_rect_uniform(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t color) {
  eadk_display_push_rect_uniform((eadk_rect_t){ x, y, width, height }, (eadk_color_t){color});
}

void eadk_bridge__display_draw_string(char *str, uint16_t x, uint16_t y, uint8_t large_font, uint16_t text_color, uint16_t bg_color) {
  eadk_display_draw_string(str, (eadk_point_t){x, y}, large_font, text_color, bg_color);
}

