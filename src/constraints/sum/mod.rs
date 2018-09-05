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

#[derive(Clone)]
pub struct SumConstraint<VarType, View, Views>
where
    View: VariableContainerView + 'static,
    Views: VariableContainerView + 'static,
    VarType: Ord + Eq + Clone,
{
    res: View,
    array: Views,
    coefs: Vec<VarType>,
    indexes: Arc<HashMap<VariableId, Type>>,
    input: Option<Vec<VariableId>>,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<VarType, View, Views> SumConstraint<VarType, View, Views>
where
    View: VariableContainerView,
    Views: VariableContainerView,
    VarType: Ord + Eq + Clone,
{
    pub fn new<Coefs>(
        res: View,
        array: Views,
        coefs: Coefs,
    ) -> SumConstraint<VarType, View, Views>
    where
        Coefs: IntoIterator<Item = VarType>,
    {
        SumConstraint {
            res: res,
            array: array,
            coefs: coefs.into_iter().collect(),
            indexes: Arc::new(HashMap::new()),
            input: None,
            output: None,
        }
    }
}

impl<Var, VarArray, VarType, View, Views, Handler> Constraint<Handler>
    for SumConstraint<VarType, View, Views>
where
    Handler: VariablesHandler
        + VariableContainerHandler<Var, View = View>
        + VariableContainerHandler<VarArray, View = Views>,
    View: VariableContainerView + 'static,
    Views: VariableContainerView + 'static,
    Var: OrderedDomain<Type = VarType>,
    VarArray: Array<Variable = Var>,
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
    //constraint_box_clone!(Handler);
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &Self = &self;
        let cloned: Self = <Self as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }

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

        let res: &mut Var = variables_handler.get_mut(&self.res);
        let array: &mut VarArray = variables_handler.get_mut(&self.array);

        let _contributions: Vec<_> = array
            .iter()
            .zip(self.coefs.iter().cloned())
            .map(|(var, coef)| coef * (var.unchecked_max() - var.unchecked_min()))
            .collect();
        let min: VarType = array
            .iter()
            .zip(self.coefs.iter().cloned())
            //.map(|(var, coef)| coef * var.unchecked_min())
            .map(|(var, coef)| coef * var.unchecked_min())
            .sum();
        let max: VarType = array
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
        let res: &Var = variables_handler.get(&self.res);
        let array: &VarArray = variables_handler.get(&self.array);
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
            let res: &mut Var = variables_handler.get_mut(&self.res);
            let array: &mut VarArray = variables_handler.get_mut(&self.array);
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
