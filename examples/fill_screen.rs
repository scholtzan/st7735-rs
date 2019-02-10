extern crate st7735;
use st7735::fonts::font57::Font57;
use st7735::ST7734;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut display = ST7734::new(None, 24, 25, 23);
    display.fill_screen(0x00FF00);
}
