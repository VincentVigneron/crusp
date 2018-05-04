extern crate rand;
extern crate snowflake;

// The order of modules matter !!!! (macros)

#[macro_use]
mod macros;
#[macro_use]
pub mod variables;
#[macro_use]
pub mod constraints;
#[macro_use]
pub mod branchers;
pub mod spaces;
pub mod search;
pub mod graph;
