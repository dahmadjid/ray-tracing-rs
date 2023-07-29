use crate::{vec3::Vec3, ray::Ray, ASPECT_RATIO, mat3::Mat3};
#[derive(Debug)]
pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub horizontal_axis: Vec3<f64>,
    pub vertical_axis: Vec3<f64>,
    pub forward_axis: Vec3<f64>,
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
        let forward_axis = Vec3::new(0.0, 0.0, -1.0);
        let focal_length = 1.0;

        Camera{
            position,
            focal_length,
            viewport_height,
            viewport_width,
            horizontal_axis, 
            vertical_axis,
            forward_axis,
        }
    }
    
    pub fn emit_ray(&self, u: f64, v: f64) -> Ray {

        let direction = self.forward_axis
        + self.horizontal_axis.clone().scale(u) + self.vertical_axis.clone().scale(v);
        
        Ray{origin: self.position.clone(), direction}
    }
    
    pub fn update_x_position(&mut self, x: f64) {
        self.position = self.position + self.horizontal_axis.clone().scale(x);
    }
    pub fn update_y_position(&mut self, y: f64) {
        self.position = self.position + self.vertical_axis.clone().scale(y);
    }
    pub fn update_z_position(&mut self, z: f64) {
        self.position = self.position + self.forward_axis.clone().scale(z);
    }

    pub fn rotate_yaw(&mut self, degrees_angle: f64) {
        self.horizontal_axis.rotate_yaw(degrees_angle);
        self.forward_axis.rotate_yaw(degrees_angle);
    }

    pub fn rotate_pitch(&mut self, degrees_angle: f64) {
        self.vertical_axis.rotate_pitch(degrees_angle);
        self.forward_axis.rotate_pitch(degrees_angle);
    }
}
