use super::{PropagationState, Propagator};
use variables::{VariableError, VariableState};
use variables::domains::{OrderedDomain, PrunableDomain};

// using macro
#[derive(Debug, Clone)]
struct ArithmeticComparatorPropagator {}
impl Propagator for ArithmeticComparatorPropagator {}
impl ArithmeticComparatorPropagator {
    pub fn new() -> ArithmeticComparatorPropagator {
        ArithmeticComparatorPropagator {}
    }

    pub fn equal<VarType: PrunableDomain<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.equal(rhs)?;
        change = change || (r != (VariableState::NoChange, VariableState::NoChange));

        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }

    pub fn equal_on_bounds<
        Left: OrderedDomain<Type = i32>,
        Right: OrderedDomain<Type = i32>,
    >(
        &self,
        lhs: &mut Left,
        rhs: &mut Right,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.weak_upperbound(rhs.max())?;
        change = change || (r != VariableState::NoChange);
        let r = rhs.weak_upperbound(lhs.max())?;
        change = change || (r != VariableState::NoChange);
        let r = lhs.weak_lowerbound(rhs.min())?;
        change = change || (r != VariableState::NoChange);
        let r = rhs.weak_lowerbound(lhs.min())?;
        change = change || (r != VariableState::NoChange);
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }

    pub fn less_than<VarType: OrderedDomain<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.less_than(rhs)?;
        change = change || (r != (VariableState::NoChange, VariableState::NoChange));
        //if lhs.max() < rhs.min() {
        //Ok(PropagationState::Subsumed)
        //} else {
        //Ok(PropagationState::FixPoint)
        //}
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }

    pub fn less_or_equal_than<VarType: OrderedDomain<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.less_or_equal_than(rhs)?;
        change = change || (r != (VariableState::NoChange, VariableState::NoChange));
        //if lhs.max() <= rhs.min() {
        //Ok(PropagationState::Subsumed)
        //} else {
        //Ok(PropagationState::FixPoint)
        //}
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }

    pub fn greater_than<VarType: OrderedDomain<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.greater_than(rhs)?;
        change = change || (r != (VariableState::NoChange, VariableState::NoChange));
        //if lhs.min() > rhs.max() {
        //Ok(PropagationState::Subsumed)
        //} else {
        //Ok(PropagationState::FixPoint)
        //}
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }

    pub fn greater_or_equal_than<VarType: OrderedDomain<Type = i32>>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) -> Result<PropagationState, VariableError> {
        let mut change = false;
        let r = lhs.greater_or_equal_than(rhs)?;
        change = change || (r != (VariableState::NoChange, VariableState::NoChange));
        //if lhs.min() >= rhs.max() {
        //Ok(PropagationState::Subsumed)
        //} else {
        //Ok(PropagationState::FixPoint)
        //}
        if change {
            Ok(PropagationState::FixPoint)
        } else {
            Ok(PropagationState::NoChange)
        }
    }
}

pub mod less_than {
    use variables::domains::OrderedDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_than(x: VarType, y: VarType)
        where VarType: OrderedDomain<Type=i32>;
        );

}

pub mod less_or_equal_than {
    use variables::domains::OrderedDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn less_or_equal_than(x: VarType, y: VarType)
        where VarType: OrderedDomain<Type=i32>;
        );

}

pub mod greater_than {
    use variables::domains::OrderedDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_than(x: VarType, y: VarType)
        where VarType: OrderedDomain<Type=i32>;
        );

}

pub mod greater_or_equal_than {
    use variables::domains::OrderedDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn greater_or_equal_than(x: VarType, y: VarType)
        where VarType: OrderedDomain<Type=i32>;
        );
}

pub mod equal {
    use variables::domains::PrunableDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn equal(x: VarType, y: VarType)
        where VarType: PrunableDomain<Type=i32>;
        );
}

pub mod equal_on_bounds {
    use variables::domains::OrderedDomain;

    constraint_build!(
        struct Propagator = super::ArithmeticComparatorPropagator;
        fn new();
        fn equal_on_bounds(x: Left, y: Right)
        where
        Left: OrderedDomain<Type = i32>,
        Right: OrderedDomain<Type = i32>;
        );
}
