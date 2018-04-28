use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

pub mod handlers;
pub mod variables_selector;
pub mod values_selector;

pub trait VariableSelector<Handler, Var, View>
where
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
{
    fn select(&mut self, handler: &Handler) -> Result<View, ()>;
}

pub trait ValuesSelector<Handler, Var, View>
where
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
{
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()>;
}

pub trait Brancher<Handler, Var, View>
where
    View: VariableView + Clone + Into<ViewIndex> + 'static,
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
{
    fn branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()>;
}
