// Scene
use crate::{BoundingBox, BoundingBoxNode, Camera, Float, HitRecord, Hittable, Ray, Vec3};
use std::rc::Rc;

pub struct Scene {
    pub objects: Vec<Rc<dyn Hittable>>,
    pub camera: Camera,
    pub accelerator: Option<Rc<BoundingBoxNode>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            camera: Camera::new(
                Vec3::new(0.0, 2.0, -5.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            accelerator: None,
        }
    }

    pub fn add_object(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj.clone());
    }

    pub fn get_mutable_camera(&mut self) -> &mut Camera {
        &mut self.camera
    }
    pub fn get_slice(&self) -> &[Rc<dyn Hittable>] {
        &self.objects[..]
    }

    fn get_mutable_slice(&mut self) -> &mut [Rc<dyn Hittable>] {
        &mut self.objects[..]
    }

    pub fn build_accelerator(&mut self) {
        self.accelerator = Some(Rc::new(BoundingBoxNode::new(&mut self.get_mutable_slice())));
    }
}
impl Hittable for Scene {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        // self.get_slice().hit(r, t_min, t_max, rec)
        self.accelerator.as_ref().unwrap().hit(r, t_min, t_max, rec)
    }
    fn bounding_box(&self) -> BoundingBox {
        // self.get_slice().bounding_box()
        self.accelerator.as_ref().unwrap().bounding_box()
    }
}
