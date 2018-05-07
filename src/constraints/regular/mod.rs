use variables::ArrayOfVars;
use variables::domains::PrunableDomain;

constraint_build!(
    struct Propagator = propagator::RegularPropagator;
    fn new();
    fn propagate(x: ArrayOfVars<VarType>) -> Option<propagator::RegularState>
        where VarType: PrunableDomain<Type = i32>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator};
    use variables::{ArrayOfVars, VariableError};
    use variables::domains::PrunableDomain;

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

        pub fn propagate<VarType: PrunableDomain<Type = i32>>(
            &self,
            _array: &mut ArrayOfVars<VarType>,
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
