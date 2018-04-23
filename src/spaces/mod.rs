/*
use branchers::handlers::BranchersHandler;
use constraints::handlers::ConstraintsHandler;
use std::collections::VecDeque;
use std::marker::PhantomData;
use variables::handlers::VariablesHandler;

// Space : Constraints + Variables

#[allow(dead_code)]
#[derive(Clone)]
pub struct Space<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    variables: Variables,
    constraints: Constraints,
    brancher: Branchers,
}

impl<Variables, Constraints, Branchers> Space<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    pub fn new(
        variables: Variables,
        constraints: Constraints,
        brancher: Branchers,
    ) -> Space<Variables, Constraints, Branchers> {
        Space {
            variables: variables,
            constraints: constraints,
            brancher: brancher,
        }
    }

    pub fn propagate(&mut self) -> () {
        self.constraints.propagate_all(&mut self.variables);
    }

    pub fn branch(
        &mut self,
    ) -> Option<
        Box<
            Iterator<Item = Box<Fn(&mut Space<Variables, Constraints, Branchers>) -> ()>>,
        >,
    > {
        unimplemented!()
    }
}

pub struct SpaceIterator<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    branches: Box<Iterator<Item = Box<Fn(&mut Variables) -> ()>>>,
    phantom_constraints: PhantomData<Constraints>,
    phantom_branchers: PhantomData<Branchers>,
}

impl<Variables, Constraints, Branchers> SpaceIterator<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    fn new(
        space: &Space<Variables, Constraints, Branchers>,
    ) -> Option<SpaceIterator<Variables, Constraints, Branchers>> {
        space
            .brancher
            .branch_fn(&space.variables)
            .map(|branches| SpaceIterator {
                branches: branches,
                phantom_constraints: PhantomData,
                phantom_branchers: PhantomData,
            })
    }
}

impl<Variables, Constraints, Branchers> Iterator
    for SpaceIterator<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler + 'static,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    type Item = Box<Fn(&mut Space<Variables, Constraints, Branchers>) -> ()>;

    fn next(
        &mut self,
    ) -> Option<Box<Fn(&mut Space<Variables, Constraints, Branchers>) -> ()>> {
        match self.branches.next() {
            Some(branch) => Some(Box::new(move |space| {
                branch(&mut space.variables);
            })),
            _ => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Solver<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    init: Space<Variables, Constraints, Branchers>,
}

enum SearchState<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    Node(
        Space<Variables, Constraints, Branchers>,
        Box<Fn(&mut Space<Variables, Constraints, Branchers>) -> ()>,
    ),
    Finish,
    BackTrack,
}

impl<Variables, Constraints, Branchers> Solver<Variables, Constraints, Branchers>
where
    Variables: VariablesHandler,
    Constraints: ConstraintsHandler<Variables>,
    Branchers: BranchersHandler<Variables>,
{
    pub fn new(
        space: Space<Variables, Constraints, Branchers>,
    ) -> Solver<Variables, Constraints, Branchers> {
        Solver { init: space }
    }

    pub fn solve(&mut self) -> bool {
        self.init.propagate();
        let mut nodes = VecDeque::new();
        let branches = match self.init.branch() {
            Some(branches) => branches,
            _ => return false,
        };
        nodes.push_back((self.init.clone(), branches));

        'main: loop {
            let state = {
                match nodes.back_mut() {
                    Some(&mut (ref mut space, ref mut branches)) => match branches.next()
                    {
                        Some(branch) => {
                            let space = space.clone();
                            SearchState::Node(space, branch)
                        }
                        None => SearchState::BackTrack,
                    },
                    None => SearchState::Finish,
                }
            };
            match state {
                SearchState::Node(mut space, branch) => {
                    branch(&mut space);
                    let branches = match self.init.branch() {
                        Some(branches) => branches,
                        _ => return false,
                    };
                    nodes.push_back((space, branches));
                }
                SearchState::BackTrack => {
                    nodes.pop_back();
                }
                SearchState::Finish => break 'main,
            }
        }
        false
    }
}
*/
