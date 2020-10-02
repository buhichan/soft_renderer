use core::ops::AddAssign;
use super::vec2::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub value: [f64; 4],
}

impl std::fmt::Display for Vec4 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{},{},{},{}", self.value[0],self.value[1],self.value[2],self.value[3])
    }
}

impl Vec4 {
    pub const Origin: Vec4 = Vec4 {
        value: [0.0, 0.0, 0.0, 1.0],
    };
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vec4 {
        Vec4 {
            value: [x, y, z, w],
        }
    }
    pub fn normalize(&mut self) {
        let length = self.length();
        for n in 0..3 {
            self.value[n] = self.value[n] / length
        }
        self.value[3] = 1.0;
    }
    pub fn x(&self) -> f64 {
        self.value[0]
    }
    pub fn y(&self) -> f64 {
        self.value[1]
    }
    pub fn z(&self) -> f64 {
        self.value[2]
    }
    pub fn w(&self) -> f64 {
        self.value[3]
    }
}

impl Vec4 {
    pub fn xy<'a>(&'a self) -> Vec2 {
        Vec2::new(self.x(), self.y())
    }
    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt() / self.w()
    }
    pub fn cross(v1: &Vec4, v2: &Vec4) -> Vec4 {
        // (a b c) X (d e f)
        // (bf-ce, cd-af, ae-bd)
        let inv_div = 1.0 / (v1.w() * v2.w());
        Vec4::new(
            (v1.y() * v2.z() - v1.z() * v2.y()) * inv_div,
            (v1.z() * v2.x() - v1.x() * v2.z()) * inv_div,
            (v1.x() * v2.y() - v1.y() * v2.x()) * inv_div,
            1.0,
        )
    }
    pub fn dot(v1: &Vec4, v2: &Vec4) -> f64 {
        let inv_div = 1.0 / (v1.w() * v2.w());
        (v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()) * inv_div
    }
}

impl Add for &Vec4 {
    type Output = Vec4;
    fn add(self, other: Self) -> Vec4 {
        let mut clone = self.clone();
        clone.value[0] = clone.x() / clone.w() + other.x() / other.w();
        clone.value[1] = clone.y() / clone.w() + other.y() / other.w();
        clone.value[2] = clone.z() / clone.w() + other.z() / other.w();
        clone.value[3] = 1.0;
        clone
    }
}

impl AddAssign<&Self> for Vec4 {
    fn add_assign(&mut self, other: &Self) {
        self.value[0] += other.value[0];
        self.value[1] += other.value[1];
        self.value[2] += other.value[2];
        self.value[3] += other.value[3];
    }
}

impl Sub for &Vec4 {
    type Output = Vec4;
    fn sub(self, other: Self) -> Vec4 {
        let mut clone = self.clone();
        clone.value[0] = clone.x() / clone.w() - other.x() / other.w();
        clone.value[1] = clone.y() / clone.w() - other.y() / other.w();
        clone.value[2] = clone.z() / clone.w() - other.z() / other.w();
        clone.value[3] = 1.0;
        clone
    }
}

impl Div<f64> for &Vec4 {
    type Output = Vec4;
    fn div(self, other: f64) -> Vec4 {
        let mut clone = self.clone();
        clone.value[3] *= other;
        clone
    }
}

impl Mul<f64> for &Vec4 {
    type Output = Vec4;
    fn mul(self, other: f64) -> Vec4 {
        let mut clone = self.clone();
        clone.value[3] /= other;
        clone
    }
}
