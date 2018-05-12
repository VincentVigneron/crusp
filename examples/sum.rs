#[macro_use]
extern crate crusp;

fn main() {
    let coefs = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let result = cp_model!(
        model {
            let res = var int (0 .. 100);
            let values = array[10] of var int (1 .. 15);

            let a = values[0];
            let b = values[1];
            let c = values[2];
            let d = values[3];
            let e = values[4];
            let f = values[5];
            let g = values[6];
            let h = values[7];
            let i = values[8];
            let j = values[9];


            constraint res :: coefs * values;
        }
        branchers {
            branch([a,b,c,d,e,f,g,h,i,j], variables_order, domain_order);
        }
        solve;
        output (res,a,b,c,d,e,f,g,h,i,j);
    );
    match result {
        Some((res, a, b, c, d, e, f, g, h, i, j)) => println!(
            "{} = {} + {} + {} + {} + {} + {} + {} + {} + {} + {}",
            value!(res),
            value!(a),
            value!(b),
            value!(c),
            value!(d),
            value!(e),
            value!(f),
            value!(g),
            value!(h),
            value!(i),
            value!(j)
        ),
        None => println!("No solution!"),
    }
}
