use crate::{vec3::Vec3, ray::Ray, ASPECT_RATIO, mat3::Mat3, quat::Quat};
#[derive(Debug)]
pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub window_width: usize,
    pub window_height: usize,
    pub z_axis: Vec3<f64>,
    pub position: Vec3<f64>,
    pub ray_directions: Vec<Vec3<f64>>,
}

impl Camera {
    pub fn new(vfov: f64, window_width: usize, window_height: usize) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height =  1.0 * h;
        let viewport_width =  viewport_height * ASPECT_RATIO;

        let z_axis = Vec3::new(0.0, 0.0, -1.0);
        let ray_directions = Vec::with_capacity(window_width * window_height);
        
        let mut camera = Camera{
            position: Vec3::new(0., 0., 6.),
            viewport_height,
            viewport_width,
            window_height,
            window_width,
            ray_directions,
            z_axis,
        };
        camera.calculate_ray_directions();
        camera
    }
    
    pub fn calculate_ray_directions(&mut self) {
        let up = Vec3::new(0.0, 1.0, 0.0);
        let right_direction = self.z_axis.cross(&up).normalize().scale(self.viewport_width/2.0);
        let up_direction = self.z_axis.cross(&right_direction).normalize().scale(self.viewport_height/2.0);
        self.ray_directions = Vec::with_capacity(self.window_width * self.window_height);
        for y in (0..self.window_height).rev() {
            let v = y as f64 / self.window_height as f64 * 2.0 - 1.0;
            for x in 0..self.window_width {
                let u = x as f64 / self.window_width as f64 * 2.0 - 1.0;
                self.ray_directions.push(self.z_axis + right_direction.clone().scale(u) + up_direction.clone().scale(v));
            }
        }
    }
    
    pub fn update_x_position(&mut self, x: f64) {
        let up_dir = Vec3::new(0.0, 1.0, 0.0);
        self.position = self.position + self.z_axis.cross(&up_dir).scale(x);
    }
    pub fn update_y_position(&mut self, y: f64) {
        self.position.y += y;
    }
    pub fn update_z_position(&mut self, z: f64) {
        self.position = self.position + self.z_axis.clone().scale(z);
    }

    pub fn rotate(&mut self, pitch_delta_radians: f64, yaw_delta_radians: f64) {
        let up_dir = Vec3::new(0.0, 1.0, 0.0);
        let right_direction = self.z_axis.cross(&up_dir).normalize();
        self.z_axis.rotate(Quat::angle_axis(-pitch_delta_radians, right_direction).cross(&Quat::angle_axis(-yaw_delta_radians, up_dir)).normalize());
    }

}
