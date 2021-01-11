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


struct Graph<ConstraintId, Mode, VariableId, State, Event> {
    cst_vertices: Vec<(ConstraintId,Mode)>,
    var_vertices: Vec<VariableId>,
    arcs: From(VID,Event) => To(CID,MODE),

    trans: HashMap<VID, Events => TO>

    cst_mode: Vec<Mode>
    cst_costs: Vec<i64>,
    events: PriorityQueue<CID>,
}

pub trait Event {
    fn none() -> Self;
}


// From u8 or from u32 for out event
impl Constraint {
    fn propagate(Mode mode) -> () {
        // how to generate thattt
        match mode ? {
            mode1 => propagate1???
            mode2 => propagate2???
            mode3 => propagate3???
        }

    }

    // events update graph of events and potentially stats
    fn propagate(&mut self, events: &mut events, _mode: Mode) -> () {
        self.cst.propagate(events,...)
    }
}

impl Graph {
    fn add_event(&mut self, vid, event) => do_smthg {
        for cid in events_of((vid, event)) {
            let new_cst = self.update(cid, event);
            if new_cst {
                events.push(cid, cost[cid]);
            }
        }
    }

    fn update(&mut self, cid, event) -> bool {
        let mode = cst_mode.get_mut();
        let ret = mode.is_none();
        mode = mode | event;
    }

    fn peek(&mut self) -> Option<Event> {
        let event = events.peek();
        match event {
            Some(cid) => {
                let mode = cst_mode[cid];
                cst_mode = none;
                Some((cid,mode))
            },
            None => None,
        }
    }
}
