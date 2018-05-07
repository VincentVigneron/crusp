#[macro_use]

pub mod tests;

pub use self::bounds::IntVarBounds;
pub use self::intervals::IntVarIntervals;
pub use self::values::IntVarValues;

mod values;
mod intervals;
mod bounds;
