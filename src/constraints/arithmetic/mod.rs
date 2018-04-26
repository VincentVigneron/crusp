use super::{PropagationState, Propagator};
use variables::VariableError;
use variables::int_var::{BoundsIntVar, ValuesIntVar};

// using macro
#[derive(Debug, Clone)]
struct ArithmeticComparatorPropagator {}
impl Propagator for ArithmeticComparatorPropagator {}
impl ArithmeticComparatorPropagator {
    pub fn new() -> ArithmeticComparatorPropagator {
        ArithmeticComparatorPropagator {}
    }

    pub fn equal<VarType: ValuesIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let _ = lhs.equal(rhs)?;
        Ok(PropagationState::FixPoint)
    }

    pub fn equal_on_bounds<
        Left: BoundsIntVar<Type = i32>,
        Right: BoundsIntVar<Type = i32>,
    >(
        &self,
        lhs: &mut Left,
        rhs: &mut Right,
    ) -> Result<PropagationState, VariableError> {
        let _ = lhs.weak_upperbound(rhs.max())?;
        let _ = rhs.weak_upperbound(lhs.max())?;
        let _ = lhs.weak_lowerbound(rhs.min())?;
        let _ = rhs.weak_lowerbound(lhs.min())?;
        Ok(PropagationState::FixPoint)
    }

    pub fn less_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let (_, _) = lhs.less_than(rhs)?;
        if lhs.max() < rhs.min() {
            Ok(PropagationState::Subsumed)
        } else {
            Ok(PropagationState::FixPoint)
        }
    }

    pub fn less_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let (_, _) = lhs.less_or_equal_than(rhs)?;
        if lhs.max() <= rhs.min() {
            Ok(PropagationState::Subsumed)
        } else {
            Ok(PropagationState::FixPoint)
        }
    }

    pub fn greater_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let (_, _) = lhs.greater_than(rhs)?;
        if lhs.min() > rhs.max() {
            Ok(PropagationState::Subsumed)
        } else {
            Ok(PropagationState::FixPoint)
        }
    }

    pub fn greater_or_equal_than<VarType: BoundsIntVar<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let (_, _) = lhs.greater_or_equal_than(rhs)?;
        if lhs.min() >= rhs.max() {
            Ok(PropagationState::Subsumed)
        } else {
            Ok(PropagationState::FixPoint)
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

pub mod equal {
    use variables::int_var::ValuesIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn equal(x: VarType, y: VarType)
        where VarType: ValuesIntVar<Type=i32>;
        );
}

pub mod equal_on_bounds {
    use variables::int_var::BoundsIntVar;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn equal_on_bounds(x: Left, y: Right)
        where
        Left: BoundsIntVar<Type = i32>,
        Right: BoundsIntVar<Type = i32>;
        );
}
