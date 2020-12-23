#[macro_use]

pub mod tests;

// IntVarBounds<T>    [min;max]
// IntVarBitset<T>    Vec<[lb;ub]>
// IntVarList<T>      Vec<T>
// IntVarIntervals<T> {offset; len; BitSet<>}

pub use self::bounds::{IntVarBounds, IntVarBoundsArray, IntVarBoundsRefArray};
pub use self::intervals::{IntVarIntervals, IntVarIntervalsArray, IntVarIntervalsRefArray};
pub use self::values::{IntVarValues, IntVarValuesArray, IntVarValuesRefArray};
pub use self::values::{IntVarBitset, IntVarBitsetArray, IntVarBitsetRefArray};

mod bounds;
mod intervals;
mod values;
mod bitset;
