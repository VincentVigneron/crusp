use branchers::BranchersHandler;
use constraints::handlers::{ConstraintsHandler, ConstraintsHandlerBuilder};
use constraints::PropagationState;
use std::fmt::Debug;
use std::marker::PhantomData;
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
    VariablesHandlerBuilder,
};
use variables::VariableError;

#[derive(Clone)]
pub struct SpaceBuilder<Variables, VariablesBuilder, Constraints, ConstraintsBuilder>
where
    Variables: VariablesHandler,
    VariablesBuilder: VariablesHandlerBuilder<Variables>,
    Constraints: ConstraintsHandler<Variables>,
    ConstraintsBuilder: ConstraintsHandlerBuilder<Variables, Constraints>,
{
    variables: VariablesBuilder,
    constraints: ConstraintsBuilder,
    brancher: BranchersHandler<Variables>,
    _phantom: PhantomData<*const Constraints>,
}

impl<Variables, VariablesBuilder, Constraints, ConstraintsBuilder>
    SpaceBuilder<Variables, VariablesBuilder, Constraints, ConstraintsBuilder>
where
    Variables: VariablesHandler,
    VariablesBuilder: VariablesHandlerBuilder<Variables>,
    Constraints: ConstraintsHandler<Variables>,
    ConstraintsBuilder: ConstraintsHandlerBuilder<Variables, Constraints>,
{
    pub fn new(
        variables: VariablesBuilder,
        constraints: ConstraintsBuilder,
        brancher: BranchersHandler<Variables>,
    ) -> Self {
        SpaceBuilder {
            variables: variables,
            constraints: constraints,
            brancher: brancher,
            _phantom: PhantomData,
        }
    }

    //fn finalize(self) -> Space<Variables, Constraints> -> Result<PropagationState,VaraibleError> {
    //let mut variables = self.variables.finalize();
    //let constraints = self.constraints.finalize(&mut variables)?;
    //Space {
    //variables: variables,
    //constraints: constraints,
    //branchers: self.branchers,
    //}
    //}
}

#[derive(Clone)]
pub struct Space<Variables, Constraints>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
{
    variables: Variables,
    constraints: Constraints,
    brancher: BranchersHandler<Variables>,
}

pub enum SpaceState<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    Subsumed,
    Branches(SpaceIterator<Variables, Constraints>),
}

impl<Variables, Constraints> Space<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    pub fn new(
        variables: Variables,
        constraints: Constraints,
        brancher: BranchersHandler<Variables>,
    ) -> Space<Variables, Constraints> {
        Space {
            variables: variables,
            constraints: constraints,
            brancher: brancher,
        }
    }

    pub fn get_variable<'a, View>(&'a self, view: &View) -> &'a View::Container
    where
        View: VariableContainerView,
        Variables: VariableContainerHandler<View>,
    {
        self.variables.get(view)
    }

    // disable run method after it was used (chagne type/ using state)..
    pub fn run(&mut self) -> Result<SpaceState<Variables, Constraints>, VariableError> {
        self.propagate()?;
        match self.branch() {
            Some(branches) => Ok(SpaceState::Branches(branches)),
            _ => Ok(SpaceState::Subsumed),
        }
    }

    fn propagate(&mut self) -> Result<PropagationState, VariableError> {
        self.constraints.propagate_all(&mut self.variables)
    }

    fn branch(&mut self) -> Option<SpaceIterator<Variables, Constraints>> {
        SpaceIterator::new(self)
    }
}

pub struct SpaceIterator<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    branches: Box<Iterator<Item = Box<Fn(&mut Variables) -> ()>>>,
    phantom_constraints: PhantomData<Constraints>,
}

impl<Variables, Constraints> SpaceIterator<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    fn new(
        space: &mut Space<Variables, Constraints>,
    ) -> Option<SpaceIterator<Variables, Constraints>> {
        space
            .brancher
            .branch(&space.variables)
            .ok()
            .map(|branches| SpaceIterator {
                branches: branches,
                phantom_constraints: PhantomData,
            })
    }
}

impl<Variables, Constraints> Iterator for SpaceIterator<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    type Item = Box<Fn(&mut Space<Variables, Constraints>) -> ()>;

    fn next(&mut self) -> Option<Box<Fn(&mut Space<Variables, Constraints>) -> ()>> {
        match self.branches.next() {
            Some(branch) => Some(Box::new(move |space| {
                branch(&mut space.variables);
            })),
            _ => None,
        }
    }
}
