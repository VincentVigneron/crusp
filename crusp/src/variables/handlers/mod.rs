use super::VariableContainer;

/// Represents a variables handler. Variable handlers can manage many type of variables
/// and give acces to statistics about each variables. A `VariablesHandler` does not
/// provide acces to variable, that's why each structure that implements a `VariableHandler`
/// should at least implements one `SpecificVariablesHandler`.
pub trait VariablesHandler: Clone {}

pub trait VariablesHandlerBuilder<VarHandler: VariablesHandler> {
    fn new_builder() -> Self;
    fn finalize(self) -> VarHandler;
}

// Add a new trait VariableContainer
// Variable is a VariableContainer
// Array<Variable> is a VariableContainer
// ...
///// This trait design the view associated to a type of variable
///// managed by a `SpecificTypeHandler`.

/// Represents type holdings variables. A Variable is a VariableContainer because
/// its holds itsefl. An Array of Variables is a VariableContainer because it holds
/// a list of Variables.
pub trait VariableContainerView: Clone {
    ///// The `Type` managed by the Handler (Variable or Array of Variable).
    //type Container;
    ///// The type of `Variable` managed by the handler (Type == Variable if the Type is a variable).
    //type Variable: Variable;
}

/// Gives immutable and mutable acces to owned variables. A `SpecificVariablesHandler`
/// gives access to only one type of variable.
/// The acces to a variable is done via a `View`. It's highly recommended to use one view for one
/// type of `Variable`. When a structure implements the `SpecificVariablesHandler` for
/// for a specific `Variable` `Var`, it should also implements the `SpecificVariablesHandler` trait for
/// an `ArrayOfVars<Var>` and an `ArrayOfRefs<Var>` (each of these three `SpecificVariablesHandler`
/// should have its own view).
///
/// * `Var` - The type of variable handled.
/// * `View` - The associated view for the variable.
pub trait VariableContainerHandler<Var>
where
    Var: VariableContainer,
{
    type View: VariableContainerView;

    fn get_mut(&mut self, view: &Self::View) -> &mut Var;
    fn get(&self, view: &Self::View) -> &Var;
}

pub trait VariableContainerHandlerBuilder<Var, View, SpecificHandler, Param>
where
    Var: VariableContainer,
    View: VariableContainerView,
    SpecificHandler: VariableContainerHandler<Var, View = View>,
{
    fn add(&mut self, views: Param) -> View;
}

//pub fn get_mut_from_handler<'a, Handler, View>(
//vars: &'a mut Handler,
//view: &View,
//) -> &'a mut View::Container
//where
//Handler: VariableContainerHandler<View>,
//View: VariableContainerView,
//{
//vars.get_mut(&view)
//}
//pub fn get_from_handler<'a, Handler, View>(
//vars: &'a Handler,
//view: &View,
//) -> &'a View::Container
//where
//Handler: VariableContainerHandler<View>,
//View: VariableContainerView,
//{
//vars.get(&view)
//}

#[macro_use]
pub mod macros;
pub mod default_handler;

mod default {
    use variables::int_var::IntVarValuesBuilder;
    variables_handler_build!(IntVarValuesBuilder);
}
pub use self::default::Builder as DefaultVariablesBuilder;
pub use self::default::Handler as DefaultVariablesHandler;
