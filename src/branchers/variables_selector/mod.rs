use super::VariableSelector;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{get_from_handler, SpecificVariablesHandler, VariablesHandler};

pub struct SequentialVariableSelector<View>
where
    View: VariableView + Into<ViewIndex> + Clone,
{
    variables: Vec<View>,
}
impl<View> SequentialVariableSelector<View>
where
    View: VariableView + Into<ViewIndex> + Clone,
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

impl<View> VariableSelector<View> for SequentialVariableSelector<View>
where
    View: VariableView + Into<ViewIndex> + Clone + 'static,
{
    fn select<Handler, Var>(&mut self, handler: &Handler) -> Result<View, ()>
    where
        Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
        Var: Variable,
    {
        self.variables
            .iter()
            .filter(|&view| {
                let var = get_from_handler(handler, view);
                !var.is_fixed()
            })
            .cloned()
            .next()
            .ok_or(())
    }
}
