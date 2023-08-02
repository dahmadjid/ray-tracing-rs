use std::time::Instant;
use crate::{hittable::*, vec3::Vec3, fonts::{render_string, RasterizedCharacter, rasterize_alphabet, CHARACTER_PX}, ASPECT_RATIO, draw_string, camera::Camera, ray::{Ray, self}};
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
        Scene{
            camera: Camera::new(45.0, window_width as usize, window_height as usize),
            objects: vec![],
            window_height,
            window_width,
            alphabet: rasterize_alphabet(),
            frame_count: 0,
            previous_frame_duration: 0,
        }
    }
    pub fn render(&mut self) -> Vec<Vec3<u8>> {
        let mut res = Vec::with_capacity((self.window_height * self.window_width) as usize);
        let now = Instant::now();

        for y in (0..self.window_height).rev() {
            for x in 0..self.window_width {

                let ray_direction = self.camera.ray_directions[(x + y * self.window_width) as usize];
                let ray = Ray{origin: self.camera.position.clone(), direction: ray_direction};
                let color = ray.color(&self.objects, 0.01, f64::INFINITY, 10);

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