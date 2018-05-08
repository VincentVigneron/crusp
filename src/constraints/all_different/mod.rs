use constraints::Constraint;
use constraints::PropagationState;
use variables::{Array, Variable, VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::PrunableDomain;
use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                          VariablesHandler};

#[derive(Debug, Clone)]
pub struct AllDifferent<Var, ArrayView>
where
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    array: ArrayView,
}

impl<Var, ArrayView> AllDifferent<Var, ArrayView>
where
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    pub fn new(variables: ArrayView) -> AllDifferent<Var, ArrayView> {
        AllDifferent { array: variables }
    }
}

impl<Var, ArrayView, Handler> Constraint<Handler> for AllDifferent<Var, ArrayView>
where
    Handler: VariablesHandler + SpecificVariablesHandler<ArrayView> + Clone,
    ArrayView: VariableView + Into<ViewIndex> + 'static,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone + 'static,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &AllDifferent<Var, ArrayView> = &self;
        let cloned: AllDifferent<Var, ArrayView> =
            <AllDifferent<Var, ArrayView> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        use std::collections::BTreeSet;
        use variables::VariableState;
        let mut change = false;

        let vars = get_mut_from_handler(variables_handler, &self.array);

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
