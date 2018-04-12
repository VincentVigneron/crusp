use std::cell::RefCell;
use std::sync::Arc;
use variables::handlers::VariablesHandler;

pub enum ConstraintState {
    Ready,
    NotReady,
}

pub trait Constraint<H: VariablesHandler> {
    fn propagate(&mut self, &mut H);
    fn try_propagate(&mut self, Arc<RefCell<H>>) -> ConstraintState;
}

pub trait PropagatorState {}
pub trait Propagator {}

#[macro_use]
pub mod macros;
pub mod handlers;
pub mod arithmetic;
pub mod increasing;
