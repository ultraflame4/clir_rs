pub const CELL_W: u32 = 2;
pub const CELL_H: u32 = 4;
pub const CELL_LEN: u32 = CELL_W * CELL_H;
pub type CellPixels = [Color; CELL_LEN as usize];

use image::Rgba;

use crate::{color::Color, NearestOption};
pub struct CellGrid {
    pub cells: Vec<CellPixels>,
    width: u32,
    height: u32,
}

impl CellGrid {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn as_image_bytes(&self) -> (Vec<Color>, u32, u32) {
        let im_w = self.width() * CELL_W;
        let im_h = self.height() * CELL_H;

        let mut data: Vec<Color> = Vec::with_capacity((im_w * im_h) as usize);

        for y in 0..im_h {
            for x in 0..im_w {
                // Y position of the cell in the cell grid
                let cell_y = y / CELL_H;
                // X position of the cell in the cell grid
                let cell_x = x / CELL_W;
                // Calculate the index of the cell using the cell's xy pos
                let cell_index = cell_x + cell_y * self.width;

                // Calculate offset pos to index the correct pixel from the cell;
                let pixel_offset_y = y % CELL_H;
                let pixel_offset_x = x % CELL_W;
                let pixel_offset = pixel_offset_x + pixel_offset_y * CELL_W;

                // println!("Pixel XY {},{} | Cell index {} X {} Y {} | Pixel offset {} X {} Y {}",x,y , cell_index, cell_x, cell_y, pixel_offset, pixel_offset_x, pixel_offset_y);

                data.push(self.cells[cell_index as usize][pixel_offset as usize])
            }
        }

        (data, im_w, im_h)
    }

    pub fn save_as(&self, fp: &str) -> Result<(), image::ImageError> {
        let (bytes, im_w, im_h) = self.as_image_bytes();
        let im: image::ImageBuffer<Rgba<f32>, _> =
            image::ImageBuffer::from_raw(im_w, im_h, bytemuck::cast_vec(bytes)).unwrap();
        let dyn_im = image::DynamicImage::from(im);
        dyn_im.into_rgba8().save(fp)
    }
}

impl From<&image::Rgba32FImage> for CellGrid {
    fn from(img: &image::Rgba32FImage) -> Self {
        let cols = img.width() / CELL_W;
        let rows = img.height() / CELL_H;

        let mut cells_arrays: Vec<[[f32; 4]; 8]> = Vec::with_capacity(img.len());
        for y in 0..(rows) {
            for x in 0..(cols) {
                let cell_x = x * CELL_W;
                let cell_y = y * CELL_H;

                // Todo optimise this portion. When using test_image_2, ~30ms is used for looping. Another ~10ms is used for creating the cells
                #[rustfmt::skip]
                let cell_pixels = [
                    img.get_pixel(cell_x, cell_y + 0).0, img.get_pixel(cell_x + 1,cell_y + 0).0,
                    img.get_pixel(cell_x, cell_y + 1).0, img.get_pixel(cell_x + 1,cell_y + 1).0,
                    img.get_pixel(cell_x, cell_y + 2).0, img.get_pixel(cell_x + 1,cell_y + 2).0,
                    img.get_pixel(cell_x, cell_y + 3).0, img.get_pixel(cell_x + 1,cell_y + 3).0,
                ];

                cells_arrays.push(cell_pixels);
            }
        }
        let cells: Vec<CellPixels> = bytemuck::cast_vec(cells_arrays);
        let height: u32 = cells.len() as u32 / cols;
        Self {
            cells,
            width: cols,
            height,
        }
    }
}

/// Computes the lightest and darkest value of the cell using its distance.
/// This is done to get the two colors with the most contrast in the cell.
/// Returns a tuple of the lightest and darkest colors (lightest, darkest)
pub fn compute_minmax_contrast(values: &CellPixels) -> (Color, Color) {
    // Biggest distances from the brightest and darkest values
    let mut current_max_bright_dist: f32 = 0.0;
    let mut current_max_dark_dist: f32 = 0.0;
    let mut pair: [Color; 2] = [Color::WHITE, Color::BLACK];
    let avg = values.iter().fold(Color::TRANSPARENT, |a, b| a + b.clone()) / values.len() as f32;

    // Hybrid approach where we use find the values closest to the darkest & lightest possible values (transparent & white)
    // This in theory should give us the colors with the biggest contrast
    for i in 0..values.len() {
        let ele = values[i];
        let dark_dist = ele.distance2(&Color::TRANSPARENT);
        let bright_dist = ele.distance2(&Color::WHITE);

        if dark_dist > current_max_dark_dist {
            current_max_dark_dist = dark_dist;
            pair[0] = ele;
            continue;
        }

        if bright_dist > current_max_bright_dist {
            current_max_bright_dist = bright_dist;
            pair[1] = ele;
            continue;
        }
    }

    // A should be brightest, B should be darkest
    let a = avg.lerp(pair[0], 0.75); // lerp results to reduces sharpness and better consistency
    let b = avg.lerp(pair[1], 0.75);

    return (a, b);
}

/// Rounds & flattens the pixels colours in the cells to either a or b. \
/// Also creates a bitmask that shows which pixel got turned into a or b with the bits conversion as a=1 , b=0 \
/// 
/// <i>Note for developers: This uses u8 to store the bitmask. If CELL_W * CELL_H != 8, the bitmask value will be incorrect and likely have missing bits </i> \ 
/// 
/// Returns (Rounded CellPixels, bitmask)
pub fn cell_flatten_ab(val: &CellPixels, a: &Color, b: &Color) -> (CellPixels, u8) {
    let mut copy = val.clone();
    let mut mask: u8 = 0;
    for p_index in 0..copy.len() {
        let current = copy[p_index].clone();

        copy[p_index] = match Color::compare_nearest(&current, a, b) {
            NearestOption::A => {
                mask |= (2 as u8).pow(p_index as u32);
                a.clone()
            }
            NearestOption::B => b.clone(),
        };
    }
    (copy, mask)
}


/// Round the pixel values in the cells to their two light & dark colors determined by minmax_contrast
pub fn round_cells(cells: &mut Vec<CellPixels>) {
    for i in 0..cells.len() {
        let ele = &cells[i];
        let (a, b) = compute_minmax_contrast(ele);
        (cells[i], _) = cell_flatten_ab(ele, &a, &b);
    }
}

/// Round the pixel values in the cells to two colors (a & b)
pub fn round_cells_with_ab(cells: &mut Vec<CellPixels>, a: &Color, b: &Color) {
    for i in 0..cells.len() {
        let ele = &cells[i];
        (cells[i], _) = cell_flatten_ab(ele, &a, &b);
    }
}
