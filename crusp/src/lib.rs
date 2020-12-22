extern crate rand;
extern crate rayon;
extern crate snowflake;
extern crate enumflags2;

#[macro_use]
mod macros;
#[macro_use]
pub mod variables;
#[macro_use]
pub mod constraints;
#[macro_use]
pub mod branchers;
pub mod graph;
pub mod search;
pub mod spaces;
