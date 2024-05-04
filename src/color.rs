use std::ops;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
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


    pub fn cross_product(&self, p1: &Self, p2: &Self) -> Self {
        Self {
            r: (p1.g - self.g) * (p2.b - self.b) - (p1.b - self.b) * (p2.g - self.g),
            g: (p1.b - self.b) * (p2.r - self.r) - (p1.r - self.r) * (p2.b - self.b),
            b: (p1.r - self.r) * (p2.g - self.g) - (p1.g - self.g) * (p2.r - self.r),
            a: 0.0,
        }
    }

    pub fn dot_product(&self, other: &Self) -> f32 {
        self.r * other.r + self.g * other.g + self.b * other.b + self.a * other.a
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
