use num;
use spidev::{Spidev, SpidevOptions, SPI_MODE_0};
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::mem::transmute;
use std::io::prelude::*;
use num::integer::sqrt;
use std::cmp::{min, max};
use crate::command::Command;
use crate::fonts::font57::Font57;
use crate::fonts::Font;

/// ST7735 driver to connect to TFT displays. The driver allows to draw simple shapes,
/// and reset the display.
///
/// Currently, there is support for using hardware SPI as well as software SPI to
/// communicate to the display. Note that using hardware SPI is much faster and
/// recommended to be used if supported by the connecting device.
///
/// # Examples
///
/// [todo]
///
pub struct ST7734 {
    /// Reset pin.
    rst: Option<Pin>,

    /// SPI clock pin.
    clk: Option<Pin>,

    /// Data/command pin.
    dc: Option<Pin>,

    /// MOSI pin.
    mosi: Option<Pin>,

    /// Hardware SPI
    spi: Option<Spidev>,
}

impl ST7734 {
    pub fn new_with_spi(spi: &str, dc: u64) -> ST7734 {
        let mut spi = Spidev::open(spi).expect("error initializing SPI");
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(20000000)
            .mode(SPI_MODE_0)
            .build();
        spi.configure(&options).expect("error configuring SPI");

        let dc_pin = Pin::new(dc);
        dc_pin.set_direction(Direction::Out);

        let mut display = ST7734 {
            rst: None,
            clk: None,
            dc: Some(dc_pin),
            mosi: None,
            spi: Some(spi),
        };

        display.init();
        display
    }

    pub fn new(rst: Option<u64>, clk: u64, dc: u64, mosi: u64) -> ST7734 {
        let clk_pin = Pin::new(clk);
        clk_pin.set_direction(Direction::Out);
        clk_pin.set_value(0).expect("error while setting clock 0");

        let dc_pin = Pin::new(dc);
        dc_pin.set_direction(Direction::Out);

        let mosi_pin = Pin::new(mosi);
        mosi_pin.set_direction(Direction::Out);

        let rst_pin = match rst {
            Some(r) => {
                let pin = Pin::new(r);
                pin.set_direction(Direction::Out);
                Some(pin)
            },
            None => None
        };

        let mut display = ST7734 {
            rst: rst_pin,
            clk: Some(clk_pin),
            dc: Some(dc_pin),
            mosi: Some(mosi_pin),
            spi: None,
        };

        display.init();
        display
    }

    pub fn init(&mut self) {
        let init_commands: Vec<Command> = vec![
            Command { instruction: Instruction::SWRESET, delay: Some(200), arguments: vec![] },
            Command { instruction: Instruction::SLPOUT, delay: Some(200), arguments: vec![] },
            Command { instruction: Instruction::DISPON, delay: Some(100), arguments: vec![] },
        ];

        self.execute_commands(init_commands);
    }

    fn pulse_clock(&self) {
        self.clk.unwrap().set_value(1).expect("error while pulsing clock");
        self.clk.unwrap().set_value(0).expect("error while pulsing clock");
    }

    fn write_byte(&mut self, value: u8, data: bool) {
        let mode = match data {
            false => 0,
            true => 1
        };

        self.dc.unwrap().set_value(mode).expect("error while writing byte");

        if let Some(ref mut spi) = self.spi {
            spi.write(&[value]);
        } else {
            let mask = 0x80;
            for bit in 0..8 {
                self.mosi.unwrap().set_value(value & (mask >> bit));
                self.pulse_clock();
            }
        }

    }

    fn write_word(&mut self, value: u16) {
        let bytes: [u8; 2] = unsafe { transmute(value.to_be()) };
        self.write_byte(bytes[0], true);
        self.write_byte(bytes[1], true);
    }

    fn execute_commands(&mut self, commands: Vec<Command>) {
        for cmd in &commands {
            self.execute_command(cmd);
        }
    }

    fn execute_command(&mut self, cmd: &Command) {
        self.write_byte(num::ToPrimitive::to_u8(&cmd.instruction).unwrap(), false);

        match cmd.delay {
            Some(d) => {
                if cmd.arguments.len() > 0 {
                    sleep(Duration::from_millis(d));
                }
            },
            None => {
                for argument in &cmd.arguments {
                    self.write_byte(*argument, true);
                }
            }
        }
    }

    fn write_color(&mut self, color: u32) {
        let bytes: [u8; 4] = unsafe { transmute(color.to_be()) };

        if let Some(ref mut spi) = self.spi {
            self.dc.unwrap().set_value(1).expect("error while writing byte");
            spi.write(&[bytes[1], bytes[2], bytes[3]]);
        } else {
            self.write_byte(bytes[1], true);
            self.write_byte(bytes[2], true);
            self.write_byte(bytes[3], true);
        }
    }

    fn set_address_window(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) {
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::CASET).unwrap(), false);
        self.write_word(x0);
        self.write_word(x1);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RASET).unwrap(), false);
        self.write_word(y0);
        self.write_word(y1);
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u32) {
        self.set_address_window(x, y, x, y);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);
        self.write_color(color);
    }

    pub fn draw_rect(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: u32) {
        let width = x1 - x0 + 1;
        let height = y1 - y0 + 1;
        self.set_address_window(x0, y0, x1, y1);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);
        for i in 0..(width * height) {
            self.write_color(color);
        }
    }

    pub fn draw_horizontal_line(&mut self, x0: u16, x1: u16, y: u16, color: u32) {
        let length = x1 - x0 + 1;
        self.set_address_window(x0, y, x1, y);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);
        // todo: move to draw pixel
        for i in 0..length {
            self.write_color(color);
        }
    }

    pub fn draw_vertical_line(&mut self, x: u16, y0: u16, y1: u16, color: u32) {
        let length = y1 - y0 + 1;
        self.set_address_window(x, y0, x, y1);
        self.write_byte(num::ToPrimitive::to_u8(&Instruction::RAMWR).unwrap(), false);

        // todo: move to draw pixel
        for i in 0..length {
            self.write_color(color);
        }
    }

    pub fn draw_line(&mut self, x0: u16, y0: u16, x1: u16, y1: u16, color: u32) {
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

    pub fn draw_circle(&mut self, x_pos: u16, y_pos: u16, radius: u16, color: u32) {
        let x_end = ((0.7071 * (radius as f32)) + 1.0) as u16;

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

    pub fn draw_filled_circle(&mut self, x_pos: u16, y_pos: u16, radius: u16, color: u32) {
        let r2 = radius * radius;
        for x in 0..radius {
            let y = sqrt(r2 - x * x);
            let y0 = y_pos - y;
            let y1 = y_pos + y;
            self.draw_vertical_line(x_pos + x, y0, y1, color);
            self.draw_vertical_line(x_pos - x, y0, y1, color);
        }
    }

    pub fn draw_character(&mut self, c: char, x: u16, y: u16, color: u32) {
        let character_data = Font57::get_char(c);
        eprintln!("{:?}", character_data);

        let mut mask = 0x01;

        for row in 0..7 {
            for col in 0..5 {
                let bit = character_data[col] & (mask << row);

                eprintln!("-");

                if bit != 0 {
                    eprintln!("{:?} {:?}", row, col);
                    self.draw_pixel(x - (col as u16), y - (row as u16), color);
                }
            }
        }
    }

    pub fn fill_screen(&mut self, color: u32) {
        self.draw_rect(0, 0, 127, 159, color);
    }

    pub fn clear_screen(&mut self) {
        self.draw_rect(0, 0, 127, 159, 0x0);
    }
}

#[derive(FromPrimitive, ToPrimitive)]
/// System instructions.
pub enum Instruction {
    NOP     = 0x00,
    SWRESET = 0x01,
    RDDID   = 0x04,
    RDDST   = 0x09,
    SLPIN   = 0x10,
    SLPOUT  = 0x11,
    PTLON   = 0x12,
    NORON   = 0x13,
    INVOFF  = 0x20,
    INVON   = 0x21,
    DISPOFF = 0x28,
    DISPON  = 0x29,
    CASET   = 0x2A,
    RASET   = 0x2B,
    RAMWR   = 0x2C,
    RAMRD   = 0x2E,
    PTLAR   = 0x30,
    COLMOD  = 0x3A,
    MADCTL  = 0x36,
    FRMCTR1 = 0xB1,
    FRMCTR2 = 0xB2,
    FRMCTR3 = 0xB3,
    INVCTR  = 0xB4,
    DISSET5 = 0xB6,
    PWCTR1  = 0xC0,
    PWCTR2  = 0xC1,
    PWCTR3  = 0xC2,
    PWCTR4  = 0xC3,
    PWCTR5  = 0xC4,
    VMCTR1  = 0xC5,
    RDID1   = 0xDA,
    RDID2   = 0xDB,
    RDID3   = 0xDC,
    RDID4   = 0xDD,
    PWCTR6  = 0xFC,
    GMCTRP1 = 0xE0,
    GMCTRN1 = 0xE1,
}