extern crate st7735;
use st7735::fonts::font57::Font57;
use st7735::ST7734;
use std::thread::sleep;
use std::time::Duration;
use st7735::color::{Color, DefaultColor};

fn main() {
    let mut display = ST7734::new(None, 24, 25, 23);
    display.fill_screen(&Color::from_default(DefaultColor::Blue));
}
