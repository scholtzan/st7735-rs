#![crate_type = "lib"]
#![crate_name = "st7735"]

//! todo
//!

#[macro_use]
extern crate num_derive;

pub mod st7735;
pub mod command;
pub mod color;
pub mod fonts;

// todo

//https://github.com/arduino-libraries/TFT/blob/master/src/utility/Adafruit_ST7735.cpp

//const RCMD1: Vec<Command> = vec![
//    Command { instruction: Instruction::SWRESET, delay: true, arguments: [150]},
//    Command { instruction: Instruction::SLPOUT, delay: true, arguments: [255]},
//    Command { instruction: Instruction::FRMCTR1, delay: false, arguments: [0x01, 0x2C, 0x2D]},
//    Command { instruction: Instruction::FRMCTR2, delay: false, arguments: [0x01, 0x2C, 0x2D]},
//    Command { instruction: Instruction::FRMCTR3, delay: false, arguments: [0x01, 0x2C, 0x2D, 0x01, 0x2C, 0x2D]},
//    Command { instruction: Instruction::INVCTR, delay: false, arguments: [0x07]},
//    Command { instruction: Instruction::PWCTR1, delay: false, arguments: [0xA2, 0x02, 0x84]},
//    Command { instruction: Instruction::PWCTR2, delay: false, arguments: [0xC5]},
//    Command { instruction: Instruction::PWCTR3, delay: false, arguments: [0x0A, 0x00]},
//    Command { instruction: Instruction::PWCTR4, delay: false, arguments: [0x8A, 0x2A]},
//    Command { instruction: Instruction::PWCTR5, delay: false, arguments: [0x8A, 0xEE]},
//    Command { instruction: Instruction::VMCTR1, delay: false, arguments: [0x0E]},
//    Command { instruction: Instruction::INVOFF, delay: false, arguments: []},
//    Command { instruction: Instruction::MADCTL, delay: false, arguments: [0xC8]},
//    Command { instruction: Instruction::COLMOD, delay: false, arguments: [0x05]},
//];
//
//const RCMD2_GREEN: Vec<Command> = vec![
//    Command { instruction: Instruction::CASET, delay: false, arguments: [0x00, 0x02, 0x00, 0x7F+0x02]},
//    Command { instruction: Instruction::RASET, delay: false, arguments: [0x00, 0x01, 0x00, 0x9F+0x01]},
//];
//
//const RCMD2_RED: Vec<Command> = vec![
//    Command { instruction: Instruction::CASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x7F]},
//    Command { instruction: Instruction::RASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x9F]},
//];
//
//const RCMD2_GREEN144: Vec<Command> = vec![
//    Command { instruction: Instruction::CASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x7F]},
//    Command { instruction: Instruction::RASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x7F]},
//];
//
//const RCMD2_GREEN160X80: Vec<Command> = vec![
//    Command { instruction: Instruction::CASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x7F]},
//    Command { instruction: Instruction::RASET, delay: false, arguments: [0x00, 0x00, 0x00, 0x9F]},
//];
//
//const RCMD3: Vec<Command> = vec![
//    Command { instruction: Instruction::GMCTRP1, delay: false, arguments: [0x02, 0x1c, 0x07, 0x12, 0x37, 0x32, 0x29, 0x2d, 0x29, 0x25, 0x2B, 0x39, 0x00, 0x01, 0x03, 0x10]},
//    Command { instruction: Instruction::GMCTRN1, delay: false, arguments: [0x03, 0x1d, 0x07, 0x06, 0x2E, 0x2C, 0x29, 0x2D, 0x2E, 0x2E, 0x37, 0x3F, 0x00, 0x00, 0x02, 0x10]},
//    Command { instruction: Instruction::NORON, delay: true, arguments: [10]},
//    Command { instruction: Instruction::DISPON, delay: true, arguments: [100]},
//];


