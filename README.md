# CRUSP (Constraint Rust Programming)

[![Build Status](https://travis-ci.org/VincentVigneron/crusp.svg?branch=master)](https://travis-ci.org/VincentVigneron/crusp)
[![codecov](https://codecov.io/gh/VincentVigneron/crusp/branch/master/graph/badge.svg)](https://codecov.io/gh/VincentVigneron/crups)

## Documentation
[API Documentation](https://vincentvigneron.github.io/crusp/)

## Examples

### Ordering
```rust
#[macro_use]
extern crate crusp;

fn main() {
    let result = cp_model!(
        model {
            let a = var int(3 .. 10);
            let b = var int(2 .. 6);
            let c = var int(1 .. 9);

            constraint a < b;
            constraint b < c;
        }
        branchers {
            branch([a,b,c], variables_order, domain_max);
        }
        solve;
        output (a,b,c);
    );
    match result {
        Some((a, b, c)) => println!("{} < {} < {}", a, b, c),
        None => println!("No solution!"),
    }
}
```

```
5 < 6 < 9
```

### More Money
```rust
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
```

```
10652 = 9567 + 1085
```

## TODO LIST
- [ ] Develop all IntVar (bounds, values, intervals, bitset)
- [ ] Develop BoolVar
- [ ] Develop SetVar
- [ ] Develop FloatVar
- [ ] Develop other kinds of variables (String, Path, etc.)
- [ ] Finish procedural macro for Constraints
- [ ] Proc marcro support for multiple propagate method (to distinguish propagation based on variable/consitency lvl)
- [ ] Proc marcro consitency lvl per variable with custom propagator (i.e. #[var(bound:propagate_bound,val:propagate)] x: IntVar; => will call propagate_bound if only bound change otherwise will call propagate)
- [ ] Previous one requieres to "dedup" constraint inside events graph (constraint+code inside graph)
- [ ] Allow constraint mutation during cloning (maybe create multiple constraint or do not copy). Maybe add mutable_clone(&self, constraints: &mut ConstraintsBuilder); Will not return clone but will ask to put the new constraints. Maybe add method register_new and remove to ConstraintsBuilder. The events graph might be requried here?
- [ ] Unregister variable/constraint from events Graph during propagation stage
- [ ] Consitency lvl based on variable type. Require the graph to support many types or will ask a conversion to a common type (probably a bitmask type because it allows easy subsomption betweeen different events. And most event will be u32 or u8). Each variable has one type if two types of variables have the same type is not a problem
because propagation is based on variable + event. The other solution is to have one graph per type of variable. Which is not problematic because everything is staticly dispatched.
- [ ] Support for nogoods
- [ ] Support for LNS
- [ ] Support for QCSP
- [ ] Collect stats during search (number of failures per var, number of nodes, etc...)
- [ ] Create a NoState method to remove all of this
- [ ] Distinguish between VAL and DOM change for IntVarList
- [ ] Limit the use of Sync and Send
- [x] Change constraint output function and pass a reference to a mutable collection in the propagation method instead.
- [ ] Priority Graph Split Event And Graph.
- [ ] Event use iterator (avoid unecessary computation).
- [ ] Priority Documentation
- [ ] Priority Remove ProcessUniqueId dependency.
- [ ] Remove Vec and use array if possible.
- [ ] Detect identic ArrayOfRefs one refarray for same views (if possible)
- [ ] Prefer where clause to generics list.
- [ ] Prefer IntoIter to Iterator
- [ ] Change box\_clone to mutated\_clone or with another (more explicit) name.
- [ ] Adding Subsumed state to VariableState.
- [ ] Precise the list of structs which have to be cloneable (Constraint, Propagator, Variable, View, ...) even if the traits do not require Clone because they are used as trait objects somewhere.
- [ ] Optimise IntVarValues (adding cut, ...)
- [ ] Misspellings!!!
- [ ] Add an integer parameter to variable state in order to determine the number of removed values (?).
- [ ] Use expect instead of pani.
- [ ] Display trait for Variable, VariableHandler, Constraint, ...
