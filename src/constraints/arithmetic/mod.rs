use super::{PropagationError, PropagationState, Propagator};
use variables::int_var::BoundsIntVar;

// using macro
// TODO adding a subsume state to VariableState
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
    ) -> Result<PropagationState, PropagationError> {
        match lhs.less_than(rhs) {
            Ok(_) => {
                if lhs.max() < rhs.min() {
                    Ok(PropagationState::Subsumed)
                } else {
                    Ok(PropagationState::FixPoint)
                }
            }
            Err(_) => Err(PropagationError::DomainWipeout),
        }
    }

    pub fn less_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, PropagationError> {
        match lhs.less_or_equal_than(rhs) {
            Ok(_) => {
                if lhs.max() <= rhs.min() {
                    Ok(PropagationState::Subsumed)
                } else {
                    Ok(PropagationState::FixPoint)
                }
            }
            Err(_) => Err(PropagationError::DomainWipeout),
        }
    }

    pub fn greater_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, PropagationError> {
        match lhs.greater_than(rhs) {
            Ok(_) => {
                if lhs.min() > rhs.max() {
                    Ok(PropagationState::Subsumed)
                } else {
                    Ok(PropagationState::FixPoint)
                }
            }
            Err(_) => Err(PropagationError::DomainWipeout),
        }
    }

    pub fn greater_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, PropagationError> {
        match lhs.greater_or_equal_than(rhs) {
            Ok(_) => {
                if lhs.min() >= rhs.max() {
                    Ok(PropagationState::Subsumed)
                } else {
                    Ok(PropagationState::FixPoint)
                }
            }
            Err(_) => Err(PropagationError::DomainWipeout),
        }
    }
}

pub mod less_than {
    use variables::int_var::BoundsIntVar;
    use variables::int_var::IntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_than(x: VarType, y: VarType)
        where VarType: BoundsIntVar<Type=i32> | IntVar<Type=i32>;
        );

}

pub mod less_or_equal_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_or_equal_than(x: VarType, y: VarType)
        where VarType: BoundsIntVar<Type=i32>;
        );

}

pub mod greater_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_than(x: VarType, y: VarType)
        where VarType: BoundsIntVar<Type=i32>;
        );

}

pub mod greater_or_equal_than {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_or_equal_than(x: VarType, y: VarType)
        where VarType: BoundsIntVar<Type=i32>;
        );
}
