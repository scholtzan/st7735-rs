extern crate st7735;
use st7735::fonts::font57::Font57;
use st7735::ST7734;
use std::thread::sleep;
use std::time::Duration;
use st7735::Orientation;

fn main() {
    let mut display = ST7734::new_with_spi("/dev/spidev0.0", 25);
    display.clear_screen();
    display.draw_horizontal_line(0, 128, 20, 0xFF0000);
    display.draw_horizontal_line(0, 128, 140, 0xFF0000);
    display.draw_rect(30, 30, 60, 70, 0xBBFFAA);
    display.draw_circle(90, 50, 20, 0x0000FF);
    display.draw_filled_circle(45, 90, 20, 0xAA00FF);
    display.draw_character('!', 80, 85, 0x00FF00, Font57 {});
    display.draw_character('i', 85, 85, 0x00FF00, Font57 {});
    display.draw_character('H', 90, 85, 0x00FF00, Font57 {});
    display.draw_line(0, 110, 128, 130, 0xFFBBAA);
}
