use branchers::BranchersHandler;
use constraints::PropagationState;
use constraints::handlers::ConstraintsHandler;
use std::fmt::Debug;
use std::marker::PhantomData;
use variables::{Variable, VariableError, ViewIndex};
use variables::handlers::{get_from_handler, SpecificVariablesHandler, VariablesHandler};

#[derive(Clone)]
pub struct Space<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    pub variables: Variables,
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

    pub fn get_variable<'a, Var, View>(&'a self, view: &View) -> &'a Var
    where
        Var: Variable,
        View: Into<ViewIndex> + 'static,
        Variables: SpecificVariablesHandler<Var, View>,
    {
        get_from_handler(&self.variables, view)
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
