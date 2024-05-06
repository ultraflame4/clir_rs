use std::ops;

use crate::NearestOption;

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
pub struct RGBColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

unsafe impl bytemuck::Zeroable for RGBColorU8 {}
unsafe impl bytemuck::Pod for RGBColorU8 {}

impl Into<RGBColorU8> for Color {
    fn into(self) -> RGBColorU8 {
        RGBColorU8 {
            r: (self.r * self.a * 255.0).round().clamp(0.0, 255.0) as u8,
            g: (self.g * self.a * 255.0).round().clamp(0.0, 255.0) as u8,
            b: (self.b * self.a * 255.0).round().clamp(0.0, 255.0) as u8,
        }
    }
}


impl RGBColorU8{
    pub fn u32(&self) -> u32{
        let mut x:u32 = 0;
        x |= (self.r as u32) << 24;
        x |= (self.g as u32) << 16;
        x |= (self.b as u32) << 8;
        x |= 255;
        x
    }
}


impl Color {
    
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
    
    /// Compares the val against a & b.
    /// Returns NearestOption::A if a is closer , NearestOption::B if b is closer
    pub fn compare_nearest(val: &Color, a: &Color, b: &Color) -> NearestOption {
        let dist_a = val.distance2(a);
        let dist_b = val.distance2(b);
        if dist_a > dist_b {
            NearestOption::B
        } else {
            NearestOption::A
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
