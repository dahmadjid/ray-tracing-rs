use std::time::Instant;
use crate::{hittable::*, vec3::Vec3, fonts::{render_string, RasterizedCharacter, rasterize_alphabet, CHARACTER_PX}, ASPECT_RATIO, draw_string, camera::Camera};
pub enum Object {
    Sphere(Sphere),
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
    pub window_width: u32, 
    pub window_height: u32, 
    alphabet: [Option<RasterizedCharacter>; 128],
    frame_count: u32,
    previous_frame_duration: u128,
}

impl Scene {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let s = Scene{
            camera: Camera::new(Vec3::new(0., 0., 2.), 90.0),
            objects: vec![],
            window_height,
            window_width,
            alphabet: rasterize_alphabet(),
            frame_count: 0,
            previous_frame_duration: 0,
        };

        println!("{:?}", s.camera);
        s
    }
    pub fn render(&mut self) -> Vec<Vec3<u8>> {
        let mut res = Vec::with_capacity((self.window_height * self.window_width) as usize);
        let now = Instant::now();

        for j in (0..self.window_height).rev() {
            for i in 0..self.window_width {
                let u = (i as f64) / (self.window_width) as f64 * 2.0 - 1.;
                let v = (j as f64) / (self.window_height) as f64 * 2.0 - 1.;
                let ray = self.camera.emit_ray(u, v);
                let color = ray.color(&self.objects, 0.01, f64::INFINITY, 1);

                res.push(
                    Vec3::new(
                        color.x as u8, 
                        color.y as u8, 
                        color.z as u8, 
                    )
                );
            }
        }
        let x_pos = 100;
        let y_pos = 50;
        draw_string!(&format!("{:?}ms", self.previous_frame_duration as f64 / 1000.), &self.alphabet, &mut res, self.window_width, x_pos, y_pos);
        self.frame_count += 1;
        if self.frame_count % 10 == 0 {
            let new_now = Instant::now();
            self.previous_frame_duration = new_now.duration_since(now).as_micros();
        }

        return res;
    }
}