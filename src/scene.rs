use std::time::Instant;

use crate::{hittable::*, vec3::Vec3, ray::Ray};
pub enum Object {
    Sphere(Sphere),
}

pub struct Camera {
    pub height: f64,
    pub width: f64,
    pub horizontal_axis: Vec3<f64>,
    pub vertical_axis: Vec3<f64>,
    pub position: Vec3<f64>,
    pub focal_length: f64,
}

impl Camera {
    pub fn new(width: f64, height: f64, position: Vec3<f64>, focal_length: f64) -> Camera {
        Camera{
            position,
            focal_length,
            height: height,
            width: width,
            horizontal_axis: Vec3::new(width, 0.0, 0.0),
            vertical_axis: Vec3::new(0.0, height, 0.0),
        }

    }
    pub fn emit_ray(&self, u: f64, v: f64) -> Ray {
        let lower_left_corner = self.position.clone()
        - self.horizontal_axis.scale(0.5)
        - self.vertical_axis.scale(0.5)
        - Vec3::new(0.0, 0.0, self.focal_length);

        let direction = lower_left_corner
        + self.horizontal_axis.scale(u) 
        + self.vertical_axis.scale(v) 
        - self.position.clone();

        Ray{origin: self.position.clone(), direction}
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
    pub window_width: u32, 
    pub window_height: u32, 
}

impl Scene {
    pub fn render(&self) -> Vec<Vec3<u8>> {
        let mut res = Vec::with_capacity((self.window_height * self.window_width).try_into().unwrap());
        let now = Instant::now();

        for j in (0..self.window_height).rev() {
            for i in 0..self.window_width {
                let u = (i as f64) / (self.window_width -1) as f64;
                let v = (j as f64) / (self.window_height -1) as f64;
                let ray = self.camera.emit_ray(u, v);
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
        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));
        return res;
    }
}