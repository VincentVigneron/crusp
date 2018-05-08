use super::{ArrayView, VariableView, ViewIndex};

/// Represents a variables handler. Variable handlers can manage many type of variables
/// and give acces to statistics about each variables. A `VariablesHandler` does not
/// provide acces to variable, that's why each structure that implements a `VariableHandler`
/// should at least implements one `SpecificVariablesHandler`.
pub trait VariablesHandler: Clone {
    //fn retrieve_all_changed_states(
    //&mut self,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;

    //fn retrieve_changed_states<Views>(
    //&mut self,
    //views: Views,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>
    //where
    //Views: Iterator<Item = ViewIndex>;
}

pub trait VariablesHandlerBuilder<VarHandler: VariablesHandler> {
    fn finalize(self) -> VarHandler;
}

/// Gives immutable and mutable acces to owned variables. A `SpecificVariablesHandler`
/// gives access to only one type of variable.
/// The acces to a variable is done via a `View`. It's highly recommended to use one view for one
/// type of `Variable`. When a structure implements the `SpecificVariablesHandler` for
/// for a specific `Variable` `Var`, it should also implements the `SpecificVariablesHandler` trait for
/// an `ArrayOfVars<Var>` and an `ArrayOfRefs<Var>` (each of these three `SpecificVariablesHandler`
/// should have its own view).
///
/// * `Var` - The type of variable handled.
/// * `View` - The associated view for the variable.
pub trait SpecificVariablesHandler<View>: VariablesHandler
where
    View: VariableView + Into<ViewIndex> + 'static,
{
    fn get_mut(&mut self, view: &View) -> &mut View::Variable;
    fn get(&self, view: &View) -> &View::Variable;
    fn get_unique_id(&self, view: &View) -> ViewIndex;

    //fn retrieve_state(&mut self, view: &View) -> VariableState;
    //// Retrieve state of the view but also of the subiview
    //fn retrieve_states<'a, Views: Iterator<Item = &'a View>>(
    //&mut self,
    //views: Views,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    //fn retrieve_all_changed_states(
    //&mut self,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    //fn into_indexes(&self, &View) -> Box<Iterator<Item = ViewIndex>>;

    // fn iter(&self) -> &mut Variable;
}

pub trait SpecificVariablesHandlerBuilder<View, VarHandler, Param>
    : VariablesHandlerBuilder<VarHandler>
where
    View: VariableView + Into<ViewIndex> + 'static,
    VarHandler: SpecificVariablesHandler<View>,
{
    fn add(&mut self, views: Param) -> View;
}

pub trait SpecificArraysHandler<View>: VariablesHandler
where
    View: ArrayView + Into<ViewIndex> + 'static,
{
    fn get_mut(&mut self, view: &View) -> &mut View::Array;
    fn get(&self, view: &View) -> &View::Array;
    fn get_unique_id(&self, view: &View, position: usize) -> ViewIndex;
    fn get_unique_ids(&self, view: &View) -> Box<Iterator<Item = ViewIndex>>;

    //fn retrieve_state(&mut self, view: &View) -> VariableState;
    //// Retrieve state of the view but also of the subiview
    //fn retrieve_states<'a, Views: Iterator<Item = &'a View>>(
    //&mut self,
    //views: Views,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    //fn retrieve_all_changed_states(
    //&mut self,
    //) -> Box<Iterator<Item = (ViewIndex, VariableState)>>;
    //fn into_indexes(&self, &View) -> Box<Iterator<Item = ViewIndex>>;

    // fn iter(&self) -> &mut Variable;
}

pub trait SpecificArraysHandlerBuilder<View, VarHandler, Param>
    : VariablesHandlerBuilder<VarHandler>
where
    View: ArrayView + Into<ViewIndex> + 'static,
    VarHandler: SpecificArraysHandler<View>,
{
    fn add_array(&mut self, views: Param) -> View;
}

#[macro_export]
macro_rules! unsafe_get_mut_from_handler {
    ($variables: expr, $view: expr) => {
        &mut *(get_mut_from_handler($variables,&$view) as *mut _)
    }
}
pub fn get_mut_from_handler<'a, Handler, View>(
    vars: &'a mut Handler,
    view: &View,
) -> &'a mut View::Variable
where
    Handler: SpecificVariablesHandler<View>,
    View: VariableView + Into<ViewIndex> + 'static,
{
    vars.get_mut(&view)
}
pub fn get_from_handler<'a, Handler, View>(
    vars: &'a Handler,
    view: &View,
) -> &'a View::Variable
where
    Handler: SpecificVariablesHandler<View>,
    View: VariableView + Into<ViewIndex> + 'static,
{
    vars.get(&view)
}

#[macro_use]
pub mod macros;
pub mod default_handler;
