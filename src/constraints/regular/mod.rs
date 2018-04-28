use variables::Array;
use variables::int_var::ValuesIntVar;

constraint_build!(
    struct Propagator = propagator::RegularPropagator;
    fn new();
    fn propagate(x: Array<VarType>) -> Option<propagator::RegularState>
        where VarType: ValuesIntVar;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator};
    use variables::{Array, VariableError};
    use variables::int_var::ValuesIntVar;

    #[derive(Debug, Clone)]
    pub struct RegularState {}

    #[derive(Debug, Clone)]
    pub struct RegularPropagator {
        input: Option<RegularState>,
        output: Option<RegularState>,
    }

    impl Propagator for RegularPropagator {}
    impl RegularPropagator {
        pub fn new() -> RegularPropagator {
            RegularPropagator {
                input: None,
                output: None,
            }
        }

        pub fn prepare(&mut self, state: RegularState) {
            self.input = Some(state);
        }

        pub fn retrieve_state(&mut self) -> Option<RegularState> {
            use std::mem;
            let mut state = None;
            mem::swap(&mut state, &mut self.output);
            state
        }

        pub fn propagate<VarType: ValuesIntVar>(
            &self,
            _array: &mut Array<VarType>,
            state: &mut Option<RegularState>,
        ) -> Result<PropagationState, VariableError> {
            //self.output = self.intput;
            //self.input = None;
            if state.is_none() {
                *state = Some(RegularState {});
            }
            Ok(PropagationState::FixPoint)
        }
    }
}
