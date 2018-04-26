use snowflake::ProcessUniqueId;

// TODO adding name to view
pub mod int_var;
pub mod handlers;

// TODO adding a subsume state to VariableState
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableState {
    BoundChange,
    ValuesChange,
    NoChange,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableError {
    DomainWipeout,
}

pub trait Variable: Clone {
    fn is_fixed(&self) -> bool;
    fn get_state(&self) -> &VariableState;
    fn retrieve_state(&mut self) -> VariableState;
}

// TODO impl should be cloneable
// TODO impl should be PartialEq SubView = view and view = Subview...
pub trait VariableView {
    fn get_id(&self) -> ProcessUniqueId;
}

// TODO index remove pub befor var
// TODO store state
#[derive(Debug, Clone)]
pub struct Array<Var: Variable> {
    pub variables: Vec<Var>,
    state: VariableState,
    //states: Vec<VariableState>,
}

impl<Var: Variable> Array<Var> {
    pub fn new(len: usize, var: Var) -> Option<Self> {
        Some(Array {
            variables: vec![var.clone(); len],
            state: VariableState::NoChange,
            //states: vec![VariableState::NoChange; len],
        })
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut Var {
        unsafe { &mut *(self.variables.get_unchecked_mut(idx) as *mut _) }
    }

    pub fn get(&self, idx: usize) -> &Var {
        unsafe { self.variables.get_unchecked(idx) }
    }
}
impl<Var: Variable> Variable for Array<Var> {
    fn is_fixed(&self) -> bool {
        unimplemented!()
    }
    fn get_state(&self) -> &VariableState {
        &self.state
    }
    fn retrieve_state(&mut self) -> VariableState {
        self.variables
            .iter()
            .map(|var| var.get_state())
            .scan(VariableState::NoChange, |acc, state| {
                if *acc == VariableState::BoundChange {
                    return None;
                }
                *acc = if *acc == VariableState::NoChange {
                    state.clone()
                } else if *state == VariableState::BoundChange {
                    VariableState::BoundChange
                } else {
                    acc.clone()
                };

                Some(acc.clone())
            })
            .last()
            .unwrap()
    }
}

#[macro_use]
pub mod macros;
