#![feature(trace_macros)]
#[macro_use]
extern crate solver_cp;

//use solver_cp::branchers::handlers::*;
use solver_cp::constraints::handlers::*;
use solver_cp::variables::*;
use solver_cp::variables::Array;
use solver_cp::variables::handlers::*;
use solver_cp::variables::int_var::IntVar;

fn test_branch<
    H: VariablesHandler + SpecificVariablesHandler<IntVar, View>,
    View: VariableView,
>(
    vars: &H,
    view: &View,
) -> H {
    let mut vars = vars.clone();
    {
        let var = get_mut_from_handler(&mut vars, view);
        let min = var.min();
        var.unsafe_set_value(min);
    }

    vars
}

fn main() {
    let mut variables_handler = default_handler::Builder::new();
    let mut constraints_handler = SequentialConstraintsHandler::new();

    variables!(
        handler = variables_handler;
        a = var int(3 .. 10);
        b = var int(2 .. 6);
        c = var int(1 .. 9);
        d = var int(2 .. 11);
        e = array[10] of var int(1 .. 9);
   );
    constraints!(
        handler = constraints_handler;
        constraint a < b;
        constraint c < d;
        constraint increasing(e);
    );

    let mut variables_handler = variables_handler.finalize();

    let new_vars = test_branch(&variables_handler, &a);
    println!("{:?}", new_vars);

    /*
    println!("=============");
    println!("{:?}", variables_handler);
    */

    constraints_handler.propagate_all(&mut variables_handler);

    /*
    println!("=============");
    println!("{:?}", variables_handler);
    */
}
