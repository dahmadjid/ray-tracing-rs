#![allow(dead_code)]

use crate::{vec3::{Vec3}, scene::Object, hittable::{HitReturn, Hittable}};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn at(&self, t:f64) -> Vec3<f64> {
        self.origin.clone() + self.direction.clone().scale(t)
    }

    pub fn hit(&self, objects: &Vec<Object>, t_min: f64, t_max: f64) -> Option<HitReturn> {
        let mut ret : Option<HitReturn> = None;
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

    pub fn color(&self, objects: &Vec<Object>, t_min: f64, t_max: f64, max_depth: u32) -> Vec3<u8> {
        if max_depth == 0 {
            return Vec3::new(0, 0, 0);
        }
        let mut total = Vec3::new(0., 0., 0.);
        let mut ray = self.clone();
        let mut hit_count = 0;
        for _ in (0..max_depth).rev() {
            if let Some(hit_return) = ray.hit(objects, t_min, t_max) {
                let mut v = Vec3::<f64>::random();
                loop {
                    if v.length_squared() < 1.0 {
                        break;
                    } 
                    v = Vec3::<f64>::random();
                };
                v = v.normalize();
                
                let target = hit_return.position + hit_return.normal + v;
                ray.origin = hit_return.position;
                ray.direction= target;
                
                let mut sphere_color = Vec3::new(1., 0., 1.);
                let light_dir = Vec3::new(-1., -1., -1.);

                total = total + sphere_color.scale(hit_return.normal.dot(&-light_dir));
                hit_count += 1;
            } else {
                break;
            }

        }
        if hit_count > 0 {
            total.scale(1. / hit_count as f64).scale(255.99).into()

        } else {
            Vec3::new(0, 0, 0)
        }
    }


}
