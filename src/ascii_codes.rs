use crate::color::Color;

pub fn fore(color: Color, truergb: bool) {
    let (r, g, b): (u8, u8, u8) = (
        (color.r * color.a * 255.0).round() as u8,
        (color.g * color.a * 255.0).round() as u8,
        (color.b * color.a * 255.0).round() as u8,
    );
    let rgb = ansi_term::Color::RGB(r, g, b);
    let color = if (truergb) {
        rgb
    } else{
        ansi_colours::ansi256_from_rgb(r,g,b)
    };
    
}
