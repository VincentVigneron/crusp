#[macro_use]
extern crate crusp;

fn main() {
    let result = cp_model!(
        model {
            let send = var int (1023 ..= 9876);
            let more = var int (1023 ..= 9876);
            let money = var int (10234 ..= 20000);

            let s = var int (1 ..= 9);
            let e = var int (0 ..= 9);
            let n = var int (0 ..= 9);
            let d = var int (0 ..= 9);
            let m = var int (1 ..= 9);
            let o = var int (0 ..= 9);
            let r = var int (0 ..= 9);
            let y = var int (0 ..= 9);

            constraint all_different([s,e,n,d,m,o,r,y]);

            constraint send = (1000*s + 100*e + 10*n + d);
            constraint more = (1000*m + 100*o + 10*r + e);
            constraint money = (10000*m + 1000*o + 100*n + 10*e + y);
            constraint money = (send + more);
        }
        branchers {
            branch([s,e,n,d,m,o,r,y], variables_order, domain_order);
        }
        solve;
        output (send, more, money);
    );
    match result {
        Some((send, more, money)) => println!("{} = {} + {}", money, send, more),
        None => println!("No solution!"),
    }
}
