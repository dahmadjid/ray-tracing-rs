mod vec3;
mod ray;
mod image;
mod hittable;
mod scene;

use hittable::Sphere;
use image::Image;
use scene::{Scene, Camera, Object};
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16 as f64/9 as f64;
    // let mut img = Image::new(1024, (1024. / aspect_ratio) as u32);
    
    let viewport_height = 2.00f64;
    let viewport_width = viewport_height * aspect_ratio;

    let mut scene = Scene{

        camera: Camera::new(viewport_width, viewport_height, Vec3::new(0.5, 0.5, 0.5), 1.),
        objects: vec![],
        image: Image { width: 1024, height: (1024. / aspect_ratio) as u32, data: vec![] }
    };

    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0.5, 0., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(1., 0., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(0., 1., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -100.5, -1.)}));

    scene.render();
    
}
