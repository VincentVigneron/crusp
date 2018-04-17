use snowflake::ProcessUniqueId;

pub mod int_var;
pub mod handlers;

//enum VariableState {
//BoundChange(usize),
//DomainChange(usize),
//NoChange(usize),
//}

pub trait Variable: Clone {
    fn is_fixed(&self) -> bool;
}

pub trait VariableView: Clone {
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
}

#[macro_use]
pub mod macros;
