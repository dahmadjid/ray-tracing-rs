use crate::{vec3::Vec3, ray::Ray};
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
        - self.horizontal_axis.clone().scale(0.5)
        - self.vertical_axis.clone().scale(0.5)
        - Vec3::new(0.0, 0.0, self.focal_length);

        let direction = lower_left_corner
        + self.horizontal_axis.clone().scale(u) 
        + self.vertical_axis.clone().scale(v) 
        - self.position.clone();

        Ray{origin: self.position.clone(), direction}
    }
}
