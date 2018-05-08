use constraints::Constraint;
use constraints::PropagationState;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use variables::{Array, VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                          VariablesHandler};

#[derive(Clone)]
pub struct SumConstraint<Var, View, ArrayView>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    res: View,
    variables: ArrayView,
    coefs: Vec<Var>,
}

impl<Var, View, ArrayView> SumConstraint<Var, View, ArrayView>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    ArrayView: VariableView,
    ArrayView::Variable: Array,
    <ArrayView::Variable as Array>::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new<Coefs>(
        res: View,
        variables: ArrayView,
        coefs: Coefs,
    ) -> SumConstraint<Var, View, ArrayView>
    where
        Coefs: IntoIterator<Item = Var>,
    {
        SumConstraint {
            res: res,
            variables: variables,
            coefs: coefs.into_iter().collect(),
        }
    }
}

impl<Var, View, ArrayView, Handler> Constraint<Handler>
    for SumConstraint<Var, View, ArrayView>
where
    Handler: VariablesHandler
        + SpecificVariablesHandler<View>
        + SpecificVariablesHandler<ArrayView>,
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
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
        let ref_self: &SumConstraint<Var, View, ArrayView> = &self;
        let cloned: SumConstraint<Var, View, ArrayView> =
            <SumConstraint<Var, View, ArrayView> as Clone>::clone(ref_self);

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

        let vars: &mut ArrayView::Variable = unsafe {
            unsafe_from_raw_point!(get_mut_from_handler(
                variables_handler,
                &self.variables
            ))
        };
        let res: &mut View::Variable = unsafe {
            unsafe_from_raw_point!(get_mut_from_handler(variables_handler, &self.res))
        };

        let _contributions: Vec<_> = vars.iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * (var.max() - var.min()))
            .collect();
        let min: Var = vars.iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * var.min())
            .sum();
        let max: Var = vars.iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * var.max())
            .sum();
        let r = res.weak_upperbound(max)?;

        change = change || (r != VariableState::NoChange);
        let r = res.weak_lowerbound(min.clone())?;
        change = change || (r != VariableState::NoChange);

        let f = res.max() - min;
        //if f < 0 {
        //return Err(VariableError::DomainWipeout);
        //}
        let vars = vars.iter_mut().zip(self.coefs.iter());
        for (var, coef) in vars {
            let bound = (f.clone() / coef.clone()) + var.min();
            let r = var.weak_upperbound(bound)?;
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
