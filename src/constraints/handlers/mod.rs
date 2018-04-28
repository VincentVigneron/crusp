use super::{Constraint, PropagationState};
use variables::VariableError;
use variables::handlers::VariablesHandler;

pub trait ConstraintsHandler<H: VariablesHandler>: Clone {
    fn propagate_all(&mut self, &mut H) -> Result<PropagationState, VariableError>;
    fn add(&mut self, Box<Constraint<H>>);
}

#[derive(Clone)]
pub struct SequentialConstraintsHandler<H: VariablesHandler> {
    constraints: Vec<Box<Constraint<H>>>,
}

impl<H: VariablesHandler> SequentialConstraintsHandler<H> {
    pub fn new() -> SequentialConstraintsHandler<H> {
        SequentialConstraintsHandler {
            constraints: Vec::new(),
        }
    }
}

impl<H: VariablesHandler> ConstraintsHandler<H> for SequentialConstraintsHandler<H> {
    fn propagate_all(
        &mut self,
        variables_handler: &mut H,
    ) -> Result<PropagationState, VariableError> {
        // Option<ConstraintState> instead
        let mut change = false;
        for constraint in self.constraints.iter_mut() {
            let has_change = match constraint.propagate(variables_handler)? {
                PropagationState::FixPoint => true,
                PropagationState::Subsumed => true,
                PropagationState::NoChange => false,
            };
            change = change || has_change;
        }
        if !change {
            return Ok(PropagationState::FixPoint);
        }
        let mut variables_states: Vec<_> =
            variables_handler.retrieve_all_changed_states().collect();
        while change {
            change = false;
            for constraint in self.constraints.iter_mut() {
                let mut states = variables_states.iter();
                if constraint.affected_by_changes(&mut states) {
                    let has_change = match constraint.propagate(variables_handler)? {
                        PropagationState::FixPoint => true,
                        PropagationState::Subsumed => true,
                        PropagationState::NoChange => false,
                    };
                    change = change || has_change;
                }
            }
            if !change {
                return Ok(PropagationState::FixPoint);
            }
            variables_states = variables_handler.retrieve_all_changed_states().collect();
        }
        //unreachable!()
        Ok(PropagationState::FixPoint)
    }

    fn add(&mut self, constraint: Box<Constraint<H>>) {
        self.constraints.push(constraint);
    }
}
