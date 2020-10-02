use super::base::*;

pub struct Program {
    pub vertex_shader: Box<dyn Fn(&Vec<ShaderData>, &Vec<ShaderData>, Vec4) -> Vec4>,
    pub fragment_shader: Box<dyn Fn(&Vec<ShaderData>, &Vec<ShaderData>, Vec2) -> Vec4>,
    pub attributes: Vec<Attribute>,
    pub uniforms: Vec<Uniform>,
}

#[derive(Clone)]
pub struct Attribute {
    pub index: usize,
    pub name: String,
}

pub struct Uniform {
    pub name: String,
    pub value: ShaderData,
}

#[derive(Clone, Debug)]
pub enum ShaderData {
    Float(f64),
    Vec4(Vec4),
    Mat4(Mat4),
}

impl std::ops::Mul<f64> for &ShaderData {
    type Output = ShaderData;
    fn mul(self, rhs: f64) -> ShaderData {
        match self {
            ShaderData::Float(v) => ShaderData::Float(v * rhs),
            ShaderData::Vec4(v) => ShaderData::Vec4(v * rhs),
            ShaderData::Mat4(v) => ShaderData::Mat4(v * rhs),
        }
    }
}

impl std::ops::Add<ShaderData> for ShaderData {
    type Output = ShaderData;
    fn add(self, rhs: ShaderData) -> ShaderData {
        match (self, rhs) {
            (ShaderData::Float(v), ShaderData::Float(rhs)) => ShaderData::Float(&v + &rhs),
            (ShaderData::Vec4(v), ShaderData::Vec4(rhs)) => ShaderData::Vec4(&v + &rhs),
            (ShaderData::Mat4(v), ShaderData::Mat4(rhs)) => ShaderData::Mat4(&v + &rhs),
            (left, _) => left,
        }
    }
}
