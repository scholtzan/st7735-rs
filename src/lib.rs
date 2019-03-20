#![crate_type = "lib"]
#![crate_name = "st7735"]

//! This crate provides a ST7735 driver to connect to TFT displays.
//!
//! Currently, there is support for using hardware SPI as well as software SPI to
//! communicate to the display. Note that using hardware SPI is much faster and
//! recommended to be used if supported by the connecting device.
//!
//! The driver also provides a simple graphics library which currently supports drawing the
//! following shapes:
//! * Rectangles (filled and border only)
//! * Circles (filled and border only)
//! * Lines (horizontal, vertical, and diagonal)
//! * Text (characters)
//!
//! # Examples
//!
//! ```
//! let mut display = ST7734::new_with_spi("/dev/spidev0.0", 25);
//! display.set_orientation(&Orientation::Portrait);
//! display.draw_rect(30, 30, 60, 70, &Color::from_default(DefaultColor::Blue));
//! ```
#![no_std]
#![feature(alloc, slice_concat_ext)]

extern crate embedded_hal;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate alloc;

pub mod color;
pub mod command;
pub mod fonts;

use crate::color::{Color, DefaultColor};
use crate::command::{Command, Instruction};
use crate::fonts::Font;

use alloc::prelude::SliceConcatExt;
use alloc::vec::Vec;
use embedded_hal::blocking::spi;
use embedded_hal::digital::OutputPin;
use embedded_hal::blocking::delay::DelayMs;
use num;
use num::integer::sqrt;
use core::cmp::{max, min};
use core::mem::transmute;

/// ST7735 driver to connect to TFT displays. The driver allows to draw simple shapes,
/// and reset the display.
///
/// Currently, there is support for using hardware SPI as well as software SPI to
/// communicate to the display. Note that using hardware SPI is much faster and
/// recommended to be used if supported by the connecting device.
///
/// # Examples
///
/// ```
/// let mut display = ST7734::new_with_spi("/dev/spidev0.0", 25);
/// display.set_orientation(&Orientation::Portrait);
/// display.draw_rect(30, 30, 60, 70, &Color::from_default(DefaultColor::Blue));
/// ```
///
pub struct ST7734<SPI, PIN, DELAY> {
    /// Reset pin.
    rst: Option<PIN>,

    /// SPI clock pin.
    clk: Option<PIN>,

    /// Data/command pin.
    dc: Option<PIN>,

    /// MOSI pin.
    mosi: Option<PIN>,

    /// Hardware SPI
    spi: Option<SPI>,

    delay: DELAY
}

/// Display orientation.
#[derive(FromPrimitive, ToPrimitive)]
pub enum Orientation {
    Portrait = 0x00,
    Landscape = 0x60,
    PortraitSwapped = 0xC0,
    LandScapeSwapped = 0xA0,
}

impl<SPI, PIN, DELAY> ST7734<SPI, PIN, DELAY>
where
    SPI: spi::Write<u8>,
    PIN: OutputPin,
    DELAY: DelayMs<u64> {

    /// Creates a new driver instance that uses hardware SPI.
    pub fn new_with_spi(spi: SPI, dc: PIN, delay: DELAY) -> ST7734<SPI, PIN, DELAY> {
        let mut display = ST7734 {
            rst: None,
            clk: None,
            dc: Some(dc),
            mosi: None,
            spi: Some(spi),
            delay
        };

        display.init();
        display
    }

    /// Creates a new driver instance that uses software SPI using the provided pins.
    pub fn new_with_gpio(rst: Option<PIN>, clk: PIN, dc: PIN, mosi: PIN, delay: DELAY) -> ST7734<SPI, PIN, DELAY> {
        let mut display = ST7734 {
            rst,
            clk: Some(clk),
            dc: Some(dc),
            mosi: Some(mosi),
            spi: None,
            delay
        };

        display.init();
        display
    }

    /// Runs commands to initialize the display.
    fn init(&mut self) {
        self.hard_reset();

        let init_commands: Vec<Command> = vec![
            Command {
                instruction: Instruction::SWRESET,
                delay: Some(200),
                arguments: vec![],
            },
            Command {
                instruction: Instruction::SLPOUT,
                delay: Some(200),
                arguments: vec![],
            },
            Command {
                instruction: Instruction::COLMOD,
                delay: None,
                arguments: vec![0x05],
            },
            Command {
                instruction: Instruction::FRMCTR1,
                delay: None,
                arguments: vec![0x01, 0x2C, 0x2D],
            },
            Command {
                instruction: Instruction::FRMCTR2,
                delay: None,
                arguments: vec![0x01, 0x2C, 0x2D],
            },
            Command {
                instruction: Instruction::FRMCTR3,
                delay: None,
                arguments: vec![0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D],
            },
            Command {
                instruction: Instruction::INVCTR,
                delay: None,
                arguments: vec![0x07],
            },
            Command {
                instruction: Instruction::PWCTR1,
                delay: None,
                arguments: vec![0xA2, 0x02, 0x84],
            },
            Command {
                instruction: Instruction::PWCTR2,
                delay: None,
                arguments: vec![0xC5],
            },
            Command {
                instruction: Instruction::PWCTR3,
                delay: None,
                arguments: vec![0x0A, 0x00],
            },
            Command {
                instruction: Instruction::PWCTR4,
                delay: None,
                arguments: vec![0x8A, 0x2A],
            },
            Command {
                instruction: Instruction::PWCTR5,
                delay: None,
                arguments: vec![0x8A, 0xEE],
            },
            Command {
                instruction: Instruction::VMCTR1,
                delay: None,
                arguments: vec![0x0E],
            },
            Command {
                instruction: Instruction::INVOFF,
                delay: None,
                arguments: vec![],
            },
            Command {
                instruction: Instruction::MADCTL,
                delay: None,
                arguments: vec![0x00],
            },
            Command {
                instruction: Instruction::DISPON,
                delay: None,
                arguments: vec![],
            },
        ];

        self.execute_commands(init_commands);
    }

    /// Pulses the clock one time.
    fn pulse_clock(&mut self) {
        if let Some(ref mut clk) = self.clk {
            clk.set_high();
            clk.set_low();
        }
    }

    /// Resets the display using the rst pin.
    pub fn hard_reset(&mut self) {
        if let Some(ref mut rst) = self.rst {
            rst.set_high();
            rst.set_low();
        }
    }

    /// Writes one byte to the display which can either be a command or data.
    fn write_byte(&mut self, value: u8, data: bool) {
        if let Some(ref mut dc) = self.dc {
            match data {
                false => dc.set_low(),
                true => dc.set_high(),
            }
        }

        if let Some(ref mut spi) = self.spi {
            let _ = spi.write(&[value]);
        } else {
            let mask = 0x80;
            for bit in 0..8 {
                if let Some(ref mut mosi) = self.mosi {
                    match value & (mask >> bit) {
                        0 => mosi.set_low(),
                        _ => mosi.set_high()
                    }
                }
                self.pulse_clock();
            }
        }
    }

    /// Writes a bulk of pixels to the display.
    fn write_bulk(&mut self, color: &Color, repetitions: u16, count: u16) {
        if let Some(ref mut dc) = self.dc {
            dc.set_low();
        }

        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);

        for _ in 0..=count {
            if let Some(ref mut spi) = self.spi {
                if let Some(ref mut dc) = self.dc {
                    dc.set_high();
                }

                let bytes: [u8; 2] = unsafe { transmute(color.hex.to_be()) };
                let mut byte_array = vec![bytes[0], bytes[1]];

                for _ in 0..=repetitions {
                    byte_array = [&byte_array[..], &bytes[..]].concat()
                }
                let _ = spi.write(&byte_array);
            } else {
                for _ in 0..=repetitions {
                    self.write_color(color);
                }
            }
        }
    }

    /// Writes a data word to the display.
    fn write_word(&mut self, value: u16) {
        let bytes: [u8; 2] = unsafe { transmute(value.to_be()) };
        self.write_byte(bytes[0], true);
        self.write_byte(bytes[1], true);
    }

    /// Sends a list of commands to the display.
    fn execute_commands(&mut self, commands: Vec<Command>) {
        for cmd in &commands {
            self.execute_command(cmd);
        }
    }

    /// Sends a single command to the display.
    fn execute_command(&mut self, cmd: &Command) {
        self.write_byte(num::ToPrimitive::to_u8(&cmd.instruction).unwrap(), false);

        match cmd.delay {
            Some(d) => {
                if cmd.arguments.len() > 0 {
                    self.delay.delay_ms(d);
                }
            }
            None => {
                for argument in &cmd.arguments {
                    self.write_byte(*argument, true);
                }
            }
        }
    }

    /// Sets the color to be used.
    fn write_color(&mut self, color: &Color) {
        let bytes: [u8; 2] = unsafe { transmute(color.hex.to_be()) };

        if let Some(ref mut spi) = self.spi {
            if let Some(ref mut dc) = self.dc {
                dc.set_high();
            }

            let _ = spi.write(&[bytes[0], bytes[1]]);
        } else {
            self.write_byte(bytes[0], true);
            self.write_byte(bytes[1], true);
        }
    }

    /// Sets the address window for the display.
    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::CASET).unwrap(), false);
        self.write_word(x0);
        self.write_word(x1);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RASET).unwrap(), false);
        self.write_word(y0);
        self.write_word(y1);
    }

    /// Changes the display orientation.
    pub fn set_orientation(&mut self, orientation: &Orientation) {
        let command = Command {
            instruction: Instruction::MADCTL,
            delay: None,
            arguments: vec![num::ToPrimitive::to_u8(orientation).unwrap()],
        };
        self.execute_command(&command);
    }

    /// Draws a single pixel with the specified `color` at the defined coordinates on the display.
    pub fn draw_pixel(&mut self, x: u16, y: u16, color: &Color) {
        self.set_address_window(x, y, x, y);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);
        self.write_color(color);
    }

    /// Draws a filled rectangle with the specified `color` on the display.
    pub fn draw_filled_rect(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        let width = x1 - x0 + 1;
        let height = y1 - y0 + 1;
        self.set_address_window(x0, y0, x1, y1);
        self.write_bulk(color, width, height);
    }

    /// Draws a rectangle with the specified `color` as border color on the display.
    pub fn draw_rect(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        self.draw_horizontal_line(x0, x1, y0, color);
        self.draw_horizontal_line(x0, x1, y1, color);
        self.draw_vertical_line(x0, y0, y1, color);
        self.draw_vertical_line(x1, y0, y1, color);
    }

    /// Draws a horizontal with the specified `color` between the provided coordinates on the display.
    pub fn draw_horizontal_line(&mut self, x0: u16, x1: u16, y: u16, color: &Color) {
        let length = x1 - x0 + 1;
        self.set_address_window(x0, y, x1, y);
        self.write_bulk(color, length, 1);
    }

    /// Draws a vertical with the specified `color` between the provided coordinates on the display.
    pub fn draw_vertical_line(&mut self, x: u16, y0: u16, y1: u16, color: &Color) {
        let length = y1 - y0 + 1;
        self.set_address_window(x, y0, x, y1);
        self.write_bulk(color, length, 1);
    }

    /// Draws a line with the specified `color` between the provided coordinates on the display.
    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: &Color) {
        if x0 == x1 {
            self.draw_vertical_line(x0, y0, y1, color);
        } else if y0 == y1 {
            self.draw_horizontal_line(x0, x1, y1, color);
        } else {
            let m = ((max(y1, y0) - min(y0, y1)) as f32) / ((max(x1, x0) - min(x0, x1)) as f32);

            if m < 1.0 {
                for x in x0..=x1 {
                    let y = ((x - x0) as f32) * m + (y0 as f32);
                    self.draw_pixel(x, y as u16, color);
                }
            } else {
                for y in y0..=y1 {
                    let x = ((y - y0) as f32) / m + (x0 as f32);
                    self.draw_pixel(x as u16, y, color);
                }
            }
        }
    }

    /// Draws a circle whose border has the specified `color` around the provided coordinates on the display.
    pub fn draw_circle(&mut self, x_pos: u16, y_pos: u16, radius: u16, color: &Color) {
        let x_end = ((core::f32::consts::FRAC_1_SQRT_2 * (radius as f32)) + 1.0) as u16;

        for x in 0..x_end {
            let y = sqrt(radius * radius - x * x) as u16;
            let u_x = x as u16;
            self.draw_pixel(x_pos + u_x, y_pos + y, color);
            self.draw_pixel(x_pos + u_x, y_pos - y, color);
            self.draw_pixel(x_pos - u_x, y_pos + y, color);
            self.draw_pixel(x_pos - u_x, y_pos - y, color);
            self.draw_pixel(x_pos + y, y_pos + u_x, color);
            self.draw_pixel(x_pos + y, y_pos - u_x, color);
            self.draw_pixel(x_pos - y, y_pos + u_x, color);
            self.draw_pixel(x_pos - y, y_pos - u_x, color);
        }
    }

    /// Draws a circle filled with the specified `color` around the provided coordinates on the display.
    pub fn draw_filled_circle(&mut self, x_pos: u16, y_pos: u16, radius: u16, color: &Color) {
        let r2 = radius * radius;
        for x in 0..radius {
            let y = sqrt(r2 - x * x);
            let y0 = y_pos - y;
            let y1 = y_pos + y;
            self.draw_vertical_line(x_pos + x, y0, y1, color);
            self.draw_vertical_line(x_pos - x, y0, y1, color);
        }
    }

    /// Draws a character filled with the specified `color` and the defined font on the display.
    pub fn draw_character<F: Font>(&mut self, c: char, x: u16, y: u16, color: &Color, _font: F) {
        let character_data = <F as Font>::get_char(c);

        let mask = 0x01;

        for row in 0..7 {
            for col in 0..5 {
                let bit = character_data[col] & (mask << row);

                if bit != 0 {
                    self.draw_pixel(x - (col as u16), y - (row as u16), color);
                }
            }
        }
    }

    /// Fills the entire screen with the specified `color`.
    pub fn fill_screen(&mut self, color: &Color) {
        self.draw_filled_rect(0, 0, 127, 159, color);
    }

    /// Fills the entire screen black.
    pub fn clear_screen(&mut self) {
        self.draw_filled_rect(0, 0, 127, 159, &Color::from_default(DefaultColor::Black));
    }
}
