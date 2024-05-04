use std::env::current_dir;

use image::{io::Reader as ImageReader, Rgba};

use crate::cell;

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
    let (cells, _) = cell::generate_cells(&img.clone().into());
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
    let (mut cells, cols) = cell::generate_cells(&img.clone().into());
    let cell_generation_time = before_cell.elapsed();

    let before_round = Instant::now();
    cell::round_cells(&mut cells);
    let round_cell_time = before_round.elapsed();

    let (data, im_w, im_h) = cell::cells_to_image(&cells, cols);
    let bytes_vec: Vec<f32> = bytemuck::cast_vec(data);
    
    println!(
        "Byte count: {:?}; Image Size {:?} x {:?} = {:?}",
        bytes_vec.len(), im_w, im_h, im_w * im_h
    );
    let im: image::ImageBuffer<Rgba<f32>, _> =
        image::ImageBuffer::from_raw(im_w, im_h, bytes_vec.clone()).unwrap();
    let dyn_im = image::DynamicImage::from(im);
    dyn_im.into_rgba8().save("./test-outputs/rounded_cells4.png").unwrap();

    println!("Image size ({}x{}) | Cells count: {} | Cell Generate Time: {:.2?} | Round Cell Pixels time: {:.2?}", img.width(), img.height(), cells.len(), cell_generation_time, round_cell_time);
}
