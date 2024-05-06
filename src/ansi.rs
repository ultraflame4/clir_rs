use crate::color::{Color, RGBColorU8};

pub fn convert(color: Color, truergb: bool) -> ansi_term::Color {
    let rgb: RGBColorU8 = color.into();
    if truergb {
        ansi_term::Color::RGB(rgb.r, rgb.g, rgb.b)
    } else{
        let i = ansi_colours::ansi256_from_rgb(rgb.u32());
        ansi_term::Color::Fixed(i)
    }
}

