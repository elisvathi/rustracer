// Raytracer
use super::ray::*;
use super::vectors::*;
use crate::geometry::hitable::{HitRecord, Hittable};
use std::f64::MAX;
use std::rc::Rc;

pub fn get_color_simple(r: &Ray, world: Rc<dyn Hittable>) -> Vec3 {
    // get_sky_color(r)
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, MAX, &mut rec) {
        let emitted = rec
            .material
            .clone()
            .unwrap()
            .as_ref()
            .emitted(rec.u, rec.v, &rec.p.unwrap());
        emitted
    } else {
        return get_sky_color(r);
    }
}

pub fn get_color(r: &Ray, world: Rc<dyn Hittable>, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.001, MAX, &mut rec) {
        let mut scattered: Ray = Ray::zero();
        let mut v = Vec3::zero();
        let emitted = rec
            .material
            .clone()
            .unwrap()
            .as_ref()
            .emitted(rec.u, rec.v, &rec.p.unwrap());
        if depth < 50
            && rec
                .material
                .clone()
                .unwrap()
                .as_ref()
                .scatter(r, &rec, &mut v, &mut scattered)
        {
            return emitted + v * get_color(&scattered, world.clone(), depth + 1);
        }
        return get_sky_color(r);
    } else {
        return get_sky_color(r);
    }
}

fn get_sky_color(r: &Ray) -> Vec3 {
    let unit = r.direction.clone();
    let t = 0.5 * (unit.get_y() + 1.0);
    let val = Vec3::new(1.0, 1.0, 1.0) + (&Vec3::new(0.5, 0.7, 1.0) * t);
    val
}
