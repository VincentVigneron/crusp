use super::VariableSelector;
use std::marker::PhantomData;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{get_from_handler, SpecificVariablesHandler, VariablesHandler};

pub struct SequentialVariableSelector<Handler, Var, View>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    variables: Vec<View>,
    phantom_handler: PhantomData<Handler>,
    phantom_var: PhantomData<Var>,
}

impl<Handler, Var, View> SequentialVariableSelector<Handler, Var, View>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    // Check variables empty and if no doublon
    pub fn new<Views: Iterator<Item = View>>(
        variables: Views,
    ) -> Result<SequentialVariableSelector<Handler, Var, View>, ()> {
        Ok(SequentialVariableSelector {
            variables: variables.collect(),
            phantom_handler: PhantomData,
            phantom_var: PhantomData,
        })
    }
}

impl<Handler, Var, View> VariableSelector<Handler, Var, View>
    for SequentialVariableSelector<Handler, Var, View>
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
                !var.is_fixed()
            })
            .cloned()
            .next()
            .ok_or(())
    }
}
