// Main File
use rustracer::{Image, Float, Vec3, Scene, Sphere, get_metal, gen_random_sphere, get_color};
use std::rc::Rc;

fn get_final_color(i: usize, j: usize, width: usize, height: usize, scene: Rc<Scene>) -> Vec3 {
    let mut result = Vec3::zero();
    let n = 1;
    let rap_w: Float = 1.0 / width as Float;
    let rap_h: Float = 1.0 / width as Float;
    for _ in 0..n {
        let rand_x: Float = rand::random::<Float>() * rap_w - rap_w / 2.0;
        let rand_y: Float = rand::random::<Float>() * rap_h - rap_h / 2.0;
        let ray = &scene.camera.get_origin_ray(
            (width as Float) / (height as Float),
            i as Float / width as Float + rand_x,
            j as Float / height as Float + rand_y,
        );
        let color = get_color(&ray, scene.clone(), 0);
        result = result + color;
    }
    result = &result / (n as Float);
    result
}

fn main() {
    let width = 640;
    let height = 480;
    let sphere_plane = Rc::new(Sphere::new(get_metal(), 100.0, Vec3::new(0.0, -100.0, 5.0)));
    let mut scene = Scene::new();
    scene.add_object(sphere_plane);
    for _ in 0..140 {
        let obj = gen_random_sphere();
        scene.add_object(obj);
    }
    scene.build_accelerator();
    let scene = Rc::new(scene);
    let mut img = Image::new(width, height);
    for i in 0..width {
        for j in 0..height {
            img.put_pixel(get_final_color(i, j, width, height, scene.clone()), j, i);
        }
    }
    println!("P3 \n{} {}\n255\n{}", width, height, img.to_string());
}
