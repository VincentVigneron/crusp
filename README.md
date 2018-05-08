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
        branch [a,b,c];
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
3 < 4 < 5
```

### More Money
```rust
#[macro_use]
extern crate crusp;

fn main() {
    let result = cp_model!(
        model {
            let send = var int (1023 .. 9876);
            let more = var int (1023 .. 9876);
            let money = var int (10234 .. 20000);

            let s = var int (1 .. 9);
            let e = var int (0 .. 9);
            let n = var int (0 .. 9);
            let d = var int (0 .. 9);
            let m = var int (1 .. 9);
            let o = var int (0 .. 9);
            let r = var int (0 .. 9);
            let y = var int (0 .. 9);

            constraint all_different([s,e,n,d,m,o,r,y]);

            constraint send = (1000*s + 100*e + 10*n + d);
            constraint more = (1000*m + 100*o + 10*r + e);
            constraint money = (10000*m + 1000*o + 100*n + 10*e + y);
            constraint money = (send + more);
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
```

```
10652 = 9567 + 1085
```

## TODO LIST
- [ ] Top Priority Constraints (maybe remove macros as it is now)
- [ ] Priority Documentation
- [ ] Priority Remove ProcessUniqueId dependency.
- [ ] Priority OrderedDomain min and max should return an Option.
- [ ] Priority Split VariableView into: VariableView and ArrayView (trait ArrayView {type Array: Array..; type Variable: Variable;}).
- [ ] Change in\_sorted\_values from PrunbalDomain.
- [ ] Detect identic ArrayOfRefs one refarray for same views (if possible)
- [ ] Specific Result for ConstraintsHandler (i.e. Error or Ok)
- [ ] Better imports inside macros (i.e. avoid conflicts with imports).
- [ ] Prefer where clause to generics list.
- [ ] Prefer IntoIter to Iterator
- [ ] Change box\_clone to mutated\_clone or with another (more explicit) name.
- [ ] Adding Subsumed state to VariableState.
- [ ] Renaming ViewType.
- [ ] Refactoring ViewType (maybe some unecessary information).
- [ ] Refactoring ViewIndex (maybe some unecessary information).
- [ ] Precise the list of structs which have to be cloneable (Constraint, Propagator, Variable, View, ...) even if the traits do not require Clone because they are used as trait objects somewhere.
- [ ] Optimise IntVarValues (adding cut, ...)
- [ ] Misspellings!!!
- [ ] Add an integer parameter to variable state in order to determine the number of removed values (?).
- [ ] Refactoring test\_int\_var macro (duplicated code, modularity)
- [ ] Unit test for equal\_on\_bounds.
- [ ] IntVarBounds
- [ ] BoolVar
- [ ] Refactoring variables\_handler\_builder (remove duplicated code).
- [ ] Consistance between generic parameters order.
- [ ] Use expect instead of panic as much as possible.
- [ ] Display trait for Variable, VariableHandler, Constraint, ...
