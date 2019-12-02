// Accelerator
use crate::{BoundingBox, Hittable, Float, Ray, HitRecord};
use rand;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BoundingBoxNode {
    pub bbox: BoundingBox,
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
}

pub fn sort_hitables(first: &dyn Hittable, second: &dyn Hittable, axis: u8) -> Ordering {
    let first_box: BoundingBox = first.bounding_box();
    let second_box: BoundingBox = second.bounding_box();
    if (&first_box.min)[axis] - (&second_box.min)[axis] < 0.0 {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}

impl Hittable for BoundingBoxNode {
    fn hit(&self, r: &Ray, t_min: Float, t_max: Float, rec: &mut HitRecord) -> bool {
        // false
        let mut left_rec = HitRecord::new();
        let mut right_rec = HitRecord::new();
        let left_hitted = self
            .left
            .as_ref()
            .unwrap()
            .as_ref()
            .hit(r, t_min, t_max, &mut left_rec);
        let right_hitted =
            self.right
                .as_ref()
                .unwrap()
                .as_ref()
                .hit(r, t_min, t_max, &mut right_rec);
        if left_hitted && right_hitted {
            if left_rec.t < right_rec.t {
                *rec = left_rec.copy();
            } else {
                *rec = right_rec.copy();
            }
            return true;
        } else if left_hitted {
            *rec = left_rec.copy();
            return true;
        } else if right_hitted {
            *rec = right_rec.copy();
            return true;
        } else {
            return false;
        }
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::surrounding_box(
            &self.left.clone().unwrap().as_ref().bounding_box(),
            &self.right.clone().unwrap().as_ref().bounding_box(),
        )
    }
}
impl BoundingBoxNode {
    pub fn zero() -> BoundingBoxNode {
        BoundingBoxNode {
            bbox: BoundingBox::zero(),
            left: None,
            right: None,
        }
    }

    pub fn new(hitables: &mut [Rc<dyn Hittable>]) -> BoundingBoxNode {
        let axis: u8 = 2 * rand::random::<Float>() as u8;
        hitables.sort_by(|a, b| sort_hitables(a.as_ref(), b.as_ref(), axis));
        let count = hitables.len();
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if count == 1 {
            left = hitables[0].clone();
            right = hitables[0].clone();
        } else if count == 2 {
            left = hitables[0].clone();
            right = hitables[1].clone();
        } else {
            let half: usize = count / 2;
            left = Rc::new(BoundingBoxNode::new(&mut hitables[0..half]));
            right = Rc::new(BoundingBoxNode::new(&mut hitables[half..]));
        }
        BoundingBoxNode {
            left: Some(left.clone()),
            right: Some(right.clone()),
            bbox: BoundingBox::surrounding_box(
                &left.as_ref().bounding_box(),
                &right.as_ref().bounding_box(),
            ),
        }
    }
}
