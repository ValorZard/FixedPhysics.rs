use num_traits::{Float, PrimInt};
use fixed::traits::ToFixed;
use std::fmt;

use crate::type_defs::FP;

pub struct Vector2 {
    pub x : FP,
    pub y : FP,
}

impl Vector2 {
    pub fn new(x : FP, y : FP) -> Self {
        Self {
            x : x,
            y : y,
        }
    }
    pub fn from_float<T: Float + ToFixed>(x : T, y : T) -> Self {
        Self::new(FP::from_num(x), FP::from_num(y))
    }
    pub fn from_int<T: PrimInt + ToFixed>(x : T, y : T) -> Self {
        Self::new(FP::from_num(x), FP::from_num(y))
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    }
}