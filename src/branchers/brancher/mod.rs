use super::{Brancher, SpecificBrancher, ValuesSelector, VariableSelector};
use std::marker::PhantomData;
use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

#[derive(Clone, Debug)]
pub struct DefaultBrancher<Handler, Var, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View> + 'static,
    Var: Variable + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, Var, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, Var, View> + Clone + 'static,
{
    variables_selector: VarSel,
    values_selector: ValSel,
    /// Mandatory
    phantom_handler: PhantomData<Handler>,
    phantom_var: PhantomData<Var>,
    phantom_view: PhantomData<View>,
}

impl<Handler, Var, View, VarSel, ValSel>
    DefaultBrancher<Handler, Var, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View> + 'static,
    Var: Variable + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, Var, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, Var, View> + Clone + 'static,
{
    pub fn new(
        variables_selector: VarSel,
        values_selector: ValSel,
    ) -> Option<DefaultBrancher<Handler, Var, View, VarSel, ValSel>> {
        Some(DefaultBrancher {
            variables_selector: variables_selector,
            values_selector: values_selector,
            phantom_handler: PhantomData,
            phantom_var: PhantomData,
            phantom_view: PhantomData,
        })
    }
}

impl<Handler, Var, View, VarSel, ValSel> Brancher<Handler>
    for DefaultBrancher<Handler, Var, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View> + 'static,
    Var: Variable + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, Var, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, Var, View> + Clone + 'static,
{
    fn branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        self.specific_branch(variables)
    }

    fn mutated_clone(&self) -> Box<Brancher<Handler>> {
        let ref_self: &DefaultBrancher<Handler, Var, View, VarSel, ValSel> = &self;
        let cloned: DefaultBrancher<Handler, Var, View, VarSel, ValSel> =
            <DefaultBrancher<Handler, Var, View, VarSel, ValSel> as Clone>::clone(
                ref_self,
            );

        Box::new(cloned) as Box<Brancher<Handler>>
    }
}

impl<Handler, Var, View, VarSel, ValSel> SpecificBrancher<Handler, Var, View>
    for DefaultBrancher<Handler, Var, View, VarSel, ValSel>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View> + 'static,
    Var: Variable + 'static,
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    VarSel: VariableSelector<Handler, Var, View> + Clone + 'static,
    ValSel: ValuesSelector<Handler, Var, View> + Clone + 'static,
{
    fn specific_branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        let variable = self.variables_selector.select(variables)?;
        self.values_selector.select(variables, variable)
    }
}
