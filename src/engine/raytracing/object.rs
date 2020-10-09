use crate::Vec3;

pub trait Object {
    fn bounding_box() -> (Vec3, Vec3);
}