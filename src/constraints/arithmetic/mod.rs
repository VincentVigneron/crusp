use super::Propagator;
use variables::int_var::IntVar;

#[derive(Debug, Clone)]
struct ArithmeticComparatorPropagator {}
impl Propagator for ArithmeticComparatorPropagator {}
impl ArithmeticComparatorPropagator {
    pub fn new() -> ArithmeticComparatorPropagator {
        ArithmeticComparatorPropagator {}
    }

    pub fn less_than(&self, lhs: &mut IntVar, rhs: &mut IntVar) {
        let _ = lhs.less_than(rhs);
    }

    pub fn greater_or_equal_than(&self, lhs: &mut IntVar, rhs: &mut IntVar) {
        let _ = rhs.less_than(lhs);
    }
}

pub mod less_than {
    use variables::int_var::IntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_than(x: IntVar, y: IntVar) -> ();
        );

}

pub mod greater_or_equal_than {
    use variables::int_var::IntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_or_equal_than(x: IntVar, y: IntVar) -> ();
        );
}
