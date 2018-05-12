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

            constraint send = sum([s,e,n,d]*[1000,100,10,1]);
            constraint more = sum([m,o,r,e]*[1000,100,10,1]);
            constraint money = sum([m,o,n,e,y]*[10000,1000,100,10,1]);
            constraint money = sum([send,more]*[1,1]);
        }
        branchers {
            branch([s,e,n,d,m,o,r,y], variables_order, domain_order);
        }
        solve;
        output (send, more, money);
    );
    match result {
        Some((send, more, money)) => {
            println!("{} = {} + {}", value!(money), value!(send), value!(more))
        }
        None => println!("No solution!"),
    }
}
