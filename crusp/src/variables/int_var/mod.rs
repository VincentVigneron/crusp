#[macro_use]

pub mod tests;

pub use self::bounds::IntVarBounds;
pub use self::intervals::IntVarIntervals;
pub use self::values::IntVarValues;
pub use self::values::IntVarValuesBuilder;

mod bounds;
mod intervals;
mod values;
