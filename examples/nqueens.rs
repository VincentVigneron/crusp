#[macro_use]
extern crate crusp;

fn main() {
    let n = 4;
    let min_n = -(n as i32);
    let max_n = (n as i32) * 2;
    let result = cp_model!(
        model {
            let queens = array[n] of var int(0 .. (n as i32));
            let diag1 = array[n] of var int(0 .. max_n);
            let diag2 = array[n] of var int(min_n .. (n as i32));
            let a = queens[0];
            let b = queens[1];
            let c = queens[2];
            let d = queens[3];

            for i in 0..n {
                let q = queens[i];
                let d1 = diag1[i];
                let d2 = diag2[i];
                constraint   add(d1,q,i as i32);
                constraint   add(d2,q,-(i as i32));
            }

            constraint all_different(queens);
            constraint all_different(diag1);
            constraint all_different(diag2);
        }
        branchers {
            branch([queens[i] for i in 0 .. n], variables_order, domain_order);
        }
        solve;
        output (a,b,c,d);
        );
    match result {
        Some((a, b, c, d)) => println!("{} {} {} {}", a, b, c, d),
        None => println!("No solution!"),
    }
}
