use constraints::Constraint;
use constraints::PropagationState;
use variables::{Array, ArrayView, Variable, VariableError, VariableState, ViewIndex};
use variables::domains::PrunableDomain;
use variables::handlers::{SpecificArraysHandler, VariablesHandler};

#[derive(Debug, Clone)]
pub struct AllDifferent<Var, Views>
where
    Views: ArrayView,
    Views::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    array: Views,
}

impl<Var, Views> AllDifferent<Var, Views>
where
    Views: ArrayView,
    Views::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    pub fn new(variables: Views) -> AllDifferent<Var, Views> {
        AllDifferent { array: variables }
    }
}

impl<Var, Views, Handler> Constraint<Handler> for AllDifferent<Var, Views>
where
    Handler: VariablesHandler + SpecificArraysHandler<Views> + Clone,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &AllDifferent<Var, Views> = &self;
        let cloned: AllDifferent<Var, Views> =
            <AllDifferent<Var, Views> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        use std::collections::BTreeSet;
        use variables::VariableState;
        let mut change = false;

        let vars = variables_handler.get_array_mut(&self.array);

        let affected: BTreeSet<Var> = vars.iter().filter_map(|var| var.value()).collect();
        let unaffected: Vec<_> = vars.iter()
            .enumerate()
            .map(|(i, var)| (i, var.value()))
            .filter(|&(_, ref var)| var.is_none())
            .map(|(i, _)| i)
            .collect();
        if unaffected.is_empty() {
            return Ok(PropagationState::Subsumed);
        }

        for i in unaffected.into_iter() {
            let var = vars.get_unchecked_mut(i);
            match var.remove_if(|val| affected.contains(&val))? {
                VariableState::NoChange => {}
                _ => {
                    change = true;
                }
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
