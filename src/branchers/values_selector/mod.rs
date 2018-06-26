use super::ValuesSelector;
use variables::domains::{AssignableDomain, IterableDomain, OrderedDomain};
use variables::handlers::{
    get_from_handler, get_mut_from_handler, VariableContainerHandler,
    VariableContainerView, VariablesHandler,
};
use variables::Variable;

#[derive(Clone, Debug)]
pub struct DomainOrderValueSelector {}

impl DomainOrderValueSelector {
    // Check variables empty and if no doublon
    pub fn new() -> DomainOrderValueSelector {
        DomainOrderValueSelector {}
    }
}

// Remove Into<VariableId> Requirement if possible (does not make sense).
impl<Handler, View> ValuesSelector<Handler, View> for DomainOrderValueSelector
where
    Handler: VariablesHandler + VariableContainerHandler<View> + 'static,
    View: VariableContainerView + Send + 'static,
    View::Container: Variable + AssignableDomain + IterableDomain + 'static,
    <View::Container as Variable>::Type: Send,
{
    // Error if no value
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> () + Send>>>, ()> {
        let var = get_from_handler(handler, &view);
        let branches: Vec<_> = var.iter()
            .cloned()
            .map(|val| (val, view.clone()))
            .map(move |(value, view)| {
                let patch: Box<Fn(&mut Handler) -> () + Send> =
                    Box::new(move |vars: &mut Handler| {
                        let var = get_mut_from_handler(vars, &view);
                        var.set_value(value.clone())
                            .expect("Should not happen DomainOrderValueSelector Fn.");
                    });
                patch
            })
            .collect();
        Ok(Box::new(branches.into_iter()))
    }
}

#[derive(Clone, Debug)]
pub struct MinValueSelector {}

impl MinValueSelector {
    // Check variables empty and if no doublon
    pub fn new() -> MinValueSelector {
        MinValueSelector {}
    }
}

// Remove Into<VariableId> Requirement if possible (does not make sense).
impl<Handler, View, Var> ValuesSelector<Handler, View> for MinValueSelector
where
    Handler: VariablesHandler + VariableContainerHandler<View> + 'static,
    View: VariableContainerView + Send + 'static,
    View::Container: Variable<Type = Var>
        + AssignableDomain
        + IterableDomain
        + OrderedDomain
        + 'static,
    Var: Ord + Eq + Clone + 'static,
    <View::Container as Variable>::Type: Send,
{
    // Error if no value
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> () + Send>>>, ()> {
        let var = get_from_handler(handler, &view);
        let mut values: Vec<_> = var.iter().cloned().collect();
        values.sort();
        let branches: Vec<_> = values
            .into_iter()
            .map(|val| (val, view.clone()))
            .map(move |(value, view)| {
                let patch: Box<Fn(&mut Handler) -> () + Send> =
                    Box::new(move |vars: &mut Handler| {
                        let var = get_mut_from_handler(vars, &view);
                        var.set_value(value.clone())
                            .expect("Should not happen DomainOrderValueSelector Fn.");
                    });
                patch
            })
            .collect();
        Ok(Box::new(branches.into_iter()))
    }
}
#[derive(Clone, Debug)]
pub struct MaxValueSelector {}

impl MaxValueSelector {
    // Check variables empty and if no doublon
    pub fn new() -> MaxValueSelector {
        MaxValueSelector {}
    }
}

// Remove Into<VariableId> Requirement if possible (does not make sense).
impl<Handler, View, Var> ValuesSelector<Handler, View> for MaxValueSelector
where
    Handler: VariablesHandler + VariableContainerHandler<View> + 'static,
    View: VariableContainerView + Send + 'static,
    View::Container: Variable<Type = Var>
        + AssignableDomain
        + IterableDomain
        + OrderedDomain
        + 'static,
    <View::Container as Variable>::Type: Send,
    Var: Ord + Eq + Clone + 'static,
{
    // Error if no value
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> () + Send>>>, ()> {
        let var = get_from_handler(handler, &view);
        let mut values: Vec<_> = var.iter().cloned().collect();
        values.sort();
        let branches: Vec<_> = values
            .into_iter()
            .rev()
            .map(|val| (val, view.clone()))
            .map(move |(value, view)| {
                let patch: Box<Fn(&mut Handler) -> () + Send> =
                    Box::new(move |vars: &mut Handler| {
                        let var = get_mut_from_handler(vars, &view);
                        var.set_value(value.clone())
                            .expect("Should not happen DomainOrderValueSelector Fn.");
                    });
                patch
            })
            .collect();
        Ok(Box::new(branches.into_iter()))
    }
}
