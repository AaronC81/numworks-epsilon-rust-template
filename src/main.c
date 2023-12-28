#include <eadk.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

const char eadk_app_name[] __attribute__((section(".rodata.eadk_app_name"))) = "App";
const uint32_t eadk_api_level  __attribute__((section(".rodata.eadk_api_level"))) = 0;

/* --- Rust support ---------------------------- */

void rs_main(void); 

void api_push_rect_uniform(uint16_t x, uint16_t y, uint16_t width, uint16_t height, uint16_t color) {
  eadk_rect_t rect = { x, y, width, height };
  eadk_display_push_rect_uniform(rect, (eadk_color_t){color});
}

void api_draw_string(char *str, uint16_t x, uint16_t y, uint8_t large_font, uint16_t text_color, uint16_t bg_color) {
  eadk_display_draw_string(str, (eadk_point_t){x, y}, large_font, text_color, bg_color);
}

/* --------------------------------------------- */

int main(int argc, char * argv[]) {
  rs_main();
  while (1) {
    eadk_keyboard_scan();
  }
}
