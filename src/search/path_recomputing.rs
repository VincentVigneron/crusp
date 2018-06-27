use constraints::handlers::ConstraintsHandler;
use spaces::{BranchState, Space};
use std::collections::VecDeque;
use std::fmt::Debug;
use variables::handlers::VariablesHandler;

#[allow(dead_code)]
#[derive(Clone)]
pub struct SolverPathRecomputing<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    init: Space<Variables, Constraints>,
    solution: Option<Space<Variables, Constraints>>,
}

impl<Variables, Constraints> From<Space<Variables, Constraints>>
    for SolverPathRecomputing<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    fn from(space: Space<Variables, Constraints>) -> Self {
        SolverPathRecomputing::new(space)
    }
}

impl<Variables, Constraints> SolverPathRecomputing<Variables, Constraints>
where
    Variables: VariablesHandler + 'static + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    pub fn new(
        space: Space<Variables, Constraints>,
    ) -> SolverPathRecomputing<Variables, Constraints> {
        SolverPathRecomputing {
            init: space,
            solution: None,
        }
    }

    // replace macros by functions?
    pub fn solve(&mut self) -> bool {
        match self.init.run_branch() {
            Ok(BranchState::Subsumed) => {
                self.solution = Some(self.init.clone());
                true
            }
            Ok(BranchState::Branches(branches)) => self.dfs(branches),
            _ => false,
        }
    }

    fn dfs(
        &mut self,
        mut branches: Box<Iterator<Item = Box<Fn(&mut Variables) -> () + Send>>>,
    ) -> bool {
        let mut space = self.init.clone();
        //depth
        let branch = branches.next();
        if branch.is_none() {
            self.solution = Some(space);
            return true;
        }
        let mut branch = {
            // using a Stack is necessary, because destructuring an Option contatining
            // a Box move the value inside the option, however while let
            // requires to copy the value.
            let mut stack = VecDeque::new();
            stack.push_back(branch.unwrap());
            stack
        };
        let mut branches = {
            //let mut next_branches = VecDeque::with_capacity(); // add method returning hypotetical
            let mut stack = VecDeque::new();
            stack.push_back(branches);
            stack
        };
        let mut path = VecDeque::new();

        while let Some(explored_branch) = branch.pop_back() {
            explored_branch(&mut space.variables);
            match space.run_branch() {
                Ok(BranchState::Subsumed) => {
                    self.solution = Some(space);
                    return true;
                }
                Ok(BranchState::Branches(mut next_branches)) => {
                    path.push_back(explored_branch);
                    let next_branch = next_branches.next();
                    if next_branch.is_none() {
                        self.solution = Some(space);
                        return true;
                    }
                    branch.push_back(next_branch.unwrap());
                    branches.push_back(next_branches);
                }
                _ => {
                    'backtrack: while !branches.is_empty() {
                        match branches.back_mut() {
                            Some(ref mut next_branches) => {
                                if let Some(next_branch) = next_branches.next() {
                                    space = self.init.clone();
                                    for p in path.iter() {
                                        p(&mut space.variables);
                                    }
                                    branch.push_back(next_branch);
                                    break 'backtrack;
                                }
                            }
                            _ => unreachable!(),
                        }
                        // TODO backjump instead
                        path.pop_back();
                        branches.pop_back();
                    }
                    if branches.is_empty() {
                        return false;
                    }
                }
            }
        }
        // unreachable
        false
    }

    pub fn solution(&mut self) -> Option<Space<Variables, Constraints>> {
        use std::mem;
        let mut sol = None;
        mem::swap(&mut sol, &mut self.solution);
        sol
    }
}
