extern crate st7735;
use st7735::color::{Color, DefaultColor};
use st7735::ST7734;
use linux_embedded_hal::Delay;
use linux_embedded_hal::Pin;
use linux_embedded_hal::Spidev;

fn main() {
    let delay = Delay;
    let clk = Pin::new(24);
    let dc = Pin::new(25);
    let mosi = Pin::new(23);
    let mut display: ST7734<Spidev, Pin, Delay> = ST7734::new_with_gpio(None, clk, dc, mosi, delay);
    display.fill_screen(&Color::from_default(DefaultColor::Blue));
}
