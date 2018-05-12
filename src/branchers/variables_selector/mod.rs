use super::VariableSelector;
use variables::handlers::{
    get_from_handler, VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::Variable;

// Change vec to array require get_view inside VariableHandler
#[derive(Clone, Debug)]
pub struct SequentialVariableSelector<View>
where
    View: VariableContainerView,
{
    variables: Vec<View>,
}

impl<View> SequentialVariableSelector<View>
where
    View: VariableContainerView,
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

impl<Handler, View> VariableSelector<Handler, View> for SequentialVariableSelector<View>
where
    Handler: VariablesHandler + VariableContainerHandler<View>,
    View: VariableContainerView,
    View::Container: Variable,
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
