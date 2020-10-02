use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone,Copy, Debug)]
pub struct Vec3 {
    pub value: [f64; 3],
}
impl Vec3 {
    pub const Origin: Vec3 = Vec3 {
        value: [0.0, 0.0, 0.0],
    };
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { value: [x, y, z] }
    }
    pub fn normalize(&mut self) {
        let length = self.length();
        for n in self.value.iter_mut() {
            *n = *n / length
        }
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.value[0] * self.value[0]
            + self.value[1] * self.value[1]
            + self.value[2] * self.value[2])
            .sqrt()
    }
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        // (a b c) X (d e f)
        // (bf-ce, cd-af, ae-bd)
        Vec3::new(
            v1.value[1] * v2.value[2] - v1.value[2] * v2.value[1],
            v1.value[2] * v2.value[0] - v1.value[0] * v2.value[2],
            v1.value[0] * v2.value[1] - v1.value[1] * v2.value[0],
        )
    }
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.value[0] * v2.value[0] + v1.value[1] * v2.value[1] + v1.value[2] * v2.value[2]
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Vec3 {
        let mut clone = self.clone();
        clone.value[0] += other.value[0];
        clone.value[1] += other.value[1];
        clone.value[2] += other.value[2];
        clone
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: Self) -> Vec3 {
        let mut clone = self.clone();
        clone.value[0] -= other.value[0];
        clone.value[1] -= other.value[1];
        clone.value[2] -= other.value[2];
        clone
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        let mut clone = self.clone();
        clone.value[0] /= other;
        clone.value[1] /= other;
        clone.value[2] /= other;
        clone
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        let mut clone = self.clone();
        clone.value[0] *= other;
        clone.value[1] *= other;
        clone.value[2] *= other;
        clone
    }
}
