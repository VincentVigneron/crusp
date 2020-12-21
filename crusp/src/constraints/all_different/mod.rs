use constraints::Constraint;
use constraints::PropagationState;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use variables::domains::PrunableDomain;
use variables::handlers::{VariableContainerHandler, VariablesHandler};
use variables::{Array, Variable, VariableError, VariableId, VariableState};

#[derive(Debug, Clone)]
pub struct AllDifferent2<Var, Vars>
where
    Var: Variable,
    Var::Type: Eq + Ord + Clone,
    Vars: Array<Variable = Var>,
{
    array: Rc<RefCell<Vars>>,
    used: Vec<bool>,
    nb_used: usize,
    output: Option<Vec<(VariableId, VariableState)>>,
    id_to_pos: Arc<HashMap<VariableId, usize>>,
    changes: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct AllDifferent<Var, Vars, VCH>
where
    Var: Variable,
    Var::Type: Eq + Ord + Clone,
    Vars: Array<Variable = Var>,
    VCH: VariableContainerHandler<Vars>,
{
    array: VCH::View,
    used: Vec<bool>,
    nb_used: usize,
    output: Option<Vec<(VariableId, VariableState)>>,
    id_to_pos: Arc<HashMap<VariableId, usize>>,
    changes: Option<Vec<usize>>,
}

impl<Var, Vars, VCH> AllDifferent<Var, Vars, VCH>
where
    Var: Variable,
    Var::Type: Eq + Ord + Clone,
    Vars: Array<Variable = Var>,
    VCH: VariableContainerHandler<Vars>,
{
    pub fn new(array: VCH::View) -> Self {
        AllDifferent {
            array: array,
            used: vec![],
            nb_used: 0,
            output: None,
            id_to_pos: Arc::new(HashMap::new()),
            changes: None,
        }
    }
}
use std::fmt;

impl<Var, Vars, VCH> AllDifferent<Var, Vars, VCH>
where
    Var: Variable + PrunableDomain + 'static,
    Var::Type: Eq + Ord + Clone + 'static + fmt::Display + fmt::Debug,
    Vars: Array<Variable = Var>,
    VCH: VariableContainerHandler<Vars>,
{
    fn propagate_changes<Handler>(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError>
    where
        Handler: VariablesHandler + VariableContainerHandler<Vars> + Clone,
    {
        use std::collections::BTreeSet;
        use std::mem;
        use variables::VariableState;

        let mut changes = None;
        mem::swap(&mut changes, &mut self.changes);
        if self.output.is_none() {
            self.output = Some(Vec::new());
        }
        let mut changes = changes.unwrap();
        changes.sort();

        {
            let vars: &mut Vars = variables_handler.get_mut(&self.array);

            let mut affected = BTreeSet::new();

            for pos in changes {
                let var = vars.get_unchecked_mut(pos);
                if !var.is_affected() || self.used[pos] {
                    continue;
                }
                let val = var.value().unwrap();
                self.used[pos] = true;
                self.nb_used += 1;
                if !affected.insert(val) {
                    return Err(VariableError::DomainWipeout);
                }
            }

            let mut changes = vec![];
            for (pos, (var, _)) in vars
                .iter_mut()
                .zip(self.used.iter())
                .enumerate()
                .filter(|&(_, (_, ref used))| !**used)
            {
                match var.remove_if(|val| affected.contains(&val))? {
                    VariableState::NoChange => {}
                    state => {
                        self.output.as_mut().unwrap().push((var.id(), state));
                    }
                }
                if var.is_affected() {
                    changes.push(pos);
                }
            }
            if changes.is_empty() {
                if self.nb_used == self.used.len() {
                    return Ok(PropagationState::Subsumed);
                }

                if !self.output.as_ref().unwrap().is_empty() {
                    return Ok(PropagationState::FixPoint);
                } else {
                    self.output = None;
                    return Ok(PropagationState::NoChange);
                }
            }
            self.changes = Some(changes);
        }

        self.propagate_changes(variables_handler)
    }

    fn propagate_all<Handler>(
        &mut self,
        variables_handler: &mut Handler,
    ) -> Result<PropagationState, VariableError>
    where
        Handler: VariablesHandler + VariableContainerHandler<Vars> + Clone,
    {
        use std::collections::BTreeSet;
        use variables::VariableState;

        let mut output = vec![];
        self.output = None;

        {
            let vars = variables_handler.get_mut(&self.array);

            let (new_affected, unaffected): (Vec<_>, Vec<_>) = vars.iter()
                .zip(self.used.iter())
                .enumerate()
                //.filter(|&(_, (_, ref used))| !**used)
                .map(|(pos, (var, _))| (pos, var.value()))
                .partition(|(_, val)| val.is_some());
            let mut affected = BTreeSet::new();
            for (pos, val) in new_affected.into_iter() {
                if !self.used[pos] {
                    self.used[pos] = true;
                    self.nb_used += 1;
                }
                let dup = !affected.insert(val.unwrap());
                if dup {
                    return Err(VariableError::DomainWipeout);
                }
            }

            if unaffected.is_empty() {
                return Ok(PropagationState::Subsumed);
            }
            let mut changes = Vec::new();
            for (pos, _) in unaffected.into_iter() {
                let var = vars.get_unchecked_mut(pos);
                match var.remove_if(|val| affected.contains(&val))? {
                    VariableState::NoChange => {}
                    state => {
                        output.push((var.id(), state));
                        changes.push(pos);
                    }
                }
            }
        }

        if !output.is_empty() {
            self.output = Some(output);
            self.propagate_changes(variables_handler)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
}

impl<Var, Vars, VCH> Constraint<VCH> for AllDifferent<Var, Vars, VCH>
where
    VCH: VariablesHandler + VariableContainerHandler<Vars> + Clone,
    Var: Variable + PrunableDomain + 'static,
    Var::Type: Eq + Ord + Clone + 'static + fmt::Display + fmt::Debug,
    Vars: Array<Variable = Var>,
    VCH: VariableContainerHandler<Vars>,
{
    fn box_clone(&self) -> Box<Constraint<VCH>> {
        let ref_self: &AllDifferent<Var, Vars, VCH> = &self;
        let cloned: AllDifferent<Var, Vars, VCH> =
            <AllDifferent<Var, Vars, VCH> as Clone>::clone(ref_self);

        Box::new(cloned) as Box<Constraint<VCH>>
    }

    fn propagate(
        &mut self,
        variables_handler: &mut VCH,
    ) -> Result<PropagationState, VariableError> {
        match self.changes {
            None => self.propagate_all(variables_handler),
            _ => self.propagate_changes(variables_handler),
        }
    }
    #[allow(unused)]
    fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>) {
        self.changes = Some(
            states
                .map(|id| {
                    *self
                        .id_to_pos
                        .get(&id)
                        .expect("Error AllDifferent unknown VariableId.")
                }).collect::<Vec<_>>(),
        );
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
        variables_handler: &VCH,
    ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
        //Box::new({
        //let deps: Vec<_> = self.id_to_pos
        //.keys()
        //.cloned()
        //.map(|id| (id, VariableState::MaxBoundChange))
        //.collect();
        //deps.into_iter()
        //})
        let deps: Vec<_> = variables_handler
            .get(&self.array)
            .iter()
            .map(|var| (var.id(), VariableState::ValuesChange))
            .collect();
        Box::new(deps.into_iter())
    }
    fn initialise(
        &mut self,
        variables_handler: &mut VCH,
    ) -> Result<PropagationState, VariableError> {
        let len = variables_handler.get(&self.array).len();
        self.used = vec![false; len];
        self.id_to_pos = Arc::new(
            variables_handler
                .get(&self.array)
                .iter()
                .map(|val| val.id())
                .enumerate()
                .map(|(pos, id)| (id, pos))
                .collect(),
        );
        self.propagate(variables_handler)
    }
}
