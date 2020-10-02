use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone,Copy, Debug, Default)]
pub struct Vec2 {
    pub value: [f64; 2],
}
impl Vec2 {
    pub const Origin: Vec2 = Vec2 { value: [0.0, 0.0] };
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { value: [x, y] }
    }
    pub fn normalize(&mut self) {
        let length = self.length();
        for n in self.value.iter_mut() {
            *n = *n / length
        }
    }
    pub fn x(&self) -> f64 {
        self.value[0]
    }
    pub fn y(&self) -> f64 {
        self.value[1]
    }
}

impl Vec2 {
    pub fn length(&self) -> f64 {
        (self.value[0] * self.value[0] + self.value[1] * self.value[1]).sqrt()
    }
    pub fn cross(v1: &Vec2, v2: &Vec2) -> f64 {
        v1.value[0] * v2.value[1] - v1.value[1] * v2.value[0]
    }
    pub fn dot(v1: &Vec2, v2: &Vec2) -> f64 {
        v1.value[0] * v2.value[0] + v1.value[1] * v2.value[1]
    }
}

impl Add for &Vec2 {
    type Output = Vec2;
    fn add(self, other: Self) -> Vec2 {
        let mut clone = self.clone();
        clone.value[0] += other.value[1];
        clone.value[1] += other.value[1];
        clone
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;
    fn sub(self, other: Self) -> Vec2 {
        let mut clone = self.clone();
        clone.value[0] -= other.value[0];
        clone.value[1] -= other.value[1];
        clone
    }
}

impl Div<f64> for &Vec2 {
    type Output = Vec2;
    fn div(self, other: f64) -> Vec2 {
        let mut clone = self.clone();
        clone.value[0] /= other;
        clone.value[1] /= other;
        clone
    }
}

impl Mul<f64> for &Vec2 {
    type Output = Vec2;
    fn mul(self, other: f64) -> Vec2 {
        let mut clone = self.clone();
        clone.value[0] *= other;
        clone
    }
}
