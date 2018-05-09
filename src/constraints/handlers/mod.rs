use super::{Constraint, PropagationState};
use graph::TwoNodesGraph;
use variables::{VariableError, VariableState, ViewIndex};
use variables::handlers::VariablesHandler;

pub trait ConstraintsHandlerBuilder<Variables: VariablesHandler, Constraints: ConstraintsHandler<Variables>>
     {
    fn add(&mut self, Box<Constraint<Variables>>);
    fn finalize(self, variables: &mut Variables) -> Result<Constraints, VariableError>;
}

pub trait ConstraintsHandler<Variables: VariablesHandler>: Clone {
    fn propagate_all(
        &mut self,
        variables: &mut Variables,
    ) -> Result<PropagationState, VariableError>;
}

pub struct DefaultConstraintsHandlerBuilder<Variables: VariablesHandler> {
    constraints: Vec<Box<Constraint<Variables>>>,
}

impl<Variables: VariablesHandler> DefaultConstraintsHandlerBuilder<Variables> {
    pub fn new() -> DefaultConstraintsHandlerBuilder<Variables> {
        DefaultConstraintsHandlerBuilder {
            constraints: Vec::new(),
        }
    }
}

impl<
    Variables: VariablesHandler,
> ConstraintsHandlerBuilder<Variables, DefaultConstraintsHandler<Variables>>
    for DefaultConstraintsHandlerBuilder<Variables> {
    fn add(&mut self, constraint: Box<Constraint<Variables>>) {
        self.constraints.push(constraint);
    }

    fn finalize(
        mut self,
        variables: &mut Variables,
    ) -> Result<DefaultConstraintsHandler<Variables>, VariableError> {
        let mut graph: TwoNodesGraph<ViewIndex, usize, VariableState> =
            TwoNodesGraph::new();
        for (idx, constraint) in self.constraints.iter().enumerate() {
            for (view, state) in constraint.dependencies(&variables) {
                graph.insert_node1_to_node2(view, state, idx);
            }
        }
        // Sort according to complexity?
        for constraint in self.constraints.iter_mut() {
            constraint.initialise(variables)?;
        }
        let len = self.constraints.len();
        Ok(DefaultConstraintsHandler {
            constraints: self.constraints,
            subsumeds: vec![false; len],
            graph: graph,
        })
    }
}

#[derive(Clone)]
pub struct DefaultConstraintsHandler<H: VariablesHandler> {
    constraints: Vec<Box<Constraint<H>>>,
    subsumeds: Vec<bool>,
    graph: TwoNodesGraph<ViewIndex, usize, VariableState>,
}

impl<H: VariablesHandler> ConstraintsHandler<H> for DefaultConstraintsHandler<H> {
    fn propagate_all(
        &mut self,
        variables_handler: &mut H,
    ) -> Result<PropagationState, VariableError> {
        for (idx, constraint, subsumed) in self.constraints
            .iter_mut()
            .enumerate()
            .zip(self.subsumeds.iter_mut())
            .map(|((a, b), c)| (a, b, c))
            .filter(|&(_, _, ref subsumed)| !**subsumed)
        {
            constraint.prepare(Box::new(vec![].into_iter()));
            match constraint.propagate(variables_handler)? {
                PropagationState::FixPoint => for (view, state) in constraint.result() {
                    self.graph.add_event(view, idx, state);
                },
                PropagationState::Subsumed => {
                    *subsumed = true;
                    continue;
                }
                PropagationState::NoChange => {}
            };
        }

        while let Some(events) = self.graph.events() {
            for (idx, changes) in events {
                let constraint = self.constraints.get_mut(idx).unwrap();
                let subsumed = self.subsumeds.get_mut(idx).unwrap();
                constraint.prepare(Box::new(changes.into_iter()));
                match constraint.propagate(variables_handler)? {
                    PropagationState::FixPoint => {
                        for (view, state) in constraint.result() {
                            self.graph.add_event(view, idx, state);
                        }
                    }
                    PropagationState::Subsumed => {
                        *subsumed = true;
                        continue;
                    }
                    PropagationState::NoChange => {}
                };
            }
        }
        Ok(PropagationState::FixPoint)
    }
}
