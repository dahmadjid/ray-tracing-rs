mod vec3;
mod ray;
mod hittable;
mod scene;
mod window;
mod fonts;
mod camera;
mod mat3;

use hittable::Sphere;
use scene::{Scene, Object};
use vec3::Vec3;
use window::Window;
const ASPECT_RATIO: f64 = 1.0;

fn main() {
    let window =  Window{width: 800, height: (800. / ASPECT_RATIO) as u32, title: "Ray Tracer"};
    let mut scene = Scene::new(window.width, window.height);

    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0., 0., 0.)}));
    // scene.camera.horizontal_axis.rotate_yaw(180.0);
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(1., 0., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(0., 1., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -100.5, -1.)}));
    window.render_loop(scene);
    // scene.camera.vertical_axis.rotate_pitch(90.0);
    // scene.camera.position.y -= 1.0;
    // println!("vertical:      {}\nhorizontal:    {}\ncross_product: {}", scene.camera.vertical_axis, scene.camera.horizontal_axis, scene.camera.horizontal_axis.cross(&scene.camera.vertical_axis));
    // let u = 400. / window.width as f64 * 2.0 - 1.;
    // let v = 400. / window.height as f64 * 2.0 - 1.;
    // let ray = scene.camera.emit_ray(u, v);
    // let color = ray.color(&scene.objects, 0.01, f64::INFINITY, 1);

    // println!("{ray:?}")
}
