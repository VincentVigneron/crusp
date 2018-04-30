use variables::List;
use variables::int_var::BoundsIntVar;

constraint_build!(
    struct Propagator = propagator::IncreasingPropagator;
    fn new();
    fn propagate(x: Array)
        where
        VarType: BoundsIntVar<Type=i32>,
        Array: List<VarType>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::List;
    use variables::int_var::BoundsIntVar;
    #[derive(Debug, Clone)]
    pub struct IncreasingPropagator {}
    impl Propagator for IncreasingPropagator {}
    impl IncreasingPropagator {
        pub fn new() -> IncreasingPropagator {
            IncreasingPropagator {}
        }

        pub fn propagate<VarType, Array>(
            &self,
            array: &mut Array,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: BoundsIntVar<Type = i32>,
            Array: List<VarType>,
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
