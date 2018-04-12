use super::{Variable, VariableView};

pub trait VariablesHandler: Clone {}

pub trait VariablesHandlerBuilder<VarHandler: VariablesHandler> {
    fn finalize(self) -> VarHandler;
}

pub trait SpecificVariablesHandlerBuilder<Var, View, VarHandler>
    : VariablesHandlerBuilder<VarHandler>
where
    Var: Variable,
    View: VariableView,
    VarHandler: SpecificVariablesHandler<Var, View>,
{
    fn add(&mut self, Var) -> View;
}

// TODO get_mut
pub trait SpecificVariablesHandler<Var, View>: VariablesHandler
where
    Var: Variable,
    View: VariableView,
{
    fn get_mut(&mut self, &View) -> &mut Var;
}

pub fn get_mut_from_handler<'a, Handler, Var, View>(
    vars: &'a mut Handler,
    view: &View,
) -> &'a mut Var
where
    Handler: SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: VariableView,
{
    vars.get_mut(&view)
}

#[macro_use]
pub mod macros;
pub mod default_handler;
