use constraints::handlers::ConstraintsHandler;
use spaces::{Space, SpaceState};
use std::collections::VecDeque;
use std::fmt::Debug;
use variables::handlers::VariablesHandler;

#[macro_use]
pub mod dsl;

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
    //Finish,
    BackTrack,
}

macro_rules! run_space {
    ($space: expr, $nodes: ident, $solution: expr) => {{
        let res = $space.run();
        match res {
            Ok(SpaceState::Subsumed) => {
                $solution = Some($space.clone());
                return true;
            },
            Ok(SpaceState::Branches(branches)) => {
                $nodes.push_back(($space.clone(), branches));
            }
            _ => return false,
        }
    }}
}

// maybe test if has next to avoid unecessary clone
macro_rules! next_search_state {
    ($nodes: expr) => {{
        match $nodes.back_mut() {
            Some(&mut (ref mut space, ref mut branches)) => match branches.next()
            {
                Some(branch) => {
                    let space = space.clone();
                    SearchState::Node(space, branch)
                }
                None => SearchState::BackTrack,
            },
            _ => unreachable!("nodes is empty can't reach this case!"),
        }
    }};
}

macro_rules! run_search_state {
    ($nodes: expr, $state: expr, $solution: expr) => {
        match $state {
            SearchState::Node(mut space, branch) => {
                branch(&mut space);
                let res = space.run();
                match res {
                    Ok(SpaceState::Subsumed) => {
                        $solution = Some(space.clone());
                        return true;
                    },
                    Ok(SpaceState::Branches(branches)) => {
                        $nodes.push_back((space.clone(), branches));
                    }
                    _ => {$nodes.pop_back();}
                }
            }
            SearchState::BackTrack => {
                $nodes.pop_back();
            }
            _ => unreachable!("nodes is empty can't reach this case!"),
        }
    }
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

    // replace macros by functions?
    pub fn solve(&mut self) -> bool {
        let mut nodes = VecDeque::new();
        // propagate => branch => test if search is ended
        run_space!(self.init, nodes, self.solution);

        while !nodes.is_empty() {
            let state = next_search_state!(nodes);
            run_search_state!(nodes, state, self.solution);
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
