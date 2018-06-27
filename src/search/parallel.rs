use constraints::handlers::ConstraintsHandler;
use rayon::prelude::*;
use spaces::{BranchState, Space};
//use std::collections::VecDeque;
use search::path_recomputing::SolverPathRecomputing;
use std::fmt::Debug;
use variables::handlers::VariablesHandler;

#[allow(dead_code)]
#[derive(Clone)]
pub struct ParallelSolver<Variables, Constraints>
where
    Variables: VariablesHandler + Debug,
    Constraints: ConstraintsHandler<Variables>,
{
    init: Space<Variables, Constraints>,
    solution: Option<Space<Variables, Constraints>>,
    level: usize,
}

impl<Variables, Constraints> ParallelSolver<Variables, Constraints>
where
    Variables: VariablesHandler + Send + Sync + 'static + Debug,
    Constraints: ConstraintsHandler<Variables> + Send + Sync,
{
    pub fn new(
        space: Space<Variables, Constraints>,
    ) -> ParallelSolver<Variables, Constraints> {
        ParallelSolver {
            init: space,
            solution: None,
            level: 4,
        }
    }

    // replace macros by functions?
    //pub fn solve(&mut self) -> Option<Space<Variables, Constraints>> {
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

    fn solve_space(
        &self,
        space: Space<Variables, Constraints>,
    ) -> Option<Space<Variables, Constraints>> {
        let mut solver = SolverPathRecomputing::new(space);
        solver.solve();
        solver.solution()
    }

    fn dfs(
        &mut self,
        branches: Box<Iterator<Item = Box<Fn(&mut Variables) -> () + Send>>>,
    ) -> bool {
        let mut branches = branches.collect::<Vec<_>>();

        //self.solution = branches
        //.into_par_iter()
        //.map(|branch| {
        //let mut space = self.init.clone();
        //branch(&mut space.variables);
        //self.solve_space(space)
        //})
        //.find_any(Option::is_some)
        //.map(Option::unwrap);
        //self.solution.is_some()

        //remove pub variables
        for chunk in branches.chunks_mut(4) {
            self.solution = chunk
                .par_iter_mut()
                .map(|branch| {
                    let mut space = self.init.clone();
                    branch(&mut space.variables);
                    self.solve_space(space)
                })
                .find_any(Option::is_some)
                .map(Option::unwrap);
            if self.solution.is_some() {
                return true;
            }
        }
        self.solution.is_some()
    }

    pub fn solution(&mut self) -> Option<Space<Variables, Constraints>> {
        use std::mem;
        let mut sol = None;
        mem::swap(&mut sol, &mut self.solution);
        sol
    }

    //fn initialisation() -> SpaceState {
    //unimplemented!()
    //}
}
