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
    array: Views,
    output: Option<Vec<(ViewIndex, VariableState)>>,
}

impl<Var, Views> Increasing<Var, Views>
where
    Views: ArrayView,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new(array: Views) -> Increasing<Var, Views>
where {
        Increasing {
            array: array,
            output: None,
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
        self.output = None;
        let mut output = vec![];
        let len = { variables_handler.get_array_mut(&self.array).len() };
        for i in 0..(len - 1) {
            let (lhs, rhs) = unsafe {
                let array = variables_handler.get_array_mut(&self.array);
                let lhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i));
                let rhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i + 1));
                lhs.less_than(rhs)?
            };
            if lhs != VariableState::NoChange {
                let view = variables_handler.get_array_id(&self.array, i);
                output.push((view.into(), lhs));
            }
            if rhs != VariableState::NoChange {
                let view = variables_handler.get_array_id(&self.array, i + 1);
                output.push((view.into(), rhs));
            }
        }
        for i in 0..(len - 1) {
            let (lhs, rhs) = unsafe {
                let array = variables_handler.get_array_mut(&self.array);
                let lhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 2 - i));
                let rhs: &mut Views::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 1 - i));
                lhs.less_than(rhs)?
            };
            if lhs != VariableState::NoChange {
                let view = variables_handler.get_array_id(&self.array, len - 2 - i);
                output.push((view.into(), lhs));
            }
            if rhs != VariableState::NoChange {
                let view = variables_handler.get_array_id(&self.array, len - 1 - i);
                output.push((view.into(), rhs));
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
        // Do nothing.
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
            variables
                .get_array_ids(&self.array)
                .into_iter()
                .map(|val| (val, VariableState::ValuesChange)),
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
