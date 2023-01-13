use ansi_term::Color;

pub const NATIVE_DARK: &str = "#45475a";

// trait to convert hex color to rgb, solely returns Color struct from ansi_term
pub trait HexToRgb {
    fn hex_to_rgb(hex: &str) -> Color;
}

impl HexToRgb for Color {
    fn hex_to_rgb(hex: &str) -> Color {
        let hex_str = u32::from_str_radix(&hex[1..], 16).unwrap();
        let r = ((hex_str >> 16) & 0xFF) as u8;
        let g = ((hex_str >> 8) & 0xFF) as u8;
        let b = (hex_str & 0xFF) as u8;
        Color::RGB(r, g, b)
    }
}
