#[macro_use]
extern crate crusp;

use crusp::variables::Variable;

fn main() {
    let result = cp_model!(
        model {
            let a = var int(3 .. 10);
            let b = var int(2 .. 6);
            let c = var int(1 .. 9);

            constraint a < b;
            constraint b < c;

            constraint increasing([a,b,c]);
        }
        branchers {
            branch([a,b,c], variables_order, domain_max);
        }
        solve;
        output (a,b,c);
    );
    match result {
        Some((a, b, c)) => println!("{} < {} < {}", value!(a), value!(b), value!(c)),
        None => println!("No solution!"),
    }
}
