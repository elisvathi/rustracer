// Texture
use super::vectors::*;

pub trait Texture {
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Vec3;
    fn to_string(&self) -> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

pub struct ConstantTexture {
    pub color: Vec3,
}
impl ConstantTexture {
    pub fn new(col: Vec3) -> ConstantTexture {
        ConstantTexture { color: col }
    }
}
impl Texture for ConstantTexture {
    fn value(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        self.color.clone()
    }
    fn to_string(&self) -> String {
        format!("Constant texture with color {}", self.color.to_string())
    }
}
