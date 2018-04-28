use branchers::BranchersHandler;
use constraints::PropagationState;
use constraints::handlers::ConstraintsHandler;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use variables::VariableError;
use variables::handlers::VariablesHandler;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Space<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    variables: Variables,
    constraints: Constraints,
    brancher: BranchersHandler<Variables>,
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

    pub fn print_variables(&self) {
        println!("{:?}", self.variables);
    }

    pub fn propagate(&mut self) -> Result<PropagationState, VariableError> {
        self.constraints.propagate_all(&mut self.variables)
    }

    pub fn branch(&mut self) -> Option<SpaceIterator<Variables, Constraints>> {
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct Solver<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    init: Space<Variables, Constraints>,
    solution: Option<Space<Variables, Constraints>>,
}

enum SearchState<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    Node(
        Space<Variables, Constraints>,
        Box<Fn(&mut Space<Variables, Constraints>) -> ()>,
    ),
    Finish,
    BackTrack,
}

impl<Variables, Constraints> Solver<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    pub fn new(space: Space<Variables, Constraints>) -> Solver<Variables, Constraints> {
        Solver {
            init: space,
            solution: None,
        }
    }

    pub fn solve(&mut self) -> bool {
        let prop = self.init.propagate();
        if prop.is_err() {
            return false;
        }
        println!("{:?}", self.init.variables);
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
                    let prop = space.propagate();
                    if prop.is_err() {
                        if nodes.is_empty() {
                            return false;
                        } else {
                            nodes.pop_back();
                        }
                    }
                    let branches = match space.branch() {
                        Some(branches) => branches,
                        // No new branch no failing on propagation success !
                        _ => {
                            self.solution = Some(space.clone());
                            return true;
                        }
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

    pub fn solution(&mut self) -> Option<Space<Variables, Constraints>> {
        use std::mem;
        let mut sol = None;
        mem::swap(&mut sol, &mut self.solution);
        sol
    }
}
