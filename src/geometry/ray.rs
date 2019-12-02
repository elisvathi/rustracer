use super::vectors::*;

pub struct Ray {
    pub bounces: i32,
    pub time: Float,
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn zero() -> Ray {
        Ray {
            bounces: 0,
            time: 0.0,
            origin: Vec3::zero(),
            direction: Vec3::zero(),
        }
    }

    pub fn new(origin: Vec3, direction: Vec3, time: Float) -> Ray {
        Ray {
            bounces: 0,
            time,
            origin,
            direction,
        }
    }

    pub fn set_time(&mut self, time: Float) {
        self.time = time;
    }
    pub fn reflected_ray(&self, origin: &Vec3, normal: &Vec3, fuzz: Float) -> Ray {
        let mut dir: Vec3 = (&(self.direction)).reflect(normal);
        if fuzz > 0.0 {
            dir = &(dir) + &(&Vec3::random_unit() * fuzz);
        }
        let mut r = Ray::new(origin.clone(), dir.clone(), 0.0);
        r.bounces = self.bounces + 1;
        r
    }
    pub fn point_at_length(&self, length: Float) -> Vec3 {
        &self.origin + &(&self.direction * length)
    }
    pub fn to_string(&self) -> String {
        format!(
            "Ray\n Start: {}\n Direction: {}",
            self.origin.to_string(),
            self.direction.to_string()
        )
    }
    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}
