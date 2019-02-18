use num;

/// Represents a pixel color.
pub struct Color {
    pub hex: u32
}

impl Color {
    /// Create color from hex value.
    ///
    /// # Example
    ///
    /// ```
    /// let color_green = Color::from_hex(0x00FF00);
    /// ```
    pub fn from_hex(hex: u32) -> Color {
        Color {
            hex
        }
    }

    /// Create color from a `DefaultColor`.
    ///
    /// # Example
    ///
    /// ```
    /// let color_red = Color::from_default(DefaultColor::Red);
    /// ```
    pub fn from_default(default_color: DefaultColor) -> Color {
        Color {
            hex: num::ToPrimitive::to_u32(&default_color).unwrap()
        }
    }

    /// Create color from RGB values.
    ///
    /// # Example
    ///
    /// ```
    /// let color_red = Color::from_rgb(255, 0, 0);
    /// ```
    pub fn from_rgb(red: u32, green: u32, blue: u32) -> Color {
        let r = red & 0x1F;
        let g = green & 0x3F;
        let b = blue & 0x1F;
        Color {
            hex: (r << 16) + (g << 8) + b
        }
    }
}

/// Set of hex values for default colors.
#[derive(FromPrimitive, ToPrimitive)]
pub enum DefaultColor {
    Black = 0x000000,
    White = 0xFFFFFF,
    Red = 0xFF0000,
    Green = 0x00FF00,
    Blue = 0x0000FF,
    // todo
}