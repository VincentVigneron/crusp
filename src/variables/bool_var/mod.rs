use variables::{Variable, VariableState};

#[derive(Debug, Clone)]
pub struct BoolVar {}

impl Variable for BoolVar {
    type Type = bool;

    fn is_affected(&self) -> bool {
        unimplemented!()
    }
    fn value(&self) -> Option<Self::Type> {
        unimplemented!()
    }
    fn get_state(&self) -> &VariableState {
        unimplemented!()
    }
    fn retrieve_state(&mut self) -> VariableState {
        unimplemented!()
    }
}
