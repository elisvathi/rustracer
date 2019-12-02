// Bounding box
use crate::{max, min, Vec3};
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl BoundingBox {
    pub fn zero() -> BoundingBox {
        BoundingBox {
            min: Vec3::zero(),
            max: Vec3::zero(),
        }
    }
    pub fn new(min: Vec3, max: Vec3) -> BoundingBox {
        BoundingBox { min, max }
    }
    pub fn surrounding_box(b1: &BoundingBox, b2: &BoundingBox) -> BoundingBox {
        let small = Vec3 {
            x: min(b1.min.x, b2.min.x),
            y: min(b1.min.y, b2.min.y),
            z: min(b1.min.z, b2.min.z),
        };
        let big = Vec3 {
            x: max(b1.max.x, b2.max.x),
            y: max(b1.max.y, b2.max.y),
            z: max(b1.max.z, b2.max.z),
        };
        return BoundingBox::new(small, big);
    }
}
