extern crate num_traits;

pub mod physics;
pub mod type_defs;

use type_defs::FP;
use crate::physics::structs::{Vector2, RectCollider};
use fixed_macro::fixed;

fn main() {
    // LUT table test
    const SIN45: FP = fixed!(0.7071: I48F16);
    // normal fixed point testing
    let numb = FP::from_num(19) / 3;
    println!("Hello, world! {0}, {1}", numb, SIN45);

    let numb = FP::from_num(1) -  FP::from_num(3);
    println!("{}", numb);

    let vec = Vector2::from_float::<f64>(0.0, 0.0);

    let collider = RectCollider::new(FP::from_num(2), FP::from_num(2), vec);

    println!("{}", collider.get_top_left());

    let vec2 = Vector2::from_float(0.66, -0.8);
    println!("{}", vec2);
}
