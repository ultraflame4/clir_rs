use std::str::Chars;

pub const BRAILLE: &str = concat!(
    " ⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿",
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
/// Cell bitmask ordering:  8,7,6,5,4,3,2,1 -> Where every 2 bits is a row. \
/// Braille ordering:       8,7,6,4,2,5,3,1 -> DO NOT CHANGE I HAD TO MANUALLY TRIAL & ERROR THIS ORDERING
///
/// See https://en.wikipedia.org/wiki/Braille_Patterns#/media/File:Braille8dotCellNumbering.svg
pub fn cell_bitmask_to_char_index(bitmask_: u8) -> u8 {
    let bitmask : u8= bitmask_;
    let mut result = 0;

    // Keep only the last & first 3 bits so we can work on position 5-2
    result |= bitmask & 0b11100001;

    result |=
    // Isolate the 5th bit, & move it to the right by 2. And then add to the result
    (bitmask & 0b00010000) >> 2;


    result |=
    // Isolate the 4th bit, & move it to the left by 1. And then add to the result
    (bitmask & 0b00001000) << 1;


    result |=
    // Isolate the 3rd bit, & move it to the right by 1. And then add to the result
    (bitmask & 0b00000100) >> 1;


    result |=
    // Isolate the 2nd bit, & move it to the left by 2. And then add to the result
    (bitmask & 0b00000010) << 2;
    result

    // result.reverse_bits()
}
