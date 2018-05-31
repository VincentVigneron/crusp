use constraints::{Constraint, PropagationState};
use variables::domains::{OrderedDomain, PrunableDomain};
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::{Variable, VariableError, VariableId, VariableState};

#[derive(Clone)]
pub struct Equal<Var, View>
where
    View: VariableContainerView,
    View::Container: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    lhs: View,
    rhs: View,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<Var, View> Equal<Var, View>
where
    View: VariableContainerView,
    View::Container: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    pub fn new(lhs: View, rhs: View) -> Equal<Var, View> {
        Equal {
            lhs: lhs,
            rhs: rhs,
            output: None,
        }
    }
}

impl<Var, View, Handler> Constraint<Handler> for Equal<Var, View>
where
    Handler: VariablesHandler + VariableContainerHandler<View> + Clone,
    View: VariableContainerView + 'static,
    View::Container: PrunableDomain<Type = Var>,
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
        let mut output = vec![];
        self.output = None;

        unsafe {
            let lhs: &mut View::Container =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Container =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));
            let (lhs_state, rhs_state) = lhs.equal(rhs)?;
            match lhs_state {
                VariableState::NoChange => {}
                state => {
                    output.push((lhs.id(), state));
                }
            }
            match rhs_state {
                VariableState::NoChange => {}
                state => {
                    output.push((rhs.id(), state));
                }
            }
        }

        if !output.is_empty() {
            self.output = Some(output);
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
    #[allow(unused)]
    fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>) {
        // Do nothing
    }
    fn result(&mut self) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        use std::mem;
        let mut res = None;
        mem::swap(&mut self.output, &mut res);
        match res {
            None => Box::new(vec![].into_iter()),
            Some(changes) => Box::new(changes.into_iter()),
        }
    }
    #[allow(unused)]
    fn dependencies(
        &self,
        variables: &Handler,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        Box::new(
            vec![
                (variables.get(&self.lhs).id(), VariableState::ValuesChange),
                (variables.get(&self.rhs).id(), VariableState::ValuesChange),
            ].into_iter(),
        )
    }
    #[allow(unused)]
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        self.propagate(variables_handler)
    }
}

#[derive(Clone)]
pub struct EqualBounds<Var, View>
where
    View: VariableContainerView,
    View::Container: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    lhs: View,
    rhs: View,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<Var, View> EqualBounds<Var, View>
where
    View: VariableContainerView,
    View::Container: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    pub fn new(lhs: View, rhs: View) -> EqualBounds<Var, View> {
        EqualBounds {
            lhs: lhs,
            rhs: rhs,
            output: None,
        }
    }
}

impl<Var, View, Handler> Constraint<Handler> for EqualBounds<Var, View>
where
    Handler: VariablesHandler + VariableContainerHandler<View> + Clone,
    View: VariableContainerView + 'static,
    View::Container: OrderedDomain<Type = Var>,
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
        let mut output = vec![];
        self.output = None;
        unsafe {
            let lhs: &mut View::Container =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Container =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));

            let state = lhs.weak_upperbound(rhs.unchecked_max())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((lhs.id(), state));
                }
            }
            let state = rhs.weak_upperbound(lhs.unchecked_max())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((rhs.id(), state));
                }
            }
            let state = lhs.weak_lowerbound(rhs.unchecked_min())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((lhs.id(), state));
                }
            }
            let state = rhs.weak_lowerbound(lhs.unchecked_min())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((rhs.id(), state));
                }
            }
        }
        if !output.is_empty() {
            self.output = Some(output);
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
    #[allow(unused)]
    fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>) {
        // Do nothing
    }
    fn result(&mut self) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        use std::mem;
        let mut res = None;
        mem::swap(&mut self.output, &mut res);
        match res {
            None => Box::new(vec![].into_iter()),
            Some(changes) => Box::new(changes.into_iter()),
        }
    }
    fn dependencies(
        &self,
        variables: &Handler,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        Box::new(
            vec![
                (variables.get(&self.lhs).id(), VariableState::ValuesChange),
                (variables.get(&self.rhs).id(), VariableState::ValuesChange),
            ].into_iter(),
        )
    }
    #[allow(unused)]
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        self.propagate(variables_handler)
    }
}
