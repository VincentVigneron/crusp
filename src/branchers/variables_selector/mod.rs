use super::VariableSelector;
use variables::domains::FiniteDomain;
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
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

impl<Var, Handler, View> VariableSelector<Handler, View>
    for SequentialVariableSelector<View>
where
    Handler: VariablesHandler + VariableContainerHandler<Var, View = View>,
    View: VariableContainerView,
    Var: Variable,
{
    fn select(&mut self, handler: &Handler) -> Result<View, ()> {
        self.variables
            .iter()
            .filter(|&view| {
                let var = handler.get(view);
                !var.is_affected()
            })
            .cloned()
            .next()
            .ok_or(())
    }
}

// Change vec to array require get_view inside VariableHandler
#[derive(Clone, Debug)]
pub struct SmallestDomainVariableSelector<View>
where
    View: VariableContainerView,
{
    variables: Vec<View>,
}

impl<View> SmallestDomainVariableSelector<View>
where
    View: VariableContainerView,
{
    // Check variables empty and if no doublon
    pub fn new<Views: Iterator<Item = View>>(
        variables: Views,
    ) -> Result<SmallestDomainVariableSelector<View>, ()> {
        Ok(SmallestDomainVariableSelector {
            variables: variables.collect(),
        })
    }
}

impl<Var, Handler, View> VariableSelector<Handler, View>
    for SmallestDomainVariableSelector<View>
where
    Handler: VariablesHandler + VariableContainerHandler<Var, View = View>,
    View: VariableContainerView,
    Var: Variable + FiniteDomain,
{
    fn select(&mut self, handler: &Handler) -> Result<View, ()> {
        let mut variables = self.variables
            .iter()
            .map(|view| {
                let var = handler.get(&view);
                (view, var.size())
            })
            .filter(|&(_, dom)| dom > 1)
            .map(|(view, dom)| (view.clone(), dom))
            .collect::<Vec<_>>();
        variables.sort_by_key(|&(_, dom)| dom);
        variables.first().map(|(view, _)| view.clone()).ok_or(())
    }
}
