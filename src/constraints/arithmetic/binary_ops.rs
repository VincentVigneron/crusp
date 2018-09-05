use constraints::{Constraint, PropagationState};
use std::ops::{Add, Sub};
use variables::domains::{IterableDomain, PrunableDomain};
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::{Variable, VariableError, VariableId, VariableState};

#[derive(Clone)]
pub struct AddConstant<VarType, View>
where
    View: VariableContainerView,
    VarType: Eq + Ord + Clone + 'static,
{
    res: View,
    var: View,
    coef: VarType,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<VarType, View> AddConstant<VarType, View>
where
    View: VariableContainerView,
    VarType: Eq + Ord + Clone + 'static,
{
    pub fn new(res: View, var: View, coef: VarType) -> AddConstant<VarType, View> {
        AddConstant {
            res: res,
            var: var,
            coef: coef,
            output: None,
        }
    }
}

use std::fmt::Debug;

impl<Var, VarType, View, Handler> Constraint<Handler> for AddConstant<VarType, View>
where
    Handler: VariablesHandler + VariableContainerHandler<View> + Clone,
    View: VariableContainerView + 'static,
    Var: PrunableDomain<Type = VarType> + IterableDomain + Debug,
    VarType: Eq
        + Ord
        + Clone
        + 'static
        + Add<Output = VarType>
        + Sub<Output = VarType>
        + Debug,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &AddConstant<VarType, View> = &self;
        let cloned: AddConstant<VarType, View> =
            <AddConstant<VarType, View> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        let mut output = vec![];
        self.output = None;

        unsafe {
            let res: &mut Var =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.res));
            let var: &mut Var =
                unsafe_from_raw_point!(variables_handler.get_mut(&self.var));
            let domain: Vec<_> = var.iter()
                .cloned()
                .map(|var| var + self.coef.clone())
                .collect();
            let state = res.in_values(domain.into_iter())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((res.id(), state));
                }
            }
            let domain: Vec<_> = res.iter()
                .cloned()
                .map(|res| res - self.coef.clone())
                .collect();
            let state = var.in_values(domain.into_iter())?;
            match state {
                VariableState::NoChange => {}
                state => {
                    output.push((var.id(), state));
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
                (variables.get(&self.res).id(), VariableState::ValuesChange),
                (variables.get(&self.var).id(), VariableState::ValuesChange),
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
