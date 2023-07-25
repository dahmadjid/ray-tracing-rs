use crate::vec3::Vec3;
use crate::ray::Ray;
#[derive(Clone, Copy)]
pub struct HitReturn {
    pub position: Vec3<f64>,
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
        let oc = ray.origin.clone() - self.center.clone();
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b * half_b - a*c;
        if discriminant < 0.0 {
            None
        } else {
            let discriminant_sqrted = discriminant.sqrt();
            let mut root = (-half_b - discriminant_sqrted) / a;
            if root > t_max || root < t_min {
                root = (-half_b + discriminant_sqrted ) / a;
                if root > t_max || root < t_min {
                    return None;
                } 
            }
            let ray_at = ray.at(root);
            let normal = (ray_at.clone() - self.center.clone()).scale(1.0 / self.radius);
            if ray.direction.dot(&normal) > 0.0 {
                Some(HitReturn{position: ray_at, normal: -normal, front_face: true, t: root})
            } else {
                Some(HitReturn{position: ray_at, normal: normal, front_face: false, t: root})
            }
        }
    }
}