extern crate rand;
extern crate snowflake;

// The order of modules matter !!!! (macros)

trace_macros!(false);
#[macro_use]
mod macros;
#[macro_use]
pub mod variables;
trace_macros!(false);
pub mod constraints;
trace_macros!(false);
pub mod branchers;
pub mod spaces;
