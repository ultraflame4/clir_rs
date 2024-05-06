use std::str::Chars;

pub const BRAILLE: &str = concat!(
    "⣿⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿",
    "⡀⡁⡂⡃⡄⡅⡆⡇⡈⡉⡊⡋⡌⡍⡎⡏⡐⡑⡒⡓⡔⡕⡖⡗⡘⡙⡚⡛⡜⡝⡞⡟⡠⡡⡢⡣⡤⡥⡦⡧⡨⡩⡪⡫⡬⡭⡮⡯⡰⡱⡲⡳⡴⡵⡶⡷⡸⡹⡺⡻⡼⡽⡾",
    "⡿⢀⢁⢂⢃⢄⢅⢆⢇⢈⢉⢊⢋⢌⢍⢎⢏⢐⢑⢒⢓⢔⢕⢖⢗⢘⢙⢚⢛⢜⢝⢞⢟⢠⢡⢢⢣⢤⢥⢦⢧⢨⢩⢪⢫⢬⢭⢮⢯⢰⢱⢲⢳⢴⢵⢶⢷⢸⢹⢺⢻⢼⢽",
    "⢾⢿⣀⣁⣂⣃⣄⣅⣆⣇⣈⣉⣊⣋⣌⣍⣎⣏⣐⣑⣒⣓⣔⣕⣖⣗⣘⣙⣚⣛⣜⣝⣞⣟⣠⣡⣢⣣⣤⣥⣦⣧⣨⣩⣪⣫⣬⣭⣮⣯⣰⣱⣲⣳⣴⣵⣶⣷⣸⣹⣺⣻⣼⣽⣾⣿"
);


pub enum CharsetWarnings{
    None,
    NotEnoughCharacters
}

/// Converts a cell bitmask to the proper index for indexing the various char sets. \
/// This has to be done because the way braille patterns increments is not the same as cell mask bits \
///
/// Cell bitmask ordering:  1,2,3,4,5,6,7,8 -> Where every 2 bits is a row. \
/// Braille ordering:       1,4,2,5,3,6,7,8
///
/// See https://en.wikipedia.org/wiki/Braille_Patterns#/media/File:Braille8dotCellNumbering.svg
pub fn cell_bitmask_to_char_index(bitmask_: u8) -> u8 {
    let bitmask : u8= bitmask_;
    let mut result = 0;

    // Keep only the 1st & last 3 bits so we can work on position 2-5
    result |= bitmask & 0b10000111;

    result |=
    // Isolate the 2nd bit, & move it to the right by 1. And then add to the result
    (bitmask & 0b01000000) >> 1;


    result |=
    // Isolate the 3rd bit, & move it to the right by 2. And then add to the result
    (bitmask & 0b00100000) >> 2;


    result |=
    // Isolate the 4th bit, & move it to the left by 2. And then add to the result
    (bitmask & 0b00010000) << 2;


    result |=
    // Isolate the 5th bit, & move it to the left by 1. And then add to the result
    (bitmask & 0b00001000) << 1;
    result

    // result.reverse_bits()
}
