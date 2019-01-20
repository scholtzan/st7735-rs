use st7735::ST7734;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let display = ST7734::new(None, 24, 25, 23);
    eprintln!("initialized");
    display.fill_screen(0x00FF00);
    sleep(Duration::from_millis(10000));
    eprintln!("done");
}