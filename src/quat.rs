#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display, vec};

use rand::Rng;

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Quat<T> {
    pub w: T,
    pub v: Vec3<T>,
}

impl<T> Quat<T> {
    pub fn new(w:T, x: T, y: T, z: T) -> Self {
        Self{w, v: Vec3::new(x, y, z)}
    }
}

impl Quat<f64> {
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.w * rhs.w + self.v.dot(&rhs.v)
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Quat::new(
            self.w * rhs.w - self.v.x * rhs.v.x - self.v.y * rhs.v.y - self.v.z * rhs.v.z,
            self.w * rhs.v.x + self.v.x * rhs.w + self.v.y * rhs.v.z - self.v.z * rhs.v.y,
            self.w * rhs.v.y + self.v.y * rhs.w + self.v.z * rhs.v.x - self.v.x * rhs.v.z,
            self.w * rhs.v.z + self.v.z * rhs.w + self.v.x * rhs.v.y - self.v.y * rhs.v.x
        )
    }
    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> Self {
        self.w *= factor;
        self.v.scale(factor);
        *self
    }

    pub fn normalize(&self) -> Quat<f64> {
        let one_over_len = 1.0 / self.length();
        Quat::new(
            self.w * one_over_len,
            self.v.x * one_over_len, 
            self.v.y * one_over_len, 
            self.v.z * one_over_len,
        )
    }

    pub fn floor(&self) -> Quat<i64> {
        Quat::new(
            self.w.floor() as i64,
            self.v.x.floor() as i64,
            self.v.y.floor() as i64, 
            self.v.z.floor() as i64,
        )
    }

    pub fn ceil(&self) -> Quat<i64> {
        Quat::new(
            self.w.ceil() as i64,
            self.v.x.ceil() as i64,
            self.v.y.ceil() as i64, 
            self.v.z.ceil() as i64,
        )
    }

    pub fn random() -> Quat<f64> {
        let mut rng = rand::thread_rng();
        Quat::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }

    pub fn shift(&mut self, scalar: f64) -> Quat<f64> {
        self.w += scalar;
        self.v.shift(scalar);
        *self
    }

    pub fn angle_axis(angle_radians: f64, unit_axis: Vec3<f64>) -> Quat<f64> {
        let half_cos_angle = (angle_radians / 2.0).cos();
        let half_sin_angle = (angle_radians / 2.0).sin();
        Quat::new(
            half_cos_angle,
            unit_axis.x * half_sin_angle,
            unit_axis.y * half_sin_angle,
            unit_axis.z * half_sin_angle
        )
    }

}

impl<T: Add<Output = T>> Add for Quat<T> {
    type Output = Quat<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self{w: self.w + rhs.w, v:self.v + rhs.v}
    }
}

impl<T: Sub<Output = T>> Sub for Quat<T>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{w: self.w - rhs.w, v:self.v - rhs.v}
    }
}


impl<T: Neg<Output = T>> Neg for Quat<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(self.w, -self.v.x, -self.v.y, -self.v.z)
    }
}

impl<T: Display> Display for Quat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.w, self.v.x, self.v.y, self.v.z)
    }
}

impl From<Quat<f64>> for Quat<u8> {
    fn from(q: Quat<f64>) -> Quat<u8> {
        Quat::<u8>{w: q.w as u8, v: q.v.into()}
    }
}

impl From<Quat<i64>> for Quat<u8> {
    fn from(q: Quat<i64>) -> Quat<u8> {
        Quat::<u8>{w: q.w as u8, v: q.v.into()}
    }
}

impl From<Quat<u8>> for Quat<f64> {
    fn from(q: Quat<u8>) -> Quat<f64> {
        Quat::<f64>{w: q.w as f64, v: q.v.into()}
    }
}

impl<T: PartialEq> PartialEq for Quat<T> {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.v == other.v  
    }
}