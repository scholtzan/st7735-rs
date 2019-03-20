extern crate st7735;
use st7735::color::{Color, DefaultColor};
use st7735::fonts::font57::Font57;
use st7735::Orientation;
use st7735::ST7734;
use linux_embedded_hal::spidev::{SpidevOptions, SPI_MODE_0};
use linux_embedded_hal::Spidev;
use linux_embedded_hal::Delay;
use linux_embedded_hal::Pin;

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").expect("error initializing SPI");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SPI_MODE_0)
        .build();
    spi.configure(&options).expect("error configuring SPI");

    let mut display = ST7734::new_with_spi(spi, Pin::new(25), Delay);
    display.clear_screen();
    display.set_orientation(&Orientation::Portrait);
    let color_red = Color::from_default(DefaultColor::Red);
    display.draw_horizontal_line(0, 128, 20, &color_red);
    display.draw_horizontal_line(0, 128, 140, &color_red);
    display.draw_rect(30, 30, 60, 70, &Color::from_default(DefaultColor::Blue));
    display.draw_circle(90, 50, 20, &Color::from_default(DefaultColor::Blue));
    display.draw_filled_circle(45, 90, 20, &Color::from_default(DefaultColor::Blue));
    display.draw_character(
        '!',
        80,
        85,
        &Color::from_default(DefaultColor::White),
        Font57 {},
    );
    display.draw_character(
        'i',
        85,
        85,
        &Color::from_default(DefaultColor::White),
        Font57 {},
    );
    display.draw_character(
        'H',
        90,
        85,
        &Color::from_default(DefaultColor::White),
        Font57 {},
    );
    display.draw_line(0, 110, 128, 130, &Color::from_default(DefaultColor::Green));
}
