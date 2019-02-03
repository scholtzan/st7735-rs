use st7735::ST7734;
use std::thread::sleep;
use std::time::Duration;

fn main() {
//    let display = ST7734::new(None, 24, 25, 23);
//    eprintln!("initialized");
//    display.fill_screen(0x00FF00);
//    sleep(Duration::from_millis(10000));
//    eprintln!("done");

    let mut display = ST7734::new_with_spi("/dev/spidev0.0", 25);
    display.fill_screen(0x00FF00);
    sleep(Duration::from_millis(5000));
    display.draw_horizontal_line(0, 60, 20, 0xFF0000);
    display.draw_rect(50, 50, 60, 60, 0xFF0000);
    display.draw_circle(80, 100, 20, 0x0000FF);
    display.draw_filled_circle(80, 120, 20, 0x0000FF);

}