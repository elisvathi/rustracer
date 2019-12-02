// Main File
use rustracer::{
    gen_random_sphere, get_color, get_metal, Float, ImageBuilder, PixelData, Scene, Sphere,
    ThreadPool, Vec3,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};

fn get_final_color(
    i: usize,
    j: usize,
    width: usize,
    height: usize,
    scene: Arc<Scene>,
    builder: Rc<RefCell<ImageBuilder>>,
    passes: usize,
    pool: &ThreadPool,
) {
    let rap_w: Float = 1.0 / width as Float;
    let rap_h: Float = 1.0 / width as Float;
    for _ in 0..passes {
        pool.execute(|| {
            let rand_x: Float = rand::random::<Float>() * rap_w - rap_w / 2.0;
            let rand_y: Float = rand::random::<Float>() * rap_h - rap_h / 2.0;
            let ray = &scene.camera.get_origin_ray(
                (width as Float) / (height as Float),
                i as Float / width as Float + rand_x,
                j as Float / height as Float + rand_y,
            );
            let color = get_color(&ray, scene.clone(), 0);
            PixelData {
                pixel: color,
                x: i,
                y: j,
            }
        });
    }
}

fn main() {
    let width = 640;
    let height = 480;
    let sphere_plane = Arc::new(Sphere::new(get_metal(), 100.0, Vec3::new(0.0, -100.0, 5.0)));
    let mut scene = Scene::new();
    scene.add_object(sphere_plane);
    for _ in 0..10 {
        let obj = gen_random_sphere();
        scene.add_object(obj);
    }
    scene.build_accelerator();
    let scene = Arc::new(scene);
    let passes: usize = 100;
    let image_builder = Rc::new(RefCell::new(ImageBuilder::new(width, height, passes)));
    let (sender, receiver) = mpsc::channel::<PixelData>();
    let thread_pool = ThreadPool::new(10, Arc::new(Mutex::new(sender)));
    for i in 0..width {
        for j in 0..height {
            get_final_color(
                i,
                j,
                width,
                height,
                scene.clone(),
                image_builder.clone(),
                passes,
                &thread_pool,
            );
        }
    }
    let img = image_builder.borrow().to_image();
    println!("P3 \n{} {}\n255\n{}", width, height, img.to_string());
}
