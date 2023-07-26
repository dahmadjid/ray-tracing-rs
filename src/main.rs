mod vec3;
mod ray;
mod hittable;
mod scene;
mod window;
mod fonts;

use hittable::Sphere;
use scene::{Scene, Object};
use vec3::Vec3;
use window::Window;

const ASPECT_RATIO: f64 = 16 as f64/9 as f64;

fn main() {
    
    let window =  Window{width: 1024, height: (1024. / ASPECT_RATIO) as u32, title: "Ray Tracer"};
    let mut scene = Scene::new(window.width, window.height);

    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0., 0., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(1., 0., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(0., 1., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -100.5, -1.)}));
    window.render_loop(scene);
}
