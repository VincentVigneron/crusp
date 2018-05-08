use constraints::{Constraint, PropagationState};
use variables::{VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::{OrderedDomain, PrunableDomain};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

#[derive(Clone)]
pub struct Equal<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    lhs: View,
    rhs: View,
}

impl<Var, View> Equal<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    pub fn new(lhs: View, rhs: View) -> Equal<Var, View> {
        Equal { lhs: lhs, rhs: rhs }
    }
}

impl<Var, View, Handler> Constraint<Handler> for Equal<Var, View>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + Clone,
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &Equal<Var, View> = &self;
        let cloned: Equal<Var, View> = <Equal<Var, View> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        unsafe {
            let lhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));
            let r = lhs.equal(rhs)?;
            change = change || (r != (VariableState::NoChange, VariableState::NoChange));
        }

        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
    #[allow(unused)]
    fn prepare(&mut self, states: Box<Iterator<Item = (ViewIndex, VariableState)>>) {
        unimplemented!()
    }
    fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        unimplemented!()
    }
    fn dependencies(&self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        unimplemented!()
    }
    #[allow(unused)]
    fn initialise(&mut self, variables_handler: &mut Handler) -> Result<(), ()> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct EqualBounds<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    lhs: View,
    rhs: View,
}

impl<Var, View> EqualBounds<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    pub fn new(lhs: View, rhs: View) -> EqualBounds<Var, View> {
        EqualBounds { lhs: lhs, rhs: rhs }
    }
}

impl<Var, View, Handler> Constraint<Handler> for EqualBounds<Var, View>
where
    Handler: VariablesHandler + SpecificVariablesHandler<View> + Clone,
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &EqualBounds<Var, View> = &self;
        let cloned: EqualBounds<Var, View> =
            <EqualBounds<Var, View> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        unsafe {
            let lhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));

            let r = lhs.weak_upperbound(rhs.max())?;
            change = change || (r != VariableState::NoChange);
            let r = rhs.weak_upperbound(lhs.max())?;
            change = change || (r != VariableState::NoChange);
            let r = lhs.weak_lowerbound(rhs.min())?;
            change = change || (r != VariableState::NoChange);
            let r = rhs.weak_lowerbound(lhs.min())?;
            change = change || (r != VariableState::NoChange);
        }
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
    #[allow(unused)]
    fn prepare(&mut self, states: Box<Iterator<Item = (ViewIndex, VariableState)>>) {
        unimplemented!()
    }
    fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        unimplemented!()
    }
    fn dependencies(&self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        unimplemented!()
    }
    #[allow(unused)]
    fn initialise(&mut self, variables_handler: &mut Handler) -> Result<(), ()> {
        unimplemented!()
    }
}
