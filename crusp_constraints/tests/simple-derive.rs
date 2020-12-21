//extern crate constraint_derive;

use constraint_derive::Constraint;

#[derive(Constraint)]
pub struct A {
    x: i32,
    y: i32,
}
/*
#[derive(Constraint)]
pub struct Constraint {
    #[var] x: i32,
    #[var] y: i32,
}*/

pub fn main() {

}
