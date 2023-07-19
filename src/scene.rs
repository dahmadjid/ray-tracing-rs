use std::time::Instant;

use rand::Rng;

use crate::{hittable::*, image::Image, vec3::{Vec3, Number}, ray::Ray};
pub enum Object<T> {
    Sphere(Sphere<T>),
}

pub struct Camera<T> {
    pub height: T,
    pub width: T,
    pub horizontal_axis: Vec3<T>,
    pub vertical_axis: Vec3<T>,
    pub position: Vec3<T>,
    pub focal_length: T,
}

impl<T> Camera<T> 
where T: Number {
    pub fn new(width:T, height: T, position: Vec3<T>, focal_length:T) -> Camera<T> {
        Camera{
            position,
            focal_length,
            height: height,
            width: width,
            horizontal_axis: Vec3::new(width, T::zero(), T::zero()),
            vertical_axis: Vec3::new(T::zero(), height, T::zero()),
        }

    }
}

impl<T: Number> Camera<T> 
where T: Number {
    pub fn emit_ray(&self, u: T, v: T) -> Ray<T> {
        let lower_left_corner = self.position.clone()
        - self.horizontal_axis.scale(T::from(0.5).unwrap()) 
        - self.vertical_axis.scale(T::from(0.5).unwrap()) 
        - Vec3::new(T::zero(), T::zero(), self.focal_length);

        let direction = lower_left_corner
        + self.horizontal_axis.scale(u) 
        + self.vertical_axis.scale(v) 
        - self.position.clone();

        Ray{origin: self.position.clone(), direction}
    }
}

pub struct Scene<T> {
    pub objects: Vec<Object<T>>,
    pub camera: Camera<T>,
    pub image: Image<u8>,
}

impl Scene<f64> {
    pub fn render(&mut self) {
        let now = Instant::now();

        for j in (0..self.image.height).rev() {
            for i in 0..self.image.width {
                let mut out_color = Vec3::<u32>::new(0, 0, 0); 
                let sample_per_pixel = 2;
                let mut rng = rand::thread_rng();
                for _i in 0..sample_per_pixel {
                    let rand_u: f64 = rng.gen(); 
                    let rand_v: f64 = rng.gen(); 
                    let u = (i as f64 + rand_u) / (self.image.width -1) as f64;
                    let v = (j as f64 + rand_v) / (self.image.height -1) as f64;
                    // println!("{rand_u} {rand_v}");
                    let ray = self.camera.emit_ray(u, v);
                    let color = ray.color(&self.objects, 0.0, f64::INFINITY);
                    out_color = out_color + Vec3::new(color.x as u32, color.y as u32, color.z as u32);
                }

                let temp = Vec3::new(out_color.x as f64 / sample_per_pixel as f64, out_color.y as f64 / sample_per_pixel as f64, out_color.z as f64 / sample_per_pixel as f64);
                
                self.image.data.push(
                    Vec3::new(
                        temp.x as u8, 
                        temp.y as u8, 
                        temp.z as u8, 
                    )
                );
            }
        }
        let new_now = Instant::now();
        println!("{:?}", new_now.duration_since(now));

        self.image.render();
        self.image.show();     
    }
}