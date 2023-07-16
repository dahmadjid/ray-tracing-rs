#![allow(dead_code)]

use crate::vec3::{Vec3, Number};

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
}
