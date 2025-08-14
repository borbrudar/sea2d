/// Modul, ki vsebuje strukturo `Point`, ki predstavlja točko v dveh dimenzijah.
use std::hash::Hash;

/// Struktura, ki predstavlja točko v dveh dimenzijah.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Ustvari novo točko z danima koordinatama `x` in `y`.
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}
