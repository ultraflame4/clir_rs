use std::{env::current_dir, fs};

use image::{io::Reader as ImageReader, Rgba};

use crate::{cell::{self, CellGrid}, color::{self, Color}};

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

    let before_cell = Instant::now();
    let mut cells = CellGrid::from(&img.clone().into());
    let cell_generation_time = before_cell.elapsed();

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

    let before_cell = Instant::now();
    let mut cells = CellGrid::from(&img.clone().into());
    let cell_generation_time = before_cell.elapsed();

    let before_round = Instant::now();

    // Transparent is used instead of black for bw as the alpha channel is included as part of the comparisons Hence using transparency gives better results
    cell::round_cells_with_ab(&mut cells.cells, &Color::WHITE, &Color::TRANSPARENT);
    let round_cell_time = before_round.elapsed();

    
    fs::create_dir("./test-outputs/");
    cells.save_as("./test-outputs/bw_rounded_cells4.png").unwrap();

    println!("Image size ({}x{}) | Cells count: {} | Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?}", img.width(), img.height(), cells.len(), cell_generation_time, round_cell_time);
}
