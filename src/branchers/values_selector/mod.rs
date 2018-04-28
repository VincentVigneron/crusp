use super::ValuesSelector;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{get_from_handler, get_mut_from_handler,
                          SpecificVariablesHandler, VariablesHandler};
use variables::int_var::{IntVar, ValuesIntVar};

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
    Var: ValuesIntVar,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        let var = get_from_handler(handler, &view);
        let min = var.min();
        let patch = Box::new(move |vars: &mut Handler| {
            let var: &mut Var = get_mut_from_handler(vars, &view);
            unsafe {
                var.set_value(min);
            }
        });
        unimplemented!()
    }
}
