use constraints::Constraint;
use constraints::PropagationState;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use variables::domains::OrderedDomain;
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::{Array, Variable, VariableError, VariableId, VariableState};

#[derive(Clone)]
pub struct Increasing<VarType, Views>
where
    Views: VariableContainerView,
    VarType: Ord + Eq + Clone,
{
    array: Views,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<VarType, Views> Increasing<VarType, Views>
where
    Views: VariableContainerView,
    VarType: Ord + Eq + Clone,
{
    pub fn new(array: Views) -> Increasing<VarType, Views>
where {
        Increasing {
            array: array,
            output: None,
        }
    }
}

impl<Var, ArrayVar, VarType, Views, Handler> Constraint<Handler>
    for Increasing<VarType, Views>
where
    Handler: VariablesHandler + VariableContainerHandler<ArrayVar>,
    Views: VariableContainerView + 'static,
    ArrayVar: Array<Variable = Var>,
    Var: OrderedDomain<Type = VarType>,
    VarType: Ord
        + Eq
        + Add<Output = VarType>
        + Sub<Output = VarType>
        + Mul<Output = VarType>
        + Div<Output = VarType>
        + Sum<VarType>
        + Clone
        + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &Increasing<VarType, Views> = &self;
        let cloned: Increasing<VarType, Views> =
            <Increasing<VarType, Views> as Clone>::clone(ref_self);

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
        let len = { variables_handler.get(&self.array).len() };
        for i in 0..(len - 1) {
            unsafe {
                let array: &mut ArrayVar = variables_handler.get_mut(&self.array);
                let lhs: &mut Var = unsafe_from_raw_point!(array.get_unchecked_mut(i));
                let rhs: &mut Var =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i + 1));
                let (lhs_state, rhs_state) = lhs.less_than(rhs)?;
                if lhs_state != VariableState::NoChange {
                    output.push((lhs.id(), lhs_state));
                }
                if rhs_state != VariableState::NoChange {
                    output.push((rhs.id(), rhs_state));
                }
            };
        }
        for i in 0..(len - 1) {
            unsafe {
                let array: &mut ArrayVar = variables_handler.get_mut(&self.array);
                let lhs: &mut Var =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 2 - i));
                let rhs: &mut Var =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 1 - i));
                let (lhs_state, rhs_state) = lhs.less_than(rhs)?;
                if lhs_state != VariableState::NoChange {
                    output.push((lhs.id(), lhs_state));
                }
                if rhs_state != VariableState::NoChange {
                    output.push((rhs.id(), rhs_state));
                }
            };
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
        // Do nothing.
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
        variables_handler: &Handler,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        let deps: Vec<_> = variables_handler
            .get(&self.array)
            .iter()
            .map(|var| (var.id(), VariableState::ValuesChange))
            .collect();
        Box::new(deps.into_iter())
    }
    #[allow(unused)]
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        self.propagate(variables_handler)
    }
}
