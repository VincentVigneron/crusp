use super::Constraint;
use variables::handlers::VariablesHandler;

pub trait ConstraintsHandler<H: VariablesHandler> {
    fn propagate_all(&mut self, &mut H);
    fn add(&mut self, Box<Constraint<H>>);
}

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
    fn propagate_all(&mut self, variables: &mut H) {
        for constraint in self.constraints.iter_mut() {
            constraint.propagate(variables);
        }
    }

    fn add(&mut self, constraint: Box<Constraint<H>>) {
        self.constraints.push(constraint);
    }
}
