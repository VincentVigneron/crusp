use variables::Array;
use variables::int_var::ValuesIntVar;

constraint_build!(
    struct Propagator = propagator::RegularPropagator;
    fn new();
    fn propagate(x: Array<VarType>) -> Option<propagator::RegularState>
        where VarType: ValuesIntVar;
    );

pub mod propagator {
    use constraints::{PropagationError, PropagationState, Propagator};
    use variables::Array;
    use variables::int_var::ValuesIntVar;

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
        ) -> Result<PropagationState, PropagationError> {
            if state.is_none() {
                *state = Some(RegularState {});
            }
            Ok(PropagationState::FixPoint(vec![]))
        }
    }
}
