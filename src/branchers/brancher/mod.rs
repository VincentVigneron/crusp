use super::{Brancher, SpecificBrancher, ValuesSelector, VariableSelector};
use std::marker::PhantomData;
use variables::{VariableView, ViewIndex};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

#[derive(Clone, Debug)]
pub struct DefaultBrancher<Handler, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, View> + Clone + 'static,
{
    variables_selector: VarSel,
    values_selector: ValSel,
    /// Mandatory
    phantom_handler: PhantomData<Handler>,
    phantom_view: PhantomData<View>,
}

impl<Handler, View, VarSel, ValSel> DefaultBrancher<Handler, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, View> + Clone + 'static,
{
    pub fn new(
        variables_selector: VarSel,
        values_selector: ValSel,
    ) -> Option<DefaultBrancher<Handler, View, VarSel, ValSel>> {
        Some(DefaultBrancher {
            variables_selector: variables_selector,
            values_selector: values_selector,
            phantom_handler: PhantomData,
            phantom_view: PhantomData,
        })
    }
}

impl<Handler, View, VarSel, ValSel> Brancher<Handler>
    for DefaultBrancher<Handler, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, View> + Clone + 'static,
{
    fn branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        self.specific_branch(variables)
    }

    fn mutated_clone(&self) -> Box<Brancher<Handler>> {
        let ref_self: &DefaultBrancher<Handler, View, VarSel, ValSel> = &self;
        let cloned: DefaultBrancher<Handler, View, VarSel, ValSel> =
            <DefaultBrancher<Handler, View, VarSel, ValSel> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Brancher<Handler>>
    }
}

impl<Handler, View, VarSel, ValSel> SpecificBrancher<Handler, View>
    for DefaultBrancher<Handler, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, View> + Clone + 'static,
{
    fn specific_branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        let variable = self.variables_selector.select(variables)?;
        self.values_selector.select(variables, variable)
    }
}
