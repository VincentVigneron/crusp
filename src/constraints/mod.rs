use variables::handlers::VariablesHandler;
use variables::{VariableError, VariableId, VariableState};

pub enum ConstraintState {
    Ready,
    NotReady,
}

pub enum PropagationState {
    FixPoint,
    Subsumed,
    NoChange,
}

pub trait ConstraintBuilder<Handler: VariablesHandler> {
    //fn finalize(
    //constraint_builder: Box<Self>,
    //variables_handler: &mut VariablesHandler,
    //) -> Box<Constraint<VariablesHandler>>;
}

/// Trait defining a constraint. The generic type is mandatory for the `Constraint`, even
/// if the `Constraint` depends more on its underlying variables. Indeed, the constraints
/// handlers store all the constraint the same, so the constraints are used as trait objects,
/// so they require to have the same type. Having the Handler as a generic parameter to the
/// propagate function will not allow to use `Constraint` as a trait object.
pub trait Constraint<Handler: VariablesHandler> {
    /// Constraints have to define `box_clone` in order to be cloned. The `ConstraintsHandler`
    /// handle many kind of variables, so it uses constraints as trait object. Trait object
    /// can not be cloned because `Clone` require the trait to be `Sized` but trait
    /// object can not be `Sized`. `box_clone` is a way to bypass this requirement, but
    /// `box_clone` can not guarantee that the clone is the same as the original, it is
    /// the `Constraint` developper to guarantee that the clone is equivalent to the original.
    /// Equivalent means to provide the same solution during the search not to be
    /// the same.
    fn box_clone(&self) -> Box<Constraint<Handler>>;
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError>;
    /// Initialisation should guarantee that the `Constraint` does not manipulate
    /// twice the same view. Since view of `ArrayOfRefs` does not hold any information
    /// about their underlying views (variables), it is necessary to ask this information
    /// to the variables handler. Indeed, `VariableView` has to implement the `Copy`
    /// trait. The structures implementing `Copy` must support a memcopy (i.e. the structure
    /// can be copy byte by byte and its size must be known at compile time), nevertheless ArrayOfRefs
    /// refers to non-contigous variables in memory so, in general, it requires to hold a dynamic
    /// array (Vec) of variables which the size is known at runtime.
    ///
    /// Initialise shoudl also register the variable dependencies. (?)
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError>;
    /// Prepares the `Constraint` by giving it its variables that have change since
    /// its last propagation.
    fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>);
    /// Asks the `Constraint` which variables it has modified after its last propagation.
    fn result(&mut self) -> Box<Iterator<Item = (VariableId, VariableState)>>;
    /// Asks the `Constraint` its variables dependency.
    fn dependencies(
        &self,
        variables_handler: &Handler,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>>;
}

impl<H: VariablesHandler> Clone for Box<Constraint<H>> {
    fn clone(&self) -> Box<Constraint<H>> {
        self.box_clone()
    }
}

pub trait PropagatorState {}
pub trait Propagator {}

mod all_different;
pub mod handlers;
pub use self::all_different::AllDifferent;
pub mod arithmetic;
mod increasing;
pub use self::increasing::Increasing;
mod regular;
mod sum;
pub use self::sum::SumConstraint;
