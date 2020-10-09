

use crate::Material;
use crate::Vec4;
use crate::Vec3;

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<Sphere>
}

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vec4,
    pub radius: f64,
    pub material: Material,
    pub color: Vec3,
}

impl Scene {
    pub fn add_sphere(&mut self, origin: Vec4, radius: f64, material: Material, color: Vec3) {
        self.objects.push(Sphere {
            origin,
            radius,
            material,
            color,
        })
    }
}