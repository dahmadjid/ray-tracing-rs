mod vec3;
mod ray;
mod image;
mod hittable;
mod scene;
mod window;

use hittable::Sphere;
use image::Image;
use scene::{Scene, Camera, Object};
use vec3::Vec3;
use window::Window;

use winit::event::VirtualKeyCode;
use winit::event_loop::ControlFlow;

fn main() {
    let aspect_ratio = 16 as f64/9 as f64;
    // let mut img = Image::new(1024, (1024. / aspect_ratio) as u32);
    
    let viewport_height = 2.00f64;
    let viewport_width = viewport_height * aspect_ratio;

    let mut scene = Scene{

        camera: Camera::new(viewport_width, viewport_height, Vec3::new(0., 0., 0.), 1.),
        objects: vec![],
        image: Image { width: 600, height: (600. / aspect_ratio) as u32, data: vec![] }
    };

    scene.objects.push(Object::Sphere(Sphere{radius: 0.5, center: Vec3::new(0., 0., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(1., 0., -1.)}));
    // scene.objects.push(Object::Sphere(Sphere{radius: 0.2, center: Vec3::new(0., 1., -1.)}));
    scene.objects.push(Object::Sphere(Sphere{radius: 100., center: Vec3::new(0., -100.5, -1.)}));

    let window = Window{width: scene.image.width, height: scene.image.height, title: "Ray Tracer"};
    window.render_loop(move || scene.render(), |input, control_flow| {

        if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
            *control_flow = ControlFlow::Exit;
            return;
        }

        if input.key_pressed(VirtualKeyCode::Up) {
        }

        if input.key_pressed(VirtualKeyCode::Down) {
        }

        // Resize the window
        // if let Some(size) = input.window_resized() {
        //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
        //         log_error("pixels.resize_surface", err);
        //         *control_flow = ControlFlow::Exit;
        //         return;
        //     }
        // }
    
    })
}
