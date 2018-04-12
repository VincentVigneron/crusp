use std::marker::PhantomData;
use variables::handlers::VariablesHandler;

pub trait BranchersHandler<H: VariablesHandler>: Clone {
    fn branch(&self, variables: &H) -> Option<(Self, H)>;
}

#[derive(Clone)]
pub struct DefaultBrancher<H: VariablesHandler> {
    phantom: PhantomData<H>,
}

impl<H: VariablesHandler> BranchersHandler<H> for DefaultBrancher<H> {
    fn branch(&self, variables: &H) -> Option<(Self, H)> {
        unimplemented!()
    }
}
