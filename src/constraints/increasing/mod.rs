use constraints::Constraint;
use constraints::PropagationState;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use variables::{Array, ArrayView, VariableError, VariableState, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{SpecificArraysHandler, VariablesHandler};

#[derive(Clone)]
pub struct Increasing<Var, Views>
where
    Views: ArrayView,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    variables: Views,
}

impl<Var, Views> Increasing<Var, Views>
where
    Views: ArrayView,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new(variables: Views) -> Increasing<Var, Views>
where {
        Increasing {
            variables: variables,
        }
    }
}

impl<Var, Views, Handler> Constraint<Handler> for Increasing<Var, Views>
where
    Handler: VariablesHandler + SpecificArraysHandler<Views>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord
        + Eq
        + Add<Output = Var>
        + Sub<Output = Var>
        + Mul<Output = Var>
        + Div<Output = Var>
        + Sum<Var>
        + Clone
        + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &Increasing<Var, Views> = &self;
        let cloned: Increasing<Var, Views> =
            <Increasing<Var, Views> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }

    // adding to propagator/constraint information about change view
    // add iter to array and size => len
    // [HarveySchimpf02]
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        use variables::VariableState;
        let mut change = false;
        let array = variables_handler.get_mut(&self.variables);
        let len = array.len();
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i));
                let rhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i + 1));
                let res = lhs.less_than(rhs)?;
                change =
                    change || (res != (VariableState::NoChange, VariableState::NoChange));
            }
        }
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 2 - i));
                let rhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 1 - i));
                let res = lhs.less_than(rhs)?;
                change =
                    change || (res != (VariableState::NoChange, VariableState::NoChange));
            }
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
