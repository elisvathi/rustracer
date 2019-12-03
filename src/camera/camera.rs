// Camera
use crate::{Float, Ray, Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub down: Vec3,
    pub right: Vec3,
    pub aperture: Float,
    pub angle: Float,
    pub focus_distance: Float,
}
impl Camera {
    pub fn new(position: Vec3, lookat: Vec3, up: Vec3) -> Camera {
        let direction: Vec3 = (lookat - position).normalize();
        let right = up.cross(&direction).normalize();
        Camera {
            position,
            direction: direction.clone(),
            aperture: 0.0,
            right: right.clone(),
            down: right.cross(&direction),
            angle: 55f64.to_radians(),
            focus_distance: 1.0,
        }
    }

    pub fn get_up(&self) -> Vec3 {
        self.down.inverse()
    }
    pub fn to_string(&self) -> String {
        format!(
            "Camera:\n Position: {}\n Direction: {}\n Down: {}\n Right: {}",
            self.position.to_string(),
            self.direction.to_string(),
            self.down.to_string(),
            self.right.to_string()
        )
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn get_down(&self) -> Vec3 {
        self.down
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn get_aperture(&self) -> Float {
        self.aperture
    }

    pub fn set_aperture(&mut self, aperture: Float) {
        self.aperture = aperture;
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, value: f64) {
        self.angle = value.to_radians()
    }

    pub fn get_origin_ray(&self, aspect_ratio: Float, x: Float, y: Float) -> Ray {
        let rand_dir = &Vec3::random_in_disk() * self.aperture;
        let offset: Vec3 = (&self.right * rand_dir.x) + (&self.down * rand_dir.y);
        let dir = self.get_origin_direction(aspect_ratio, x, y, offset);
        let r: Ray = Ray::new(self.position + offset, dir, 0.0);
        // r.direction.print();
        r
    }

    pub fn get_origin_direction(
        &self,
        aspect_ratio: Float,
        u: Float,
        v: Float,
        offset: Vec3,
    ) -> Vec3 {
        let mut dx: Float = 0.0;
        let mut dy: Float = 0.0;
        let u_right = self.upper_right(aspect_ratio, &mut dx, &mut dy);
        let dir = u_right + (self.right * (u * dx)) + (&self.down * (v * dy));
        let v = (&dir - &self.position) - offset;
        v.normalize()
    }

    pub fn upper_right(&self, aspect_ratio: Float, dx: &mut Float, dy: &mut Float) -> Vec3 {
        *dx = self.screen_width(aspect_ratio);
        *dy = self.screen_height();
        let result = self.position + (&self.direction.normalize() * self.focus_distance)
            - (&(&self.right * (*dx / 2.0)) * self.focus_distance)
            - (&(&self.down * (*dy / 2.0)) * self.focus_distance);
        result
    }

    pub fn screen_width(&self, aspect_ratio: Float) -> Float {
        return self.screen_height() * aspect_ratio;
    }

    pub fn screen_height(&self) -> Float {
        let theta = self.angle;
        return 2.0 * (theta / 2.0).tan();
    }
}
