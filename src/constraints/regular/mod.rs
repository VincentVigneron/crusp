use constraints::Propagator;
use variables::Array;
use variables::int_var::ValuesIntVar;

constraint_build!(
    struct Propagator = RegularPropagator;
    fn new();
    fn propagate(x: Array<VarType>) -> Option<RegularState>
        where VarType: ValuesIntVar;
    );

#[derive(Debug, Clone)]
pub struct RegularState {}

#[derive(Debug, Clone)]
pub struct RegularPropagator {}

impl Propagator for RegularPropagator {}
impl RegularPropagator {
    pub fn new() -> RegularPropagator {
        RegularPropagator {}
    }

    pub fn propagate<VarType: ValuesIntVar>(
        &self,
        _array: &mut Array<VarType>,
        state: &mut Option<RegularState>,
    ) {
        if state.is_none() {
            *state = Some(RegularState {});
        }
    }
}
