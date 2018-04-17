extern crate snowflake;

// The order of modules matter !!!! (macros)

#[macro_use]
pub mod boxed_clone;
#[macro_use]
pub mod variables;
pub mod constraints;
pub mod branchers;
pub mod spaces;
