# CRUSP (Constraint Rust Programming)

[![Build Status](https://travis-ci.org/VincentVigneron/solver_cp.svg?branch=master)](https://travis-ci.org/VincentVigneron/solver_cp)
[![codecov](https://codecov.io/gh/VincentVigneron/solver_cp/branch/master/graph/badge.svg)](https://codecov.io/gh/VincentVigneron/solver_cp)

## Example

```rust
#[macro_use]
extern crate solver_cp;

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

## TODO LIST
- [ ] Better name for ConstraintHandler Error.
- [ ] Adding multiple new functions for constraint\_build macro.
- [ ] Use where clause for generics instead of inline constraints in constraint\_build macro.
- [ ] Better imports (i.e. avoid conflicts with imports) in constraint\_build macro.
- [ ] Remove duplicated code for retrieve\_changed\_views function in constraint\_build macro.
- [ ] Remove duplicated code for box\_clone function in constraint\_build macro.
- [ ] Remove duplicated code in general in constraint\_build macro.
- [ ] Change box\_clone to mutated\_clone or with another (more explicit) name.
- [ ] Group views with the same type together in the retrieve\_changed\_views function in constraint\_build macro.
- [ ] Refactoring of constraint\_build macro.
- [ ] Remove the keyword constraint in constraints macro.
- [ ] Adding subsumed state to VariableState.
- [ ] Adding to variable view.
- [ ] Renaming ViewType.
- [ ] Refactoring ViewType (maybe some unecessary information).
- [ ] Refactoring ViewIndex (maybe some unecessary information).
- [ ] Adding documentation.
- [ ] Precise the list of structs which have to be cloneable (Constraint, Propagator, Variable, View, ...) even if the traits do not require Clone because they are used as trait objects somewhere.
- [ ] Remove the pub behind variables in struct variables::Array.
- [ ] Renaming SetIntVar (not a suitable name and confusing).
- [ ] Optimize SetIntVar (adding cut, ...)
- [ ] Renaming trait bounds for int\_var.
- [ ] Refactoring trait bounds for int\_var (i.e. set\_value is not necessary incompatible with BoundIntVar).
- [ ] Remove misspellings.
- [ ] Add an integer parameter to variable state in order to determine the number of removed values (maybe).
- [ ] Refactoring test\_int\_var macro (remove duplicated code, more generic test, test retrieve\_state, test depending of the new function, test for maximum usize value on new, ...)
- [ ] Unit test for equal\_bounds.
- [ ] Fix equal\_bounds.
- [ ] Fix BoundsIntVar.
- [ ] Add an ArrayViewTrait and a VariableViewTrait that are mutually incompatible.
- [ ] Refactoring variables\_handler\_builder (remove duplicated code).
- [ ] Consistance between generic parameters order.
- [ ] Rewritting branching.
- [ ] Use expect instead of panic as much as possible.
- [ ] Specific Result for ConstraintsHandler (i.e. Error or Ok)
- [ ] Display trait for Variable, VariableHandler, Constraint, ...
- [ ] Register la cosntraint that modified a variable (i.e. avoid to proagate twice with the same constraint "consecutively").
- [ ] Remove unecessary macros for specific varaibles handler.
