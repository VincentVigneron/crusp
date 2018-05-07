#[macro_use]
extern crate crusp;

use crusp::branchers::BranchersHandler;
use crusp::branchers::brancher::DefaultBrancher;
use crusp::branchers::values_selector::MinValueSelector;
use crusp::branchers::variables_selector::SequentialVariableSelector;
use crusp::constraints::handlers::*;
use crusp::search::Solver;
use crusp::spaces::Space;
use crusp::variables::{ArrayOfVars, Variable};
use crusp::variables::handlers::*;
use crusp::variables::int_var::*;

fn main() {
    let mut variables_handler = default_handler::Builder::new();
    let mut constraints_handler = SequentialConstraintsHandler::new();
    let mut branchers_handler = BranchersHandler::new();

    variables!(
        handler = variables_handler;
        let e = array[10] of var int(1 .. 15);
        let f = e[0];
        let g = var int(4 .. 5);
        );
    constraints!(
        handler = constraints_handler;
        constraint f |==| g;
        constraint increasing(e);
        );
    let variables_handler = variables_handler.finalize();

    let variables: Vec<_> = (0..10).map(|i| e.get(i).clone()).collect();
    let variables_selector =
        SequentialVariableSelector::new(variables.into_iter()).unwrap();
    let values_selector = MinValueSelector::new();
    let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
    branchers_handler.add_specific_brancher(Box::new(brancher));

    let space = Space::new(variables_handler, constraints_handler, branchers_handler);
    let mut solver = Solver::new(space);
    if solver.solve() {
        let solution = solver.solution().unwrap();
        println!("Solution");
        let result = (0..10)
            .map(|i| e.get(i))
            .map(|var| solution.get_variable(&var).value().unwrap())
            .map(|var| format!("{}", var))
            .collect::<Vec<String>>()
            .join(" < ");
        println!("{}", result);
    } else {
        println!("No solution");
    }
}
