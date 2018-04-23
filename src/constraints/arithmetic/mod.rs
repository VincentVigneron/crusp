use super::Propagator;
use variables::int_var::BoundsIntVar;

// using macro
#[derive(Debug, Clone)]
struct ArithmeticComparatorPropagator {}
impl Propagator for ArithmeticComparatorPropagator {}
impl ArithmeticComparatorPropagator {
    pub fn new() -> ArithmeticComparatorPropagator {
        ArithmeticComparatorPropagator {}
    }

    pub fn less_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) {
        let _ = lhs.less_than(rhs);
    }

    pub fn less_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) {
        let _ = lhs.less_or_equal_than(rhs);
    }

    pub fn greater_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) {
        let _ = rhs.greater_than(lhs);
    }

    pub fn greater_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) {
        let _ = rhs.greater_or_equal_than(lhs);
    }
}

pub mod less_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_than(x: VarType, y: VarType) -> ()
        where VarType: BoundsIntVar<Type=i32>;
        );

}

pub mod less_or_equal_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_or_equal_than(x: VarType, y: VarType) -> ()
        where VarType: BoundsIntVar<Type=i32>;
        );

}

pub mod greater_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_than(x: VarType, y: VarType) -> ()
        where VarType: BoundsIntVar<Type=i32>;
        );

}

pub mod greater_or_equal_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_or_equal_than(x: VarType, y: VarType) -> ()
        where VarType: BoundsIntVar<Type=i32>;
        );
}
