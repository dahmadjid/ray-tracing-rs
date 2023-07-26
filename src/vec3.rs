#![allow(dead_code)]

use std::{ops::{Mul, Sub, Add, Div, Neg}, fmt::Display};

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self{x, y, z}
    }
}

impl Vec3<f64> {
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y - rhs.x,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn scale(&mut self, factor: f64) -> Self {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        *self
    }

    pub fn normalize(&self) -> Vec3<f64> {
        let length = self.length();
        Vec3::new(
            self.x / length, 
            self.y / length, 
            self.z / length,
        )
    }

    pub fn floor(&self) -> Vec3<i64> {
        Vec3{
            x: self.x.floor() as i64,
            y: self.y.floor() as i64, 
            z: self.z.floor() as i64,
        } 
    }

    pub fn ceil(&self) -> Vec3<i64> {
        Vec3{
            x: self.x.ceil() as i64,
            y: self.y.ceil() as i64, 
            z: self.z.ceil() as i64,
        } 
    }

    pub fn random() -> Vec3<f64> {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn shift(&mut self, scalar: f64) -> Vec3<f64> {
        self.x += scalar;
        self.y += scalar;
        self.z += scalar;
        *self
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Mul<Output = T>> Mul for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T: Div<Output = T>> Div for Vec3<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Neg<Output = T>> Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl From<Vec3<f64>> for Vec3<u8> {
    fn from(v: Vec3<f64>) -> Vec3<u8> {
        Vec3::<u8>{x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec3<i64>> for Vec3<u8> {
    fn from(v: Vec3<i64>) -> Vec3<u8> {
        Vec3::<u8>{x: v.x as u8, y: v.y as u8, z: v.z as u8}
    }
}

impl From<Vec3<u8>> for Vec3<f64> {
    fn from(v: Vec3<u8>) -> Vec3<f64> {
        Vec3::<f64>{x: v.x as f64, y: v.y as f64, z: v.z as f64}
    }
}
