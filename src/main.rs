mod vec3;
mod ray;
mod image;
mod hittable;
mod scene;



use hittable::Sphere;
use image::Image;
use scene::{Scene, Camera, Viewport, Object};
use vec3::Vec3;


fn main() {
    let aspect_ratio = 16 as f64/9 as f64;
    // let mut img = Image::new(1024, (1024. / aspect_ratio) as u32);
    
    let viewport_height = 2.00f64;
    let viewport_width = viewport_height * aspect_ratio;

    let mut scene = Scene{
        camera: Camera{
            position: Vec3::new(0.5, 0.5, 0.5),
            focal_length: 1.00f64,
        },
        viewport: Viewport{
            height: viewport_height,
            width: viewport_width,
            horizontal_axis: Vec3::new(viewport_width, 0., 0.),
            vertical_axis: Vec3::new(0., viewport_height, 0.),
        },
        objects: vec![],
        image: Image { width: 1024, height: (1024. / aspect_ratio) as u32, data: vec![] }
    };

    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0.5, 0., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(1., 0., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(0., 1., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -100.5, -1.)}));
    scene.render();

}
