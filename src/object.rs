
use crate::engine::*;

pub trait Object{
    fn get_position (&self) -> &Vec4;
    fn set_position(&mut self, pos: &Vec4);
    fn get_front(&self) -> Vec4;
    fn set_front(&mut self, pos: &Vec4);
    fn get_up(&self)-> Vec4;
    fn set_up(&mut self, pos: &Vec4);
}
