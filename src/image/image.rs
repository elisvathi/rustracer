// Image
use crate::{Float, Vec3};
use std::string::String;
pub struct Image {
    pub pixels: Vec<Vec<Vec3>>,
    pub width: usize,
    pub height: usize,
}
impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        let data = vec![vec![Vec3::zero(); width]; height];
        Image {
            pixels: data,
            width,
            height,
        }
    }
    pub fn put_pixel(&mut self, color: Vec3, x: usize, y: usize) {
        self.pixels[x][y] = color;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec3 {
        self.pixels[x][y].clone()
    }
    pub fn min_max(&self) -> (Float, Float) {
        use std::f64::{MAX, MIN};
        let (mut a, mut b) = (MAX, MIN);
        for i in 0..self.width {
            for j in 0..self.height {
                let pixel = &self.pixels[j][i];
                for k in 0..2 {
                    if pixel[k] < a {
                        a = pixel[k]
                    }
                    if pixel[k] > b {
                        b = pixel[k]
                    }
                }
            }
        }
        (a, b)
    }

    pub fn to_string(&self) -> String {
        let (a, b) = self.min_max();
        let mut s: String = String::new();
        let mut counter = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                let vec = &self.pixels[i][j];
                s.push_str(&vec.to_pixel_string(a, b));
                // if j % 3 == 0 {
                if counter == 2 {
                    counter = 0;
                    s.push_str("\n");
                } else {
                    counter += 1;
                    s.push_str("\t");
                }
            }
        }
        s
    }
}
