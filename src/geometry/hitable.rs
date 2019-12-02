//Hitable
use crate::{BaseMaterial, BoundingBox, Float, Ray, Vec3};
use std::f64::consts::PI;
use std::rc::Rc;

pub struct HitRecord {
    pub t: Float,
    pub p: Option<Vec3>,
    pub normal: Option<Vec3>,
    pub material: Option<Rc<dyn BaseMaterial>>,
    pub u: Float,
    pub v: Float,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: None,
            normal: None,
            material: None,
            t: 0.0,
            u: 0.0,
            v: 0.0,
        }
    }
    pub fn copy(&self) -> HitRecord {
        HitRecord {
            p: match self.p {
                Some(_p) => Some(_p.clone()),
                _ => None,
            },
            normal: match self.normal {
                Some(_p) => Some(_p.clone()),
                _ => None,
            },
            material: match &self.material {
                Some(_p) => Some(_p.clone()),
                _ => None,
            },
            t: self.t,
            u: self.u,
            v: self.v,
        }
    }
}

pub struct Object {
    pub object: Box<dyn HittableObject>,
    pub material: Rc<dyn BaseMaterial>,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> BoundingBox;
}

impl Hittable for &[Rc<dyn Hittable>] {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hitted: bool = false;
        let mut closest: Float = t_max;
        for i in 0..self.len() {
            if self[i].hit(r, t_min, closest, &mut temp_record) {
                hitted = true;
                closest = (&temp_record).t;
                *rec = temp_record.copy();
            }
        }
        hitted
    }

    fn bounding_box(&self) -> BoundingBox {
        let first: Rc<dyn Hittable> = self[0].clone();
        let mut b_box = first.bounding_box();
        for i in 1..self.len() {
            b_box = BoundingBox::surrounding_box(&b_box, &self[i].bounding_box());
        }
        b_box
    }
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        let distance: Float = self.object.find_intersection(&r);
        if distance > t_min && distance < t_max {
            rec.t = distance;
            let point: Vec3 = r.point_at_length(distance);
            rec.normal = Some((&self.object).get_normal_at(&point));
            let (u, v) = (&self.object).get_uv(&point);
            rec.u = u;
            rec.v = v;
            rec.material = Some(self.material.clone());
            rec.p = Some(point);
            return true;
        }
        false
    }
    fn bounding_box(&self) -> BoundingBox {
        self.object.get_bounding_box()
    }
}

pub trait HittableObject {
    fn find_intersection(&self, r: &Ray) -> Float;
    fn get_bounding_box(&self) -> BoundingBox;
    fn get_normal_at(&self, v: &Vec3) -> Vec3;
    fn get_uv(&self, v: &Vec3) -> (Float, Float);
    fn to_string(&self) -> String;
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[derive(Copy, Clone)]
pub struct Sphere {
    radius: Float,
    center: Vec3,
}
impl Sphere {
    pub fn new(mat: Rc<dyn BaseMaterial>, radius: Float, center: Vec3) -> Object {
        let s = Box::new(Sphere { radius, center });
        Object {
            object: s,
            material: Rc::clone(&mat),
        }
    }
}

impl Object {
    pub fn to_string(&self) -> String {
        format!(
            "Object->\n {}\nMaterial->\n {}",
            self.object.to_string(),
            self.material.to_string()
        )
    }
    pub fn print(&self) {
        println!("{}", self.to_string())
    }
}

impl HittableObject for Sphere {
    fn to_string(&self) -> String {
        format!(
            "Sphere with radius {} and center: {}",
            self.radius,
            self.center.to_string()
        )
    }

    fn find_intersection(&self, r: &Ray) -> Float {
        let a: f64 = 1.0;
        let b: f64 = (2.0 * (&r.origin.get_x() - &self.center.get_x()) * r.direction.get_x())
            + (2.0 * (&r.origin.get_y() - &self.center.get_y()) * r.direction.get_y())
            + (2.0 * (&r.origin.get_z() - &self.center.get_z()) * r.direction.get_z());
        let c = (&r.origin.get_x() - &self.center.get_x()).powi(2)
            + (&r.origin.get_y() - &self.center.get_y()).powi(2)
            + (&r.origin.get_z() - &self.center.get_z()).powi(2)
            - (&self.radius * &self.radius);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let root_1 = (-1.0 * b - discriminant.sqrt()) / (2.0 * a);
            if root_1 > 0.0 {
                return root_1;
            }
            return (-1.0 * b + discriminant.sqrt()) / (2.0 * a);
        }
        -1.0
    }

    fn get_normal_at(&self, point: &Vec3) -> Vec3 {
        (point - &self.center).normalize()
    }

    fn get_uv(&self, point: &Vec3) -> (Float, Float) {
        let p = &(point - &self.center) / self.radius;
        let phi = p.get_z().atan2(p.get_x());
        let theta = p.get_y().asin();
        let u = 1.0 - (phi + PI) / (2.0 * PI);
        let v = (theta + PI / 2.0) / PI;
        (u, v)
    }

    fn get_bounding_box(&self) -> BoundingBox {
        let min = &self.center - &Vec3::new(self.radius, self.radius, self.radius);
        let max = &self.center + &Vec3::new(self.radius, self.radius, self.radius);
        BoundingBox::new(min, max)
    }
}
