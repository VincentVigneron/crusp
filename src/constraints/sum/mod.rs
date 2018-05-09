use constraints::Constraint;
use constraints::PropagationState;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use variables::{Array, ArrayView, VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{SpecificArraysHandler, SpecificVariablesHandler,
                          VariablesHandler};

#[derive(Clone)]
enum Type {
    Result,
    Variable(usize),
}
/*
struct SumVariables<'a, Var, View, Views>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    res: &'a mut View::Variable,
    variables: &'a mut Views::Array,
}

struct SumViews<Var, View, Views>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    res: View,
    variables: Views,
}

impl<Var, View, Views> SumViews<Var, View, Views>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn retrieve_variables<'a, Handler>(
        &self,
        variables_handler: &'a mut Handler,
    ) -> SumVariables<'a, Var, View, Views>
    where
        Handler: VariablesHandler
            + SpecificVariablesHandler<View>
            + SpecificArraysHandler<Views>,
    {
        unsafe {
            SumVariables {
                res: unsafe_from_raw_point!(variables_handler.get_mut(&self.res)),
                variables: unsafe_from_raw_point!(
                    variables_handler.get_array_mut(&self.variables)
                ),
            }
        }
    }
}
*/

//variables_and_views!(
//struct SumVariables;
//struct SumViews;
//struct<Var, View, Views>
//where
//View: VariableView + Into<ViewIndex> + 'static,
//View::Variable: OrderedDomain<Type = Var>,
//Views: ArrayView + Into<ViewIndex> + 'static,
//Views::Variable: OrderedDomain<Type = Var>,
//Var: Ord + Eq + Clone,
//{
//res: View,
//Variables: Views,
//}
//);
//macro_rules! variables_and_views{
//(
//struct $variables: ident;
//struct $views: ident;
//struct<$($var_type: ident),+>
//where
//{
//$( $var: ident: $view: ty),+
//}
//) => {
//struct $variables<'a, $($var_type: 'a ),+> {
//$($var: &'a mut <$tvar>::Variable),+,
//}
//struct $views<$($var_type),+> {
//$($var: $view),+
//}
//impl<$($var_type),+> $views<$($var_type),+> {
//pub fn retrieve_variables<'a, $($var_type),+, Handler>(
//&self,
//variables_handler: &'a mut Handler,
//) -> $variables<'a, $($var_type),+>
//where
//Handler: VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+,
//{
//unsafe {
//StructVars {
//$(
//$var: unsafe_from_raw_point!(variables_handler.get_mut(&self.$var)),
//),+
//}
//}
//}
//};
//}
//}

#[derive(Clone)]
pub struct SumConstraint<Var, View, Views>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    res: View,
    variables: Views,
    coefs: Vec<Var>,
    indexes: Rc<HashMap<ViewIndex, Type>>,
    input: Option<Vec<ViewIndex>>,
    output: Option<Vec<(ViewIndex, VariableState)>>,
}

impl<Var, View, Views> SumConstraint<Var, View, Views>
where
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
    Views: ArrayView + Into<ViewIndex> + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new<Coefs>(
        res: View,
        variables: Views,
        coefs: Coefs,
    ) -> SumConstraint<Var, View, Views>
    where
        Coefs: IntoIterator<Item = Var>,
    {
        SumConstraint {
            res: res,
            variables: variables,
            coefs: coefs.into_iter().collect(),
            indexes: Rc::new(HashMap::new()),
            input: None,
            output: None,
        }
    }
}

macro_rules! constraint_box_clone {
    ($handler: ident) => {
        fn box_clone(&self) -> Box<Constraint<$handler>> {
            let ref_self: &Self = &self;
            let cloned: Self = <Self as Clone>::clone(ref_self);

            Box::new(cloned) as Box<Constraint<$handler>>
        }
    }
}

impl<Var, View, Views, Handler> Constraint<Handler> for SumConstraint<Var, View, Views>
where
    Handler: VariablesHandler
        + SpecificVariablesHandler<View>
        + SpecificArraysHandler<Views>,
    View: VariableView + Into<ViewIndex> + 'static,
    View::Variable: OrderedDomain<Type = Var>,
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
    constraint_box_clone!(Handler);

    // adding to propagator/constraint information about change view
    // add iter to array and size => len
    // [HarveySchimpf02]
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        use std::mem;
        use variables::VariableState;
        let mut change = false;

        let mut input = None;
        mem::swap(&mut input, &mut self.input);

        match input {
            None => {
                // first call
            }
            Some(changes) => {
                for idx in changes.into_iter() {
                    match *self.indexes.get(&idx).unwrap() {
                        Type::Result => {
                            // DO stuff
                            // break
                        }
                        Type::Variable(_pos) => {
                            // Incremental update
                        }
                    }
                }
            }
        }

        let vars: &mut Views::Array = unsafe {
            unsafe_from_raw_point!(variables_handler.get_array_mut(&self.variables))
        };
        let res: &mut View::Variable =
            unsafe { unsafe_from_raw_point!(variables_handler.get_mut(&self.res)) };

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
        let mut output = vec![];
        output.push((variables_handler.get_variable_id(&self.res), r));

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
    fn prepare(&mut self, states: Box<Iterator<Item = ViewIndex>>) {
        self.input = Some(states.collect());
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
        _: &Handler,
    ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
        let deps: Vec<_> = self.indexes
            .keys()
            .cloned()
            .map(|id| (id, VariableState::MaxBoundChange))
            .collect();
        Box::new(deps.into_iter())
    }
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        {
            let indexes = Rc::get_mut(&mut self.indexes).unwrap();
            let res_id = variables_handler.get_variable_id(&self.res);
            indexes.insert(res_id, Type::Result);
            for (pos, id) in variables_handler.get_array_ids(&self.variables).enumerate()
            {
                if indexes.insert(id, Type::Variable(pos)).is_some() {
                    return Err(VariableError::DomainWipeout);
                }
            }
        }
        self.propagate(variables_handler)
    }
}
