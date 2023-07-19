use crate::vec3::{Vec3, Number};
use crate::ray::Ray;
#[derive(Clone)]
pub struct HitReturn<T> {
    pub position: Vec3<T>,
    pub normal: Vec3<T>,
    pub t: T,
    pub front_face: bool,

}
pub trait Hittable<T>
where T: Number {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitReturn<T>>;
}

pub struct Sphere<T> {
    pub radius: T,
    pub center: Vec3<T>,
}


impl<T> Hittable<T> for Sphere<T> 
where T: Number {
    fn hit(&self, ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitReturn<T>> {
        let oc = ray.origin.clone() - self.center.clone();
        let a = T::from(ray.direction.length_squared()).unwrap_or_default();
        let half_b = oc.dot(&ray.direction);
        let c = T::from(oc.length_squared()).unwrap() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < T::zero() {
            None
        } else {
            let discriminant_sqrted = T::from(discriminant.to_f64().unwrap_or(-1.).sqrt()).unwrap_or_default();
            let mut root = (T::zero()-half_b - discriminant_sqrted ) / a;
            if root > t_max || root < t_min {
                root = (T::zero()-half_b + discriminant_sqrted ) / a;
                if root > t_max || root < t_min {
                    return None;
                } 
            }
            let ray_at = ray.at(root);
            let normal = (ray_at.clone() - self.center.clone()).scale(T::one() / self.radius);
            if ray.direction.dot(&normal) > T::zero() {
                Some(HitReturn{position: ray_at, normal: -normal, front_face: true, t: root})
            } else {
                Some(HitReturn{position: ray_at, normal: normal, front_face: false, t: root})
            }
        }
    }
}