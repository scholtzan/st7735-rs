use num;

/// Represents a pixel color.
pub struct Color {
    pub hex: u16,
}

impl Color {
    /// Create color from hex value.
    ///
    /// # Example
    ///
    /// ```
    /// let color_green = Color::from_hex(0x00FF00);
    /// ```
    pub fn from_hex(hex: u16) -> Color {
        Color { hex }
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
            hex: num::ToPrimitive::to_u16(&default_color).unwrap(),
        }
    }

    /// Create color from RGB values.
    ///
    /// # Example
    ///
    /// ```
    /// let color_red = Color::from_rgb(255, 0, 0);
    /// ```
    pub fn from_rgb(red: u16, green: u16, blue: u16) -> Color {
        let r = red & 0x1F;
        let g = green & 0x3F;
        let b = blue & 0x1F;
        Color {
            hex: (r << 11) + (g << 5) + b,
        }
    }
}

/// Set of hex values for default colors.
#[derive(FromPrimitive, ToPrimitive)]
pub enum DefaultColor {
    Black = 0x0000,
    White = 0xFFFF,
    Red = 0xF800,
    Green = 0x0400,
    Blue = 0x001F,
    Lime = 0x07E0,
    Cyan = 0x07FF,
    Magenta = 0xF81F,
    Yellow = 0xFFE0,
    Purple = 0x8010,
    Navy = 0x0010,
    Teal = 0x0410,
    Olive = 0x8400,
    Gray = 0x8410,
}
