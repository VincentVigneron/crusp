use super::ValuesSelector;
use variables::{Variable, VariableView, ViewIndex};
use variables::domains::{AssignableDomain, IterableDomain};
use variables::handlers::{get_from_handler, get_mut_from_handler,
                          SpecificVariablesHandler, VariablesHandler};

#[derive(Clone, Debug)]
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
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View> + 'static,
    Var: Variable + AssignableDomain + IterableDomain + 'static,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    // Error if no value
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        let var = get_from_handler(handler, &view);
        let branches: Vec<_> = var.iter()
            .cloned()
            .map(|val| (val, view.clone()))
            .map(move |(value, view)| {
                let patch: Box<Fn(&mut Handler) -> ()> =
                    Box::new(move |vars: &mut Handler| {
                        let var: &mut Var = get_mut_from_handler(vars, &view);
                        var.set_value(value.clone())
                            .expect("Should not happen MinValueSelector Fn.");
                    });
                patch
            })
            .collect();
        Ok(Box::new(branches.into_iter()))
    }
}
