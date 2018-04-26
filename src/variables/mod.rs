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

// TODO renaming?
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ViewType {
    FromVar(usize),
    FromArray(usize, usize),
}

// TODO PartialEq on ProcessUniqId only !!!!
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ViewIndex {
    id: ProcessUniqueId,
    view: ViewType,
}

impl ViewIndex {
    pub fn new_from_var(id: ProcessUniqueId, x: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            view: ViewType::FromVar(x),
        }
    }
    pub fn new_from_array(id: ProcessUniqueId, x: usize, y: usize) -> ViewIndex {
        ViewIndex {
            id: id,
            view: ViewType::FromArray(x, y),
        }
    }
    pub fn is_subview_of(&self, idx: &ViewIndex) -> bool {
        if self.id != idx.id {
            return false;
        }
        match self.view {
            ViewType::FromArray(x, _) => match idx.view {
                ViewType::FromVar(x_) if x == x_ => true,
                _ => false,
            },
            _ => false,
        }
    }
}

pub trait Variable: Clone {
    fn is_fixed(&self) -> bool;
    fn get_state(&self) -> &VariableState;
    fn retrieve_state(&mut self) -> VariableState;
}

// TODO impl should be cloneable
// TODO impl should be PartialEq SubView = view and view = Subview...
pub trait VariableView {
    fn get_id(&self) -> ViewIndex;
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

    pub fn iter_mut<'a>(&'a mut self) -> Box<Iterator<Item = &mut Var> + 'a> {
        Box::new(self.variables.iter_mut())
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
