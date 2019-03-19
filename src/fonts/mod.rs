pub mod font57;
use alloc::vec::Vec;

/// Font trait implemented by fonts that can be used to display text on the display.
pub trait Font {
    /// Returns the bit representation of character `c` that can be displayed on the display.
    fn get_char(c: char) -> Vec<u8>;
}
