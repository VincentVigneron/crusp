use variables::VariableState;
use variables::handlers::VariablesHandler;

pub enum ConstraintState {
    Ready,
    NotReady,
}

// TODO Array State ?
// TODO returning tuple of state instead of a vec
// TODO Passing view to propator and variables handler? => proble with unit test

pub enum PropagationState {
    FixPoint,
    Subsumed,
    NoChange,
}

// TODO adding view as a parameter
pub enum PropagationError {
    DomainWipeout,
}

// TODO propagate without retrieving states
pub trait Constraint<H: VariablesHandler> {
    fn box_clone(&self) -> Box<Constraint<H>>;
    fn propagate(&mut self, &mut H) -> Result<PropagationState, PropagationError>;
    //fn try_propagate(&mut self, Arc<RefCell<H>>) -> ConstraintState;
}

impl<H: VariablesHandler> Clone for Box<Constraint<H>> {
    fn clone(&self) -> Box<Constraint<H>> {
        self.box_clone()
    }
}

pub trait PropagatorState {}
pub trait Propagator {}

#[macro_use]
pub mod macros;
pub mod handlers;
pub mod arithmetic;
pub mod increasing;
pub mod regular;
