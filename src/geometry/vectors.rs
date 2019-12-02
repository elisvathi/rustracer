// Vectors
// extern crate rand;
use crate::{map, to_color};
use rand;
use std::f64::consts::PI;
use std::ops::Index;
use std::ops::{Add, Div, Mul, Sub};
pub type Float = f64;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn to_string(&self) -> String {
        format!("Vector (x: {}, y: {}, z: {})", self.x, self.y, self.z)
    }

    pub fn to_pixel_string(&self, a: Float, b: Float) -> String {
        let r = to_color(self.x, a, b);
        let g = to_color(self.y, a, b);
        let b = to_color(self.z, a, b);
        format!("{} {} {}", r, g, b)
    }

    pub fn add(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn sub(&mut self, other: &Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    pub fn mult(&mut self, other: &Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }

    pub fn div(&mut self, other: &Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Vec3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn get_x(&self) -> Float {
        self.x
    }

    pub fn get_y(&self) -> Float {
        self.y
    }

    pub fn get_z(&self) -> Float {
        self.z
    }

    pub fn set_x(&mut self, x: Float) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: Float) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: Float) {
        self.z = z;
    }

    pub fn sum_squares(&self) -> Float {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn angle_between(&self, other: &Vec3) -> Float {
        (self.dot(other) / (self.sum_squares() * other.sum_squares())).acos()
    }

    pub fn inverse(&self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn sqrt(&self) -> Vec3 {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        (&(normal * (normal.dot(self) * 2.0f64)) - self).inverse()
    }

    pub fn rotate_x(&self, angle: Float) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_y(&self, angle: Float) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() + self.z * angle.sin(),
            y: self.y,
            z: self.x * -1.0 * angle.sin() + self.z * angle.cos(),
        }
    }

    pub fn rotate_z(&self, angle: Float) -> Vec3 {
        Vec3 {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z,
        }
    }

    pub fn random_unit() -> Vec3 {
        let mut v = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        v = v.rotate_x(rand::random::<Float>() * PI * 2.0);
        v = v.rotate_y(rand::random::<Float>() * PI * 2.0);
        v = v.rotate_z(rand::random::<Float>() * PI * 2.0);
        v
    }

    pub fn random_in_disk() -> Vec3 {
        let mut v: Vec3;
        loop {
            let x = rand::random::<Float>();
            let y = rand::random::<Float>();
            let z = 0.0;
            v = &(&Vec3::new(x, y, z) * 2.0) - &Vec3::new(1.0, 1.0, 0.0);
            if v.dot(&v) <= 1.0 {
                break;
            }
        }
        v
    }

    pub fn clone(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn normalize(&self) -> Vec3 {
        self.clone_direction(1.0)
    }

    pub fn magnitude(&self) -> Float {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn clone_direction(&self, value: Float) -> Vec3 {
        return self * (value / self.magnitude());
    }
    pub fn squash(&self, maximum: Float) -> Vec3 {
        Vec3 {
            x: map(self.x, 0.0, maximum, 0.0, 1.0),
            y: map(self.x, 0.0, maximum, 0.0, 1.0),
            z: map(self.x, 0.0, maximum, 0.0, 1.0),
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div for &Vec3 {
    type Output = Vec3;
    fn div(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<Float> for &Vec3 {
    type Output = Vec3;
    fn div(self, value: Float) -> Vec3 {
        Vec3 {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, value: Float) -> Vec3 {
        Vec3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl Mul<Float> for &Vec3 {
    type Output = Vec3;
    fn mul(self, value: Float) -> Vec3 {
        Vec3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}
impl Index<u8> for &Vec3 {
    type Output = Float;
    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &0.0,
        }
    }
}
