use variables::Array;
use variables::int_var::ValuesIntVar;

constraint_build!(
    struct Propagator = propagator::AllDifferentPropagator;
    fn new();
    fn propagate(vars: ArrayOfVarsOfVarsOfVarsOfVarsOfVars)
        where
            VarType: ValuesIntVar<Type=i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::Array;
    use variables::int_var::ValuesIntVar;

    #[derive(Debug, Clone)]
    pub struct AllDifferentPropagator {}
    impl Propagator for AllDifferentPropagator {}
    impl AllDifferentPropagator {
        pub fn new() -> AllDifferentPropagator {
            AllDifferentPropagator {}
        }

        pub fn propagate<VarType, ArrayOfVarsOfVarsOfVarsOfVarsOfVars>(
            &self,
            vars: &mut ArrayOfVarsOfVarsOfVarsOfVarsOfVars,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: ValuesIntVar<Type = i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>,
        {
            use std::collections::BTreeSet;
            use variables::VariableState;
            let mut change = false;

            let affected: BTreeSet<_> =
                vars.iter().filter_map(|var| var.value()).collect();
            let unaffected: Vec<_> = vars.iter()
                .enumerate()
                .map(|(i, var)| (i, var.value()))
                .filter(|&(_, ref var)| var.is_none())
                .map(|(i, _)| i)
                .collect();
            if unaffected.is_empty() {
                return Ok(PropagationState::Subsumed);
            }

            for i in unaffected.into_iter() {
                let var = vars.get_unchecked_mut(i);
                match var.remove_if(|val| affected.contains(&val))? {
                    VariableState::NoChange => {}
                    _ => {
                        change = true;
                    }
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
