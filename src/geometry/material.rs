// Materials
use crate::{Float, Texture, Vec3, Ray, HitRecord, ConstantTexture};
use std::rc::Rc;

pub struct Material {
    pub albedo: Rc<dyn Texture>,
}

pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

pub struct Metal {
    pub albedo: Rc<dyn Texture>,
    pub fuzz: Float,
}

pub trait BaseMaterial {
    fn scatter(
        &self,
        rin: &Ray,
        rec: &HitRecord,
        attentuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        Vec3::zero()
    }
    fn print(&self) {
        println!("{}", self.to_string());
    }
    fn to_string(&self) -> String;
}
impl DiffuseLight {
    pub fn new(vec: Vec3) -> DiffuseLight {
        let text = ConstantTexture { color: vec };
        DiffuseLight {
            emit: Rc::new(text),
        }
    }
}
impl Material {
    pub fn new(vec: Vec3) -> Material {
        let text = ConstantTexture { color: vec };
        Material {
            albedo: Rc::new(text),
        }
    }
}
impl Metal {
    pub fn new(albedo: Rc<dyn Texture>, fuzz: Float) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl BaseMaterial for Material {
    fn to_string(&self) -> String {
        format!("Simple material with color {}", self.albedo.to_string())
    }

    fn scatter(
        &self,
        _rin: &Ray,
        rec: &HitRecord,
        attentuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p.unwrap() + rec.normal.unwrap() + Vec3::random_unit();
        scattered.origin = rec.p.unwrap().clone();
        scattered.direction = &target - &rec.p.unwrap();
        let col = self.albedo.as_ref().value(rec.u, rec.v, &rec.p.unwrap());
        *attentuation = col;
        true
    }

    fn emitted(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        self.albedo.as_ref().value(u, v, p)
    }
}
impl BaseMaterial for Metal {
    fn to_string(&self) -> String {
        format!(
            "Metal with albedo {} and fuzziness {}",
            self.albedo.to_string(),
            self.fuzz
        )
    }

    fn scatter(
        &self,
        rin: &Ray,
        rec: &HitRecord,
        _attentuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = rin.direction.reflect(&rec.normal.unwrap());
        scattered.origin = rec.p.unwrap().clone();
        scattered.direction = reflected + (&Vec3::random_unit() * self.fuzz);
        scattered.time = rin.time;
        *_attentuation = self
            .albedo
            .as_ref()
            .value(rec.u, rec.v, &rec.normal.unwrap());
        scattered.direction.dot(&rec.normal.unwrap()) > 0.0
    }
}
impl BaseMaterial for DiffuseLight {
    fn to_string(&self) -> String {
        format!(
            "Diffuse Light material with color {}",
            self.emit.to_string()
        )
    }

    fn scatter(
        &self,
        _rin: &Ray,
        _rec: &HitRecord,
        _attentuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, u: Float, v: Float, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
