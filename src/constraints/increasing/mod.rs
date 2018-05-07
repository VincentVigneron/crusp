use variables::Array;
use variables::domains::OrderedDomain;

constraint_build!(
    struct Propagator = propagator::IncreasingPropagator;
    fn new();
    fn propagate(x: ArrayOfVarsOfVarsOfVarsOfVarsOfVars)
        where
        VarType: OrderedDomain<Type=i32>,
        ArrayOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::Array;
    use variables::domains::OrderedDomain;
    #[derive(Debug, Clone)]
    pub struct IncreasingPropagator {}
    impl Propagator for IncreasingPropagator {}
    impl IncreasingPropagator {
        pub fn new() -> IncreasingPropagator {
            IncreasingPropagator {}
        }

        pub fn propagate<VarType, ArrayOfVarsOfVarsOfVarsOfVarsOfVars>(
            &self,
            array: &mut ArrayOfVarsOfVarsOfVarsOfVarsOfVars,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: OrderedDomain<Type = i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>,
        {
            use variables::VariableState;
            let mut change = false;
            let len = array.len();
            for i in 0..(len - 1) {
                unsafe {
                    let lhs: &mut VarType = array_get_mut!(array[i]);
                    let rhs: &mut VarType = array_get_mut!(array[i + 1]);
                    let res = lhs.less_than(rhs)?;
                    change = change
                        || (res != (VariableState::NoChange, VariableState::NoChange));
                }
            }
            for i in 0..(len - 1) {
                unsafe {
                    let lhs: &mut VarType = array_get_mut!(array[len - 2 - i]);
                    let rhs: &mut VarType = array_get_mut!(array[len - 1 - i]);
                    let res = lhs.less_than(rhs)?;
                    change = change
                        || (res != (VariableState::NoChange, VariableState::NoChange));
                }
            }
            if change {
                Ok(PropagationState::FixPoint)
            } else {
                Ok(PropagationState::NoChange)
            }
        }
    }
}
