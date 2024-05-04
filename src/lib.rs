#[cfg(test)]
mod tests;

pub mod color {
    
    use std::ops;

    use cgmath::{MetricSpace, Vector3};


    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    }

    unsafe impl bytemuck::Zeroable for Color {}
    unsafe impl bytemuck::Pod for Color {}

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ColorU8 {
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    }

    unsafe impl bytemuck::Zeroable for ColorU8 {}
    unsafe impl bytemuck::Pod for ColorU8 {}

    impl Into<ColorU8> for Color {
        fn into(self) -> ColorU8 {
            ColorU8 {
                r: (self.r * 255.0).clamp(0.0, 255.0) as u8,
                g: (self.g * 255.0).clamp(0.0, 255.0) as u8,
                b: (self.b * 255.0).clamp(0.0, 255.0) as u8,
                a: (self.a * 255.0).clamp(0.0, 255.0) as u8,
            }
        }
    }

    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color{
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    impl Color {
        
        pub fn mag2(&self) -> f32{
            self.r * self.r +
            self.g * self.g +
            self.b * self.b +
            self.a * self.a
        }
        
        pub fn distance2(&self, other: &Self) -> f32 {
            (self.clone() - other.clone()).mag2()
        }

        pub fn lerp(self, other: Self, amt: f32) -> Self {
            self + ((other - self) * amt)
        }
    }
    

    impl ops::Sub<Color> for Color{
        type Output = Self;
    
        fn sub(self, rhs: Color) -> Self::Output {
            Self { 
                r: self.r - rhs.r,
                g: self.g - rhs.g,
                b: self.b - rhs.b,
                a: self.a - rhs.a,
            }
        }
    }

    impl ops::Add<Color> for Color{
        type Output = Self;
    
        fn add(self, rhs: Color) -> Self::Output {
            Self { 
                r: self.r + rhs.r,
                g: self.g + rhs.g,
                b: self.b + rhs.b,
                a: self.a + rhs.a,
            }
        }
    }

    impl ops::Div<f32> for Color{
        type Output = Self;
    
        fn div(self, rhs: f32) -> Self::Output {
            Self { 
                r: self.r / rhs,
                g: self.g / rhs,
                b: self.b / rhs,
                a: self.a / rhs,
            }
        }
    }

    impl ops::Mul<f32> for Color{
        type Output = Self;
    
        fn mul(self, rhs: f32) -> Self::Output {
            Self { 
                r: self.r * rhs,
                g: self.g * rhs,
                b: self.b * rhs,
                a: self.a * rhs,
            }
        }
    }

}

pub mod cell {
    pub const CELL_W: u32 = 2;
    pub const CELL_H: u32 = 4;
    pub type CellPixels = [Color; 8];


    use crate::color::{self, Color};

    /// Computes the minimum and maximum value of the cell using its distance.
    /// This is done to get the two colors with the most contrast in the cell.
    pub fn minmax_contrast(values: &CellPixels) -> (Color, Color) {
        // Traditional approach
        // Brute force by checking every point. This is O(N^2). However we only have 8 points to consider, so it is only 8x8 -> 64 comparison

        let mut current_max_dist: f32 = 0.0;
        let mut pair: [Color; 2] = [color::BLACK, color::BLACK];
        let avg = values.iter().fold(color::TRANSPARENT, |a,b| a+b.clone()) / values.len() as f32;
        for a in values {
            for b in values {
                if a == b {
                    continue;
                }
                let d = a.distance2(b);
                if d > current_max_dist {
                    current_max_dist = d;
                    pair[0] = a.clone();
                    pair[1] = b.clone();
                }
            }
        }
        let b = avg.lerp(pair[0], 0.5);
        let a = avg.lerp(pair[1], 0.5);
        
        

        return (a, b);
    }

    pub fn round_nearest_color(val: &Color, a: &Color, b: &Color) -> Color {
        
        let dist_a = val.distance2(a);
        let dist_b = val.distance2(b);
        if dist_a > dist_b {
            b.clone()
        } else {
            a.clone()
        }
    }

    pub fn round_cell_pixels(val: &CellPixels) -> CellPixels {
        let (a, b) = minmax_contrast(val);
        let mut copy = val.clone();
        for p_index in 0..copy.len() {
            let current = copy[p_index].clone();
            copy[p_index] = round_nearest_color(&current, &a, &b);
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
