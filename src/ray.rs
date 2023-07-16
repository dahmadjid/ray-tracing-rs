#![allow(dead_code)]

use crate::{vec3::{Vec3, Number}, scene::Object, hittable::{HitReturn, Hittable}};

#[derive(Debug, Clone)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T>
where T: Number {
    pub fn at(&self, t:T) -> Vec3<T> {
        self.origin.clone() + self.direction.scale(t)
    }

    pub fn hit(&self, objects: &Vec<Object<T>>, t_min: T, t_max: T) -> Option<HitReturn<T>> {
        let mut ret : Option<HitReturn<T>> = None;
        let mut closest = t_max;
        for obj in objects {
            if let Some(hit_return)= match obj {
                Object::Sphere(sphere) => sphere.hit(self, t_min, closest), 
            } {
                closest = hit_return.t;
                ret = Some(hit_return);
            }
        }
        ret
    }

    pub fn color(&self, objects: &Vec<Object<T>>, t_min: T, t_max: T) -> Vec3<T> {
        if let Some(hit_return) = self.hit(objects, t_min, t_max) {
            (hit_return.normal + Vec3::new(T::one(), T::one(), T::one())).scale(T::from(0.5).unwrap())
        } else {
            let unit_direction = self.direction.normalize();
            let t = T::from(0.5*(unit_direction.y + 1.0)).unwrap();
            Vec3::new(T::one(), T::one(), T::one()).scale(T::one() - t) + Vec3::new(T::from(0.5).unwrap(), T::from(0.7).unwrap(), T::one()).scale(t)
        }
    }


}
