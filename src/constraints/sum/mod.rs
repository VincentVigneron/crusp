use constraints::Constraint;
use constraints::PropagationState;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;
use variables::domains::OrderedDomain;
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::{Array, Variable, VariableError, VariableId, VariableState};

#[derive(Clone)]
enum Type {
    Result,
    Variable(usize),
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

macro_rules! views_struct {
    (
        Vars = struct $name_var:ident;
        MutVars = struct $name_mut:ident;
        Views = struct$name_view: ident<$($type_bound: ident),+> {
            $($var:ident: $type: ty),+,
        }
    ) => {
        #[derive(Clone)]
        pub struct $name_view<$($type_bound: VariableContainerView + 'static),+> {
            $($var: $type),+
        }
        #[allow(unused)]
        pub struct $name_mut<'a, $($type_bound: VariableContainerView + 'static),+> {
            $($var: &'a mut <$type>::Container),+
        }
        #[allow(unused)]
        pub struct $name_var<'a, $($type_bound: VariableContainerView + 'static),+> {
            $($var: &'a <$type>::Container),+
        }

        impl<$($type_bound: VariableContainerView),+> $name_view<$($type_bound),+> {
            pub fn new($($var:$type),+) -> Self {
                $name_view {
                    $($var),+
                }
            }

            #[allow(unused)]
            pub fn retrieve<'a, Handler>(&self, variables_handler: &'a Handler)
                -> $name_var<'a, $($type_bound),+>
            where
                Handler: VariablesHandler
                    $(+ VariableContainerHandler<$type_bound>)+
            {
                $name_var {
                    $(
                        $var: variables_handler.get(&self.$var)
                    ),+
                }
            }
            #[allow(unused)]
            pub fn retrieve_mut<'a, Handler>(&self, variables_handler: &'a mut Handler)
                -> $name_mut<'a, $($type_bound),+>
            where
                Handler: VariablesHandler
                    $(+ VariableContainerHandler<$type_bound>)+
            {
                $name_mut {
                    $(
                        $var: unsafe { unsafe_from_raw_point!(variables_handler.get_mut(&self.$var)) }
                    ),+
                }
            }

        }
    };
}

views_struct!(
Vars = struct Vars;
MutVars = struct MutVars;
Views = struct Variables<View, Views> {
    res: View,
    array: Views,
});

#[derive(Clone)]
pub struct SumConstraint<Var, View, Views>
where
    View: VariableContainerView + 'static,
    View::Container: OrderedDomain<Type = Var>,
    Views: VariableContainerView + 'static,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    variables: Variables<View, Views>,
    coefs: Vec<Var>,
    indexes: Arc<HashMap<VariableId, Type>>,
    input: Option<Vec<VariableId>>,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<Var, View, Views> SumConstraint<Var, View, Views>
where
    View: VariableContainerView,
    View::Container: OrderedDomain<Type = Var>,
    Views: VariableContainerView,
    Views::Variable: OrderedDomain<Type = Var>,
    Var: Ord + Eq + Clone,
{
    pub fn new<Coefs>(
        res: View,
        array: Views,
        coefs: Coefs,
    ) -> SumConstraint<Var, View, Views>
    where
        Coefs: IntoIterator<Item = Var>,
    {
        SumConstraint {
            variables: Variables::new(res, array),
            coefs: coefs.into_iter().collect(),
            indexes: Arc::new(HashMap::new()),
            input: None,
            output: None,
        }
    }
}

impl<Var, View, Views, Handler> Constraint<Handler> for SumConstraint<Var, View, Views>
where
    Handler: VariablesHandler
        + VariableContainerHandler<View>
        + VariableContainerHandler<Views>,
    View: VariableContainerView + 'static,
    View::Container: OrderedDomain<Type = Var>,
    Views: VariableContainerView + 'static,
    Views::Container: Array<Variable = Views::Variable>,
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

        let MutVars { res, array } = self.variables.retrieve_mut(variables_handler);

        let _contributions: Vec<_> = array
            .iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * (var.unchecked_max() - var.unchecked_min()))
            .collect();
        let min: Var = array
            .iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * var.unchecked_min())
            .sum();
        let max: Var = array
            .iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * var.unchecked_max())
            .sum();
        let r = res.weak_upperbound(max)?;

        change = change || (r != VariableState::NoChange);
        let r = res.weak_lowerbound(min.clone())?;
        change = change || (r != VariableState::NoChange);
        let mut output = vec![];
        output.push((res.id(), r));

        let f = res.unchecked_max() - min;
        //if f < 0 {
        //return Err(VariableError::DomainWipeout);
        //}
        let vars = array.iter_mut().zip(self.coefs.iter());
        for (var, coef) in vars {
            let bound = (f.clone() / coef.clone()) + var.unchecked_min();
            let r = var.weak_upperbound(bound)?;
            change = change || (r != VariableState::NoChange);
        }

        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
    fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>) {
        self.input = Some(states.collect());
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
    fn dependencies(
        &self,
        variables_handler: &Handler,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        //let Vars { res, array } = self.variables.retrieve(variables_handler);
        //let ids = res.iter_ids().chain(array.iter_ids());
        //let deps: Vec<_> = self.indexes
        //.keys()
        //.cloned()
        //.map(|id| (id, VariableState::MaxBoundChange))
        //.collect();
        //Box::new(deps.into_iter())
        use std::iter;
        let Vars { res, array } = self.variables.retrieve(variables_handler);
        let deps: Vec<_> = array
            .iter()
            .map(|var| (var.id(), VariableState::MaxBoundChange))
            .chain(iter::once((res.id(), VariableState::MaxBoundChange)))
            .collect();
        Box::new(deps.into_iter())
    }
    // Change error type
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        {
            let MutVars { res, array } = self.variables.retrieve_mut(variables_handler);
            let indexes = Arc::get_mut(&mut self.indexes).unwrap();
            indexes.insert(res.id(), Type::Result);
            for (pos, id) in array.iter().map(|var| var.id()).enumerate() {
                if indexes.insert(id, Type::Variable(pos)).is_some() {
                    return Err(VariableError::DomainWipeout);
                }
            }
        }
        self.propagate(variables_handler)
    }
}
