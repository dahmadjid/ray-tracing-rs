#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display};

use rand::Rng;

use crate::mat3::Mat3;

#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec4<T> {
    pub fn new(w:T, x: T, y: T, z: T) -> Self {
        Self{w, x, y, z}
    }
}

impl Vec4<f64> {
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.w * rhs.w + self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> Self {
        self.w *= factor;
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        *self
    }

    pub fn normalize(&self) -> Vec4<f64> {
        let length = self.length();
        Vec4::new(
            self.w / length,
            self.x / length, 
            self.y / length, 
            self.z / length,
        )
    }

    pub fn floor(&self) -> Vec4<i64> {
        Vec4{
            w: self.w.floor() as i64,
            x: self.x.floor() as i64,
            y: self.y.floor() as i64, 
            z: self.z.floor() as i64,
        } 
    }

    pub fn ceil(&self) -> Vec4<i64> {
        Vec4{
            w: self.w.ceil() as i64,
            x: self.x.ceil() as i64,
            y: self.y.ceil() as i64, 
            z: self.z.ceil() as i64,
        } 
    }

    pub fn random() -> Vec4<f64> {
        let mut rng = rand::thread_rng();
        Vec4::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }

    pub fn shift(&mut self, scalar: f64) -> Vec4<f64> {
        self.w += scalar;
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
        *self
    }

}

impl<T: Add<Output = T>> Add for Vec4<T> {
    type Output = Vec4<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.w + rhs.w, self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec4<T>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.w - rhs.w, self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T>> Mul for Vec4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.w * rhs.w, self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Div<Output = T>> Div for Vec4<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.w / rhs.w, self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Neg<Output = T>> Neg for Vec4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.w, -self.x, -self.y, -self.z)
    }
}

impl<T: Display> Display for Vec4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.w, self.x, self.y, self.z)
    }
}

impl From<Vec4<f64>> for Vec4<u8> {
    fn from(v: Vec4<f64>) -> Vec4<u8> {
        Vec4::<u8>{w: v.w as u8, x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec4<i64>> for Vec4<u8> {
    fn from(v: Vec4<i64>) -> Vec4<u8> {
        Vec4::<u8>{w: v.w as u8, x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec4<u8>> for Vec4<f64> {
    fn from(v: Vec4<u8>) -> Vec4<f64> {
        Vec4::<f64>{w: v.w as f64, x: v.x as f64, y: v.y as f64, z: v.z as f64}
    }
}

impl<T: PartialEq> PartialEq for Vec4<T> {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.x == other.x && self.y == other.y && self.z == other.z
    }
}