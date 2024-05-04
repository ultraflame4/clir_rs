#[cfg(test)]
mod tests;

pub mod color;



pub mod cell {
    pub const CELL_W: u32 = 2;
    pub const CELL_H: u32 = 4;
    pub type CellPixels = [Color; 8];


    use crate::color::{self, Color};

    /// Computes the minimum and maximum value of the cell using its distance.
    /// This is done to get the two colors with the most contrast in the cell.
    pub fn minmax_contrast(values: &CellPixels) -> (Color, Color) {
        // Traditional approach
        // Not used anymore
        // B̶r̶u̶t̶e̶ f̶o̶r̶c̶e̶ b̶y̶ c̶h̶e̶c̶k̶i̶n̶g̶ e̶v̶e̶r̶y̶ p̶o̶i̶n̶t̶. T̶h̶i̶s̶ i̶s̶ O̶(̶N̶^̶2̶)̶. H̶o̶w̶e̶v̶e̶r̶ w̶e̶ o̶n̶l̶y̶ h̶a̶v̶e̶ 8̶ p̶o̶i̶n̶t̶s̶ t̶o̶ c̶o̶n̶s̶i̶d̶e̶r̶, s̶o̶ i̶t̶ i̶s̶ o̶n̶l̶y̶ 8̶x̶8̶ -̶> 6̶4̶ c̶o̶m̶p̶a̶r̶i̶s̶o̶n̶

 
        // Biggest distances from the brightest and darkest values
        let mut current_max_bright_dist: f32 = 0.0;
        let mut current_max_dark_dist: f32 = 0.0;
        let mut pair: [Color; 2] = [color::WHITE, color::BLACK];
        let avg = values.iter().fold(color::TRANSPARENT, |a,b| a+b.clone()) / values.len() as f32;
        
        // Hybrid approach where we use find the values closest to the darkest & lightest possible values (transparent & white)
        // This in theory should give us the colors with the biggest contrast
        for i in 0..values.len() {
            let ele = values[i];
            let dark_dist = ele.distance2(&color::TRANSPARENT);
            let bright_dist = ele.distance2(&color::WHITE);

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

    pub enum NearestOption{
        A,
        B
    }

    /// Compares the val against a & b.
    /// Returns 0 if a is closer , 1 if b is closer
    pub fn compare_nearest_color(val: &Color, a: &Color, b: &Color) -> NearestOption {
        
        let dist_a = val.distance2(a);
        let dist_b = val.distance2(b);
        if dist_a > dist_b {
            NearestOption::B
        } else {
            NearestOption::A
        }
    }
    
    pub fn round_cell_pixels(val: &CellPixels) -> CellPixels {
        let (a, b) = minmax_contrast(val);
        let mut copy = val.clone();
        for p_index in 0..copy.len() {
            let current = copy[p_index].clone();
            copy[p_index] = match compare_nearest_color(&current, &a, &b) {
                NearestOption::A => a.clone(),
                NearestOption::B => b.clone(),
            };
            // round_nearest_color(&copy[p_index], &a, &b);
            // copy[p_index] = if p_index % 3 == 0 { a } else { b } .clone()
        }

        copy
    }
    
    pub fn generate_cells(img: &image::Rgba32FImage) -> (Vec<CellPixels>, u32) {
        let cols = img.width() / CELL_W;
        let rows = img.height() / CELL_H;
        
        let mut cells_arrays: Vec<[[f32; 4]; 8]> = Vec::with_capacity(img.len());
        for y in 0..(rows) {
            for x in 0..(cols) {
                let cell_x = x * CELL_W;
                let cell_y = y * CELL_H;

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

        (cells, cols)
    }

    pub fn round_cells(cells: &mut Vec<CellPixels>) {
        for i in 0..cells.len() {
            let ele = &cells[i];
 
            cells[i] = round_cell_pixels(ele);
        }
    }

    pub fn cells_to_image(cells: &Vec<CellPixels>, cols: u32) -> (Vec<Color>, u32, u32) {
        let rows = cells.len() as u32 / cols;
        let im_w = cols * CELL_W;
        let im_h = rows * CELL_H;

        let mut data: Vec<Color> = Vec::with_capacity((im_w * im_h) as usize);

        for y in 0..im_h {
            for x in 0..im_w {
                // Y position of the cell in the cell grid
                let cell_y = y / CELL_H;
                // X position of the cell in the cell grid
                let cell_x = x / CELL_W;
                // Calculate the index of the cell using the cell's xy pos
                let cell_index = cell_x + cell_y * cols;

                // Calculate offset pos to index the correct pixel from the cell;
                let pixel_offset_y = y % CELL_H;
                let pixel_offset_x = x % CELL_W;
                let pixel_offset = pixel_offset_x + pixel_offset_y * CELL_W;

                // println!("Pixel XY {},{} | Cell index {} X {} Y {} | Pixel offset {} X {} Y {}",x,y , cell_index, cell_x, cell_y, pixel_offset, pixel_offset_x, pixel_offset_y);

                data.push(cells[cell_index as usize][pixel_offset as usize])
            }
        }

        (data, im_w, im_h)
    }

    
}
