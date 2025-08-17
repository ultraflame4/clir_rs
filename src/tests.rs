use std::{env::current_dir, fs};

use image::{io::Reader as ImageReader, Rgba};

use crate::{
    cell::{self, CellGrid, CellPixels},
    charsets,
    color::{self, Color},
    outputs::AsciiImageRenderer,
};

#[test]
fn print_hello_world() {
    print!("Hello world!");
}

#[test]
fn generate_cell_test() {
    // This test will create an 2x& cell, and find the two colors most distant from each other in each cell.
    println!("Current directory: {:?}", current_dir().unwrap());
    let img = ImageReader::open("./test_resource/test_image_2.png")
        .unwrap()
        .decode()
        .unwrap();

    use std::time::Instant;
    let now = Instant::now();
    let mut cells = CellGrid::from(&img.clone().into());
    let elapsed = now.elapsed();
    println!(
        "Image size ({}x{}) | Cells count: {} | Time taken: {:.2?}",
        img.width(),
        img.height(),
        cells.len(),
        elapsed
    );
}

#[test]
fn round_cell_test() {
    // This test will create an 2x& cell, and find the two colors most distant from each other in each cell.
    println!("Current directory: {:?}", current_dir().unwrap());
    let img = ImageReader::open("./test_resource/test_image_2.png")
        .unwrap()
        .decode()
        .unwrap();

    use std::time::Instant;

    let (mut cells, cell_generation_time) = {
        let now = Instant::now();
        let cells = CellGrid::from(&img.clone().into());
        (cells, now.elapsed())
    };

    let before_round = Instant::now();
    cell::round_cells(&mut cells.cells);
    let round_cell_time = before_round.elapsed();

    fs::create_dir("./test-outputs/");
    cells.save_as("./test-outputs/rounded_cells4.png").unwrap();

    println!("Image size ({}x{}) | Cells count: {} | Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?}", img.width(), img.height(), cells.len(), cell_generation_time, round_cell_time);
}

#[test]
fn round_cell_bw_test() {
    // This test will create an 2x& cell, and find the two colors most distant from each other in each cell.
    println!("Current directory: {:?}", current_dir().unwrap());
    let img = ImageReader::open("./test_resource/test_image_2.png")
        .unwrap()
        .decode()
        .unwrap();

    use std::time::Instant;

    let (mut cells, cell_generation_time) = {
        let now = Instant::now();
        let cells = CellGrid::from(&img.clone().into());
        (cells, now.elapsed())
    };

    let before_round = Instant::now();

    // Transparent is used instead of black for bw as the alpha channel is included as part of the comparisons Hence using transparency gives better results
    cell::round_cells_with_ab(&mut cells.cells, &Color::WHITE, &Color::TRANSPARENT);
    let round_cell_time = before_round.elapsed();

    fs::create_dir("./test-outputs/");
    cells
        .save_as("./test-outputs/bw_rounded_cells4.png")
        .unwrap();

    println!("Image size ({}x{}) | Cells count: {} | Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?}", img.width(), img.height(), cells.len(), cell_generation_time, round_cell_time);
}

#[test]
fn print_bw_test() {
    // This test will create an 2x& cell, and find the two colors most distant from each other in each cell.
    println!("Current directory: {:?}", current_dir().unwrap());
    let img = ImageReader::open("./test_resource/test_image.png")
        .unwrap()
        .decode()
        .unwrap();

    use std::time::Instant;
    
    let (mut cells, cell_generation_time) = {
        let now = Instant::now();
        let cells = CellGrid::from(&img.clone().into());
        (cells, now.elapsed())
    };
    // Transparent is used instead of black for bw as the alpha channel is included as part of the comparisons Hence using transparency gives better results
    let (computed, round_cell_time) = {
        let now = Instant::now();
        let computed = cells.compute_ab(&Color::WHITE, &Color::TRANSPARENT, false);
        (computed, now.elapsed())
    };

    let (s, string_time) = {
        let now = Instant::now();
        let (img, _) = AsciiImageRenderer::render(&computed, false, None, 0.25);
        (img, now.elapsed())
    };

    fs::create_dir("./test-outputs/");
    cell::round_cells_with_ab(&mut cells.cells, &Color::WHITE, &Color::TRANSPARENT);
    cells
        .save_as("./test-outputs/bw_print_rounded_cells.png")
        .unwrap();

    println!("{}", s);
    println!("Image size ({}x{}) | Cells count: {} | Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?} | String time: {:.2?}",
             img.width(), img.height(), cells.len(), cell_generation_time, round_cell_time, string_time);
}

#[test]
fn print_colored_test() {
    // This test will create an 2x& cell, and find the two colors most distant from each other in each cell.
    println!("Current directory: {:?}", current_dir().unwrap());
    let img = ImageReader::open("./test_resource/test_image.png")
        .unwrap()
        .decode()
        .unwrap();

    use std::time::Instant;

    let (mut cells, cell_generation_time) = {
        let now = Instant::now();
        let cells = CellGrid::from(&img.clone().into());
        (cells, now.elapsed())
    };

    // Transparent is used instead of black for bw as the alpha channel is included as part of the comparisons Hence using transparency gives better results
    let (computed, round_cell_time) = {
        let now = Instant::now();
        let computed = cells.compute(false);
        (computed, now.elapsed())
    };

    let (s, string_time) = {
        let now = Instant::now();
        let (img, _) = AsciiImageRenderer::render(&computed, true, None, 0.25);
        (img, now.elapsed())
    };

    fs::create_dir("./test-outputs/");
    cells
        .save_as("./test-outputs/print_colored_cells.png")
        .unwrap();

    cell::round_cells_with_ab(&mut cells.cells, &Color::WHITE, &Color::TRANSPARENT);
    cells
        .save_as("./test-outputs/bw_print_colored_cells.png")
        .unwrap();

    println!("{}", s);
    println!(
        "Image size ({}x{}) | Cells count: {} ({}x{})",
        img.width(),
        img.height(),
        cells.len(),
        cells.width(),
        cells.height()
    );
    println!(
        "Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?} | String time: {:.2?}",
        cell_generation_time, round_cell_time, string_time
    );
}

#[test]
fn braille_charset_bits_conv_test() {
    #[rustfmt::skip]
    let cell: CellPixels  = [
        Color::WHITE, Color::BLACK,
        Color::WHITE, Color::WHITE,
        Color::WHITE, Color::WHITE,
        Color::WHITE, Color::BLACK,
    ];

    let (_, bitmask) = cell::cell_flatten_ab(&cell, &Color::WHITE, &Color::BLACK);
    println!("Cell bitmask      : 0b{:0>8b}", bitmask);
    let converted = charsets::cell_bitmask_to_char_index(bitmask);
    println!("Converted bitmask : 0b{:0>8b}", converted);
    let c = charsets::BRAILLE.chars().nth(converted as usize).unwrap();
    println!("Braille character : {:?}", c);
    assert!(c == '⡷');
}
