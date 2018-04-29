#[macro_use]
extern crate solver_cp;

use solver_cp::search::dsl::*;

fn main() {
    use solver_cp::constraints::handlers::SequentialConstraintsHandler;
    let result = cp_model!(
        model {
            let a = var int(3 .. 10);
            let b = var int(2 .. 6);
            let c = var int(1 .. 9);

            constraint a < b;
            constraint b < c;
        }
        branch [a,b,c];
        solve;
        output (a,b,c);
    );
    match result {
        Some((a, b, c)) => println!("{} < {} < {}", a, b, c),
        None => println!("No solution!"),
    }
}
