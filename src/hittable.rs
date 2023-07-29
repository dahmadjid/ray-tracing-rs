use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Copy)]
pub struct HitReturn {
    pub hit_position: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub t: f64,
    pub front_face: bool,

}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitReturn>;
}

pub struct Sphere {
    pub radius: f64,
    pub center: Vec3<f64>,
}


impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitReturn> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.origin.dot(&ray.direction);
        let c = ray.origin.dot(&ray.origin) - self.radius*self.radius;
        let discriminant =  b * b - 4. * a * c;
        if discriminant < 0.0 {
                None
        } else {
            let discriminant_sqrted = discriminant.sqrt();
            let mut root = (-b - discriminant_sqrted) / 2.0 * a;
            if root > t_max || root < t_min {
                root = (-b + discriminant_sqrted ) / 2.0 * a;
                if root > t_max || root < t_min {
                    return None;
                } 
            }
            let ray_at = ray.at(root);
            let normal = (ray_at - self.center).scale(1.0 / self.radius);
            if ray.direction.dot(&normal) > 0.0 {
                Some(HitReturn{hit_position: ray_at, normal: -normal, front_face: true, t: root})
            } else {
                Some(HitReturn{hit_position: ray_at, normal: normal, front_face: false, t: root})
            }
        }
    }
}