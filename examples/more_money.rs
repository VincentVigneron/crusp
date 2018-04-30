#[macro_use]
extern crate solver_cp;

fn main() {
    let result = cp_model!(
        model {
            let send = var int (0 .. 9999);
            let more = var int (0 .. 9999);
            let money = var int (0 .. 99999);

            let s = var int (0 .. 9);
            let e = var int (0 .. 9);
            let n = var int (0 .. 9);
            let d = var int (0 .. 9);
            let m = var int (0 .. 9);
            let o = var int (0 .. 9);
            let r = var int (0 .. 9);
            let y = var int (0 .. 9);

            constraint send = sum([s,e,n,d]*[1000,100,10,1]);
            constraint more = sum([m,o,r,e]*[1000,100,10,1]);
            constraint money = sum([m,o,n,e,y]*[10000,1000,100,10,1]);
            constraint money = sum([send,more]*[1,1]);
            //constraint send = 1000*s + 100*e + 10*n + 1*d;
            //constraint more = 1000*m + 100*o + 10*r + 1*e;
            //constraint money = 10000*m + 1000*o + 100*n + 10*e + 1*y;

            //constraint money = 1 * send + 1 * money;
        }
        branch [s,e,n,d,m,o,r,y];
        solve;
        output (send, more, money);
    );
    match result {
        Some((send, more, money)) => println!("{} = {} + {}", money, send, more),
        None => println!("No solution!"),
    }
}
