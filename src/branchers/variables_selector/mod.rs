use super::VariableSelector;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{get_from_handler, SpecificVariablesHandler, VariablesHandler};

#[derive(Clone, Debug)]
pub struct SequentialVariableSelector<View>
where
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    variables: Vec<View>,
}

impl<View> SequentialVariableSelector<View>
where
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    // Check variables empty and if no doublon
    pub fn new<Views: Iterator<Item = View>>(
        variables: Views,
    ) -> Result<SequentialVariableSelector<View>, ()> {
        Ok(SequentialVariableSelector {
            variables: variables.collect(),
        })
    }
}

impl<Handler, Var, View> VariableSelector<Handler, Var, View>
    for SequentialVariableSelector<View>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    fn select(&mut self, handler: &Handler) -> Result<View, ()> {
        self.variables
            .iter()
            .filter(|&view| {
                let var = get_from_handler(handler, view);
                !var.is_affected()
            })
            .cloned()
            .next()
            .ok_or(())
    }
}
