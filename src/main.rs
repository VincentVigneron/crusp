#[macro_use]
extern crate solver_cp;

// SPECIFIC ID
// #[derive(Eq)]
// struct SpecificId {
//  puid: ProcessUniqueId,
//  id: Some(usize),
// }

//use solver_cp::branchers::handlers::*;
//use solver_cp::branchers::handlers::*;
use solver_cp::constraints::handlers::*;
//use solver_cp::spaces::*;
//use solver_cp::variables::*;
use solver_cp::variables::Array;
use solver_cp::variables::handlers::*;
//use solver_cp::variables::int_var::IntVar;
use solver_cp::variables::int_var::values_int_var::*;

// TODO expect

fn main() {
    let mut variables_handler = default_handler::Builder::new();
    let mut constraints_handler = SequentialConstraintsHandler::new();

    //type Variables = default_handler::Handler;
    //type Constraints = SequentialConstraintsHandler<Variables>;
    //type Brancher = DefaultBrancher<Variables>;
    //type Space = solver_cp::spaces::Space<Variables, Constraints, Brancher>;

    //let space: Space;

    variables!(
        handler = variables_handler;
        let a = var int(3 .. 10);
        let b = var int(2 .. 6);
        let c = var int(1 .. 9);
        let d = var int((-1) .. 12);
        let e = array[10] of var int(1 .. 15);
        let f = e[0];
        let g = var int(3 .. 5);
        );
    constraints!(
        handler = constraints_handler;
        constraint a < b;
        constraint c >= b;
        constraint d > c;
        constraint f |==| g;
        constraint increasing(e);
        );
    // INIT
    let mut variables_handler = variables_handler.finalize();
    println!("#############");
    println!("{:?}", variables_handler);
    constraints_handler.propagate_all(&mut variables_handler);
    println!("=============");
    println!("{:?}", variables_handler);
    //let brancher_ab = FirstVariableBrancher::new(vec![a, b]);
    //let brancher_c = FirstVariableBrancher::new(vec![c]);
    //let mut brancher = MultipleBrancherHandler::new();
    //brancher.add_brancher(Box::new(brancher_ab));
    //brancher.add_brancher(Box::new(brancher_c));
    ////let brancher = brancher;

    //let mut variables_handler = variables_handler.finalize();

    //let mut branches = brancher.branch_fn(&variables_handler).expect("branches");

    //for branch in branches {
    //let mut vars = variables_handler.clone();
    //branch(&mut vars);
    //println!("{:?}", vars);
    //}

    std::process::exit(0);

    // INIT
    /*
    constraints_handler.propagate_all(&mut variables_handler);
    println!("=============");
    println!("{:?}", variables_handler);

    {
        // FIRST BRANCH (a)
        //let (brancher, mut variables_handler) =
        //brancher.branch(&variables_handler).unwrap();
        constraints_handler.propagate_all(&mut variables_handler);
        println!("============= FIRST =============");
        println!("{:?}", variables_handler);

        // SECOND BRANCH (b)
        //let (brancher, mut variables_handler) =
        //brancher.branch(&mut variables_handler).unwrap();
        constraints_handler.propagate_all(&mut variables_handler);
        println!("============= SECOND =============");
        println!("{:?}", variables_handler);

        // THIRD BRANCH (c)
        //let (brancher, mut variables_handler) =
        //brancher.branch(&mut variables_handler).unwrap();
        constraints_handler.propagate_all(&mut variables_handler);
        println!("============= THIRD =============");
        println!("{:?}", variables_handler);

        // FOURTH BRANCH (nothing)
        println!("============= FOURTH =============");
        println!("{:?}", variables_handler);
        constraints_handler.propagate_all(&mut variables_handler);
        //match brancher.branch(&mut variables_handler) {
        //None => println!("Ok!"),
        //_ => println!("Error!"),
        //}
    }
    // INIT
    constraints_handler.propagate_all(&mut variables_handler);
    println!("=============");
    println!("{:?}", variables_handler);
    */
    /*
       println!("=============");
       println!("{:?}", variables_handler);
       */

    //constraints_handler.propagate_all(&mut variables_handler);

    /*
       println!("=============");
       println!("{:?}", variables_handler);
       */
}
