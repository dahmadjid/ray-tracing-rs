use crate::{hittable::*, image::Image, vec3::{Vec3, Number}, ray::Ray};
pub enum Object<T> {
    Sphere(Sphere<T>),
}

pub struct Viewport<T> {
    pub height: T,
    pub width: T,
    pub horizontal_axis: Vec3<T>,
    pub vertical_axis: Vec3<T>,
}

pub struct Camera<T> {
    pub position: Vec3<T>,
    pub focal_length: T,
}

impl<T: Number> Camera<T> {
    pub fn emit_ray(&self, direction: Vec3<T>) -> Ray<T> {
        Ray{origin: self.position.clone(), direction:direction}
    }
}

pub struct Scene<T> {
    pub objects: Vec<Object<T>>,
    pub viewport: Viewport<T>,
    pub camera: Camera<T>,
    pub image: Image<i64>,
}

impl Scene<f64> {
    pub fn render(&mut self) {

        let lower_left_corner = self.camera.position.clone()
            - self.viewport.horizontal_axis.scale(0.5) 
            - self.viewport.vertical_axis.scale(0.5) 
            - Vec3::new(0., 0., self.camera.focal_length);

        for j in (0..self.image.height).rev() {
            for i in 0..self.image.width {
                let u = i as f64 / (self.image.width -1) as f64;
                let v = j as f64 / (self.image.height -1) as f64;
                let ray = self.camera.emit_ray(
                              lower_left_corner.clone()
                            + self.viewport.horizontal_axis.scale(u) 
                            + self.viewport.vertical_axis.scale(v) 
                            - self.camera.position.clone());
                
                let color = ray.color(&self.objects, 0.0, f64::INFINITY).scale(255.999).floor();
                if color.x >= 0 && color.y >= 0 && color.z >= 0 {
                    self.image.data.push(color);
                } else {
                    println!("color less than 0 {color:?}");
                }
            }
        }
    
        self.image.render();
        self.image.show();     
    }
}