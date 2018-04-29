use super::{Variable, VariableState, ViewIndex};

// ViewIndex or ViewIndexIndex
pub trait VariablesHandler: Clone {
    fn retrieve_all_changed_states(
        &mut self,
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;

    fn retrieve_changed_states<Views>(
        &mut self,
        views: Views,
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
    where
        Views: Iterator<Item = ViewIndex>;
}

pub trait VariablesHandlerBuilder<VarHandler: VariablesHandler> {
    fn finalize(self) -> VarHandler;
}

pub trait SpecificVariablesHandlerBuilder<Var, View, VarHandler, Param>
    : VariablesHandlerBuilder<VarHandler>
where
    Var: Variable,
    View: Into<ViewIndex> + 'static,
    VarHandler: SpecificVariablesHandler<Var, View>,
{
    fn add(&mut self, Param) -> View;
}

pub trait SpecificVariablesHandler<Var, View>: VariablesHandler
where
    Var: Variable,
    View: Into<ViewIndex> + 'static,
{
    fn get_mut(&mut self, &View) -> &mut Var;
    fn get(&self, &View) -> &Var;
    fn retrieve_state(&mut self, view: &View) -> VariableState;
    // Retrieve state of the view but also of the subiview
    fn retrieve_states<'a, Views: Iterator<Item = &'a View>>(
        &mut self,
        views: Views,
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    fn retrieve_all_changed_states(
        &mut self,
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    fn into_indexes(&self, &View) -> Box<Iterator<Item = ViewIndex>>;

    // fn iter(&self) -> &mut Variable;
}

pub fn get_mut_from_handler<'a, Handler, Var, View>(
    vars: &'a mut Handler,
    view: &View,
) -> &'a mut Var
where
    Handler: SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: Into<ViewIndex> + 'static,
{
    vars.get_mut(&view)
}
pub fn get_from_handler<'a, Handler, Var, View>(vars: &'a Handler, view: &View) -> &'a Var
where
    Handler: SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: Into<ViewIndex> + 'static,
{
    vars.get(&view)
}

#[macro_use]
pub mod macros;
pub mod default_handler;
