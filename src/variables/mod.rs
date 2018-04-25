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

// Should be cloneable
pub trait VariableView {
    fn get_id(&self) -> ProcessUniqueId;
}

// TODO index remove pub befor var
#[derive(Debug, Clone)]
pub struct Array<Var: Variable> {
    pub variables: Vec<Var>,
}

impl<Var: Variable> Array<Var> {
    pub fn new(len: usize, var: Var) -> Option<Self> {
        Some(Array {
            variables: vec![var.clone(); len],
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
        unimplemented!()
    }
    fn retrieve_state(&mut self) -> VariableState {
        unimplemented!()
    }
}

#[macro_use]
pub mod macros;
