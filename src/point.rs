use std::{hash::{Hash, Hasher}, slice::SliceIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T>{
    pub x : T,
    pub y : T,
}

/*
impl PartialEq for Point<f64> {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON
    }
}

impl PartialEq for Point<i32>{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point<f64> {}
impl Eq for Point<i32> {}

impl Hash for Point<f64> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
    }
}

impl Hash for Point<i32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
*/


impl<T> Point<T>{
    pub fn new(x : T, y : T) -> Self{
        Point{
            x : x,
            y : y,
        }
    }
}