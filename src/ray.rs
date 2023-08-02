#![allow(dead_code)]

use crate::{vec3::{Vec3}, scene::Object, hittable::{HitReturn, Hittable}};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn hit(&self, objects: &Vec<Object>, t_min: f64, t_max: f64) -> Option<HitReturn> {
        let mut ret : Option<HitReturn> = None;
        let mut closest = t_max;
        for obj in objects {
            if let Some(hit_return)= match obj {
                Object::Sphere(sphere) => sphere.hit(self, t_min, closest), 
            } {
                if hit_return.t <= closest {
                    closest = hit_return.t;
                    ret = Some(hit_return);
                }
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
                // let mut v = Vec3::<f64>::random();
                // loop {
                //     if v.length_squared() < 1.0 {
                //         break;
                //     } 
                //     v = Vec3::<f64>::random();
                // };
                // v = v.normalize();
                
                // let target = hit_return.hit_position + hit_return.normal;
                // ray.origin = hit_return.hit_position;
                // ray.direction= target;
                let light_dir = Vec3::new(-1., -1., -1.,).normalize();
                // hit_count += 1;
                // total = total + hit_return.object_color.clone().scale(light_dir.dot(&hit_return.normal).max(0.0));
                // break;
                return hit_return.object_color.clone().scale((hit_return.normal.dot(&-light_dir)).min(1.0).max(0.0)).scale(255.99).into();
                // return hit_return.hit_position.clone().scale(255.99).into();
            } else {
                return Vec3::new(135, 206, 235);
            }

        }
        if hit_count > 0 {
            total.scale(1. / hit_count as f64).scale(255.99).into()

        } else {
            return Vec3::new(135, 206, 235);
        }
    }


}
