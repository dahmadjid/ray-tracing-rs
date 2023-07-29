use crate::{vec3::Vec3, ray::Ray, ASPECT_RATIO, mat3::Mat3};
pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub horizontal_axis: Vec3<f64>,
    pub vertical_axis: Vec3<f64>,
    pub position: Vec3<f64>,
    pub focal_length: f64,
}

impl Camera {
    pub fn new(position: Vec3<f64>, vfov: f64) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height =  2.0 * h;
        let viewport_width =  viewport_height * ASPECT_RATIO;

        let horizontal_axis = Vec3::new(viewport_width/2.0, 0.0, 0.0);
        let vertical_axis = Vec3::new(0.0, viewport_height/2.0, 0.0);

        let focal_length = 1.0;
        Camera{
            position,
            focal_length,
            viewport_height,
            viewport_width,
            horizontal_axis, 
            vertical_axis,
        }
    }
    
    pub fn emit_ray(&self, u: f64, v: f64) -> Ray {

        let direction = self.horizontal_axis.cross(&self.vertical_axis).normalize()
        + self.horizontal_axis.clone().scale(u) + self.vertical_axis.clone().scale(v);
        
        Ray{origin: self.position.clone(), direction}
    }
    
    pub fn update_x_position(&mut self, x: f64) {
        self.position.x += x;
    }
    pub fn update_y_position(&mut self, y: f64) {
        self.position.y += y;
    }
    pub fn update_z_position(&mut self, z: f64) {
        self.position.z += z;
    }
    pub fn update_position(&mut self, new_pos: Vec3<f64>) {
        self.position = new_pos;
    }
}
