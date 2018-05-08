use constraints::Constraint;
use constraints::PropagationState;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use variables::{Array, VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                          VariablesHandler};

#[derive(Clone)]
pub struct Increasing<Var, ArrayView>
where
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    variables: ArrayView,
}

impl<Var, ArrayView> Increasing<Var, ArrayView>
where
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new(variables: ArrayView) -> Increasing<Var, ArrayView>
where {
        Increasing {
            variables: variables,
        }
    }
}

impl<Var, ArrayView, Handler> Constraint<Handler> for Increasing<Var, ArrayView>
where
    Handler: VariablesHandler + SpecificVariablesHandler<ArrayView>,
    ArrayView: VariableView + Into<ViewIndex> + 'static,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: OrderedDomain<Type = Var>,
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
        let ref_self: &Increasing<Var, ArrayView> = &self;
        let cloned: Increasing<Var, ArrayView> =
            <Increasing<Var, ArrayView> as Clone>::clone(ref_self);

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
        let array = get_mut_from_handler(variables_handler, &self.variables);
        let len = array.len();
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut <ArrayView::Variable as Array>::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i));
                let rhs: &mut <ArrayView::Variable as Array>::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(i + 1));
                let res = lhs.less_than(rhs)?;
                change =
                    change || (res != (VariableState::NoChange, VariableState::NoChange));
            }
        }
        for i in 0..(len - 1) {
            unsafe {
                let lhs: &mut <ArrayView::Variable as Array>::Variable =
                    unsafe_from_raw_point!(array.get_unchecked_mut(len - 2 - i));
                let rhs: &mut <ArrayView::Variable as Array>::Variable =
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
