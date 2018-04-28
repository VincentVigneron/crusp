use super::ValuesSelector;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};
use variables::int_var::IntVar;

pub struct MinValueSelector {}

impl MinValueSelector {
    // Check variables empty and if no doublon
    pub fn new() -> MinValueSelector {
        MinValueSelector {}
    }
}

// Remove Into<ViewIndex> Requirement if possible (does not make sense).
impl<Handler, Var, View> ValuesSelector<Handler, Var, View> for MinValueSelector
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable + IntVar,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        unimplemented!()
    }
}
