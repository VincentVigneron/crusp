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
    output: Option<Vec<(ViewIndex, VariableState)>>,
}

impl<Var, View> Equal<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: PrunableDomain<Type = Var>,
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
        let mut output = vec![];
        self.output = None;

        unsafe {
            let lhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));
            let (lhs, rhs) = lhs.equal(rhs)?;
            match lhs {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.lhs), state));
                }
            }
            match rhs {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.rhs), state));
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
    fn prepare(&mut self, states: Box<Iterator<Item = ViewIndex>>) {
        // Do nothing
    }
    fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
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
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        Box::new(
            vec![
                (
                    variables.get_variable_id(&self.lhs),
                    VariableState::ValuesChange,
                ),
                (
                    variables.get_variable_id(&self.rhs),
                    VariableState::ValuesChange,
                ),
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
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    lhs: View,
    rhs: View,
    output: Option<Vec<(ViewIndex, VariableState)>>,
}

impl<Var, View> EqualBounds<Var, View>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
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
        let mut output = vec![];
        self.output = None;
        unsafe {
            let lhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
            let rhs: &mut View::Variable =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));

            let state = lhs.weak_upperbound(rhs.max())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.lhs), state));
                }
            }
            let state = rhs.weak_upperbound(lhs.max())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.rhs), state));
                }
            }
            let state = lhs.weak_lowerbound(rhs.min())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.lhs), state));
                }
            }
            let state = rhs.weak_lowerbound(lhs.min())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((variables_handler.get_variable_id(&self.rhs), state));
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
    fn prepare(&mut self, states: Box<Iterator<Item = ViewIndex>>) {
        // Do nothing
    }
    fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
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
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        Box::new(
            vec![
                (
                    variables.get_variable_id(&self.lhs),
                    VariableState::ValuesChange,
                ),
                (
                    variables.get_variable_id(&self.rhs),
                    VariableState::ValuesChange,
                ),
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
