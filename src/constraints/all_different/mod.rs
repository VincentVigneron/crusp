use constraints::Constraint;
use constraints::PropagationState;
use variables::domains::PrunableDomain;
use variables::handlers::{VariableContainerHandler, VariableContainerView, VariablesHandler};
use variables::{Array, Variable, VariableError, VariableId, VariableState};

#[derive(Debug, Clone)]
pub struct AllDifferent<Var, Views>
where
    Views: VariableContainerView,
    Views::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    array: Views,
    used: Vec<bool>,
    output: Option<Vec<(VariableId, VariableState)>>,
}

impl<Var, Views> AllDifferent<Var, Views>
where
    Views: VariableContainerView,
    Views::Variable: PrunableDomain<Type = Var>,
    Var: Eq + Ord + Clone,
{
    pub fn new(array: Views) -> Self {
        AllDifferent {
            array: array,
            output: None,
            used: vec![],
        }
    }
}
use std::fmt;

impl<Var, Views, Handler> Constraint<Handler> for AllDifferent<Var, Views>
where
    Handler: VariablesHandler + VariableContainerHandler<Views> + Clone,
    Views: VariableContainerView + 'static,
    Views::Container: Array<Variable = Views::Variable>,
    Views::Variable: PrunableDomain<Type = Var> + 'static,
    Var: Eq + Ord + Clone + 'static + fmt::Display,
{
    fn box_clone(&self) -> Box<Constraint<Handler>> {
        let ref_self: &AllDifferent<Var, Views> = &self;
        let cloned: AllDifferent<Var, Views> = <AllDifferent<Var, Views> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<Handler>>
    }
    fn propagate(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        use std::collections::BTreeSet;
        use variables::VariableState;
        let mut output = vec![];
        self.output = None;

        let vars = variables_handler.get_mut(&self.array);

        //println!("+++++++++++");
        let _ = vars.iter()
            .zip(self.used.iter())
            .filter(|&(_, ref used)| **used)
            .map(|(var, _)| var.value().clone().unwrap())
            //.inspect(|val| println!("{}", val))
            .last();
        let (new_affected, unaffected): (Vec<_>, Vec<_>) = vars.iter()
            .zip(self.used.iter())
            .enumerate()
            .filter(|&(_, (_, ref used))| !**used)
            .map(|(pos, (var, _))| (pos, var.value()))
            .partition(|(_, val)| val.is_some());
        let mut affected = BTreeSet::new();
        for (pos, val) in new_affected.into_iter() {
            self.used[pos] = true;
            let dup = !affected.insert(val.unwrap());
            if dup {
                return Err(VariableError::DomainWipeout);
            }
        }
        //println!("===========");
        let _ = vars.iter()
            .zip(self.used.iter())
            .filter(|&(_, ref used)| **used)
            .map(|(var, _)| var.value().clone().unwrap())
            //.inspect(|val| println!("{}", val))
            .last();

        let end = unaffected.is_empty();
        for (pos, _) in unaffected.into_iter() {
            let var = vars.get_unchecked_mut(pos);
            match var.remove_if(|val| affected.contains(&val))? {
                VariableState::NoChange => {}
                state => {
                    output.push((var.id(), state));
                }
            }
        }
        if end {
            return Ok(PropagationState::Subsumed);
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
        // do nothing
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
        Box::new({
            let dep: Vec<_> = variables_handler
                .get(&self.array)
                .iter()
                .map(|val| (val.id(), VariableState::ValuesChange))
                .collect();
            dep.into_iter()
        })
    }
    fn initialise(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError> {
        let len = variables_handler.get(&self.array).len();
        self.used = vec![false; len];
        self.propagate(variables_handler)
    }
}
