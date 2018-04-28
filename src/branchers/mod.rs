use variables::{Variable, VariableView, ViewIndex};
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

pub mod brancher;
pub mod variables_selector;
pub mod values_selector;

//Brancher => Branch ?
pub trait VariableSelector<Handler, Var, View>
where
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
{
    fn select(&mut self, handler: &Handler) -> Result<View, ()>;
}

pub trait ValuesSelector<Handler, Var, View>
where
    View: VariableView + Into<ViewIndex> + Clone + 'static,
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
{
    fn select(
        &mut self,
        handler: &Handler,
        view: View,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()>;
}

pub trait SpecificBrancher<Handler, Var, View>: Brancher<Handler>
where
    Handler: VariablesHandler + SpecificVariablesHandler<Var, View>,
    Var: Variable,
    View: VariableView + Clone + Into<ViewIndex> + 'static,
{
    fn specific_branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()>;
}

use std::fmt::Debug;
pub trait Brancher<Handler> {
    fn branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()>
    where
        Handler: VariablesHandler;

    fn mutated_clone(&self) -> Box<Brancher<Handler>>;
}

pub struct BranchersHandler<Handler>
where
    Handler: VariablesHandler,
{
    branchers: Vec<Box<Brancher<Handler>>>,
}

impl<Handler> Clone for BranchersHandler<Handler>
where
    Handler: VariablesHandler,
{
    fn clone(&self) -> BranchersHandler<Handler> {
        let branchers = self.branchers
            .iter()
            .map(|brancher| brancher.mutated_clone())
            .collect();
        BranchersHandler {
            branchers: branchers,
        }
    }
}

impl<Handler> BranchersHandler<Handler>
where
    Handler: VariablesHandler,
{
    pub fn new() -> BranchersHandler<Handler> {
        BranchersHandler {
            branchers: Vec::new(),
        }
    }

    pub fn add_specific_brancher(&mut self, branch: Box<Brancher<Handler>>) {
        self.branchers.push(branch);
    }

    pub fn branch(
        &mut self,
        variables: &Handler,
    ) -> Result<Box<Iterator<Item = Box<Fn(&mut Handler) -> ()>>>, ()> {
        self.branchers
            .iter_mut()
            .filter_map(|brancher| brancher.branch(&variables).ok())
            .next()
            .ok_or(())
    }
}
