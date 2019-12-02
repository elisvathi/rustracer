use crate::{
    BaseMaterial, ConstantTexture, DiffuseLight, Float, Hittable, Material, Metal, Sphere, Vec3,
};
use std::rc::Rc;

pub fn basic_scene() {}
pub fn gen_random_material() -> Rc<dyn BaseMaterial> {
    let color = Vec3::random_unit();
    let value: Float = rand::random();
    let rap: Float = 1.0 / 3.0;
    if value < rap {
        return Rc::new(Material::new(color));
    } else if value > rap * 8.0 {
        return Rc::new(DiffuseLight::new(color * 100.0));
    }
    let fuzz = rand::random();
    Rc::new(Metal::new(
        Rc::new(ConstantTexture {
            // color: Vec3::new(1.0, 1.0, 1.0),
            color,
        }),
        fuzz,
    ))
}

pub fn gen_random_sphere() -> Rc<dyn Hittable> {
    let radius = rand::random::<Float>() * 0.3;
    let mut position: Vec3 = Vec3::random_unit();
    position = position * 5.0;
    position.y = radius;
    let material = gen_random_material();
    Rc::new(Sphere::new(material, radius, position))
}

pub fn get_metal() -> Rc<Metal> {
    Rc::new(Metal::new(
        Rc::new(ConstantTexture {
            color: Vec3::new(1.0, 1.0, 1.0),
        }),
        0.05,
    ))
}
