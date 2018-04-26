use super::{Constraint, PropagationState};
use variables::VariableError;
use variables::handlers::VariablesHandler;

// TODO better namings for State and Error
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
        variables: &mut H,
    ) -> Result<PropagationState, VariableError> {
        for constraint in self.constraints.iter_mut() {
            let _ = constraint.propagate(variables)?;
        }
        Ok(PropagationState::Subsumed)
    }

    fn add(&mut self, constraint: Box<Constraint<H>>) {
        self.constraints.push(constraint);
    }
}
