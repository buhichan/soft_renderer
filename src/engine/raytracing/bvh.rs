
pub struct Bvh<T> {
    left: Vec<Box<Bvh<T>>>,
    right: Vec<Box<Bvh<T>>>,
    object: Box<T>,
}

impl<T> Bvh<T> {
    pub fn new( ){
        unimplemented!()
    }
}