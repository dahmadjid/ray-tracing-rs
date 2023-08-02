mod vec3;
mod ray;
mod hittable;
mod scene;
mod window;
mod fonts;
mod camera;
mod mat3;
mod quat;
mod mat4;
mod vec4;

use hittable::Sphere;
use scene::{Scene, Object};
use vec3::Vec3;
use window::Window;
const ASPECT_RATIO: f64 = 16.0/9.0;

fn main() {
    let window =  Window{width: 1280, height: (1280. / ASPECT_RATIO) as u32, title: "Ray Tracer"};
    let mut scene = Scene::new(window.width, window.height);
    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0., 0., 0.), color: Vec3::new(1., 0., 0.,)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 1., center: Vec3::new(0., -1., 0.), color: Vec3::new(1., 0., 1.,)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -102.5, 0.), color: Vec3::new(0., 1., 0.,)}));
    window.render_loop(scene);

}
