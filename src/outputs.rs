use std::fmt::Display;

use crate::{ansi, cell::ComputedCellGrid, charsets};

pub struct AsciiImageRenderer;

impl AsciiImageRenderer {
    pub fn render(
        grid: &ComputedCellGrid,
        colored: bool,
        charset: Option<&str>,
        transparency_t: f32,
    ) -> (String, charsets::CharsetWarnings) {
        let capacity = (grid.cells.len() + grid.height() as usize)
            * ComputedCellGrid::UTF8_BYTE_SIZE
            * if colored { 8 } else { 1 };
        let mut s = String::with_capacity(capacity);

        let characters: Vec<char> = charset.unwrap_or(charsets::BRAILLE).chars().collect();
        let mut missing_char: bool = false;
        for i in 0..grid.cells.len() {
            let cell = &grid.cells[i];
            let index = charsets::cell_bitmask_to_char_index(cell.bitmask);

            let char_ = if index as usize > characters.len() {
                missing_char = true;
                '?'
            } else {
                characters[index as usize]
                // '?'
            };

            if colored {
                let fore = ansi::convert(cell.fore, true);
                // print!("{:?}",cell.fore);
                let back = ansi::convert(cell.back, true);

                if cell.back.a < transparency_t {
                    if cell.fore.a < transparency_t {
                        s.push(characters[0]);
                    } else {
                        s.push_str(&fore.paint(char_.to_string()).to_string());
                    }
                } else {
                    s.push_str(&fore.on(back).paint(char_.to_string()).to_string());
                }
            } else {
                s.push(char_);
            };

            if (i + 1) % (grid.width()) as usize == 0 {
                s.push_str("\n");
            }
        }

        (
            s,
            if missing_char {
                charsets::CharsetWarnings::NotEnoughCharacters
            } else {
                charsets::CharsetWarnings::None
            },
        )
    }
}
