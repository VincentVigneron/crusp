use super::{Variable, VariableError, VariableState};

// BoundedVar (min, max)
// SizedVar (size)
// OrderedVar (Actual BoundsIntVar)
// EqualityVar (Actual BoundsIntVar)
// DisequalityVar (Actual BoundsIntVar)
// remove && in ??
//
// Bounds => min max new_from_range
// Iterable => iter new_from_values

pub trait FiniteDomain: Variable + Clone {
    type Type: Clone;

    fn value(&self) -> Option<Self::Type>;
    fn size(&self) -> usize;
}

pub trait IterableDomain: FiniteDomain {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a>;
}

//pub trait IterableDomain: FiniteDomain + IntoIterator<Item = Self::Type> {
//fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a>;
//}

pub trait FromRangeDomain: FiniteDomain {
    fn new_from_range(min: Self::Type, max: Self::Type) -> Option<Self>;
}

pub trait FromValuesDomain: FiniteDomain + Sized {
    fn new_from_values<Values>(values: Values) -> Option<Self>
    where
        Values: IntoIterator<Item = Self::Type>;
}

pub trait AssignableDomain: FiniteDomain {
    fn set_value(&mut self, value: Self::Type) -> Result<VariableState, VariableError>;
}

pub trait OrderedDomain: FiniteDomain
where
    Self::Type: Ord + Eq,
{
    fn min(&self) -> Self::Type;
    fn max(&self) -> Self::Type;
    fn strict_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError>;
    fn weak_upperbound(&mut self, ub: Self::Type)
        -> Result<VariableState, VariableError>;
    fn strict_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError>;
    fn weak_lowerbound(&mut self, lb: Self::Type)
        -> Result<VariableState, VariableError>;
    fn less_than(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_upperbound(value.max())?;
        let state_value = value.strict_lowerbound(self.min())?;

        Ok((state_self, state_value))
    }
    fn less_or_equal_than(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_upperbound(value.max())?;
        let state_value = value.weak_lowerbound(self.min())?;

        Ok((state_self, state_value))
    }
    fn greater_than(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_lowerbound(value.min())?;
        let state_value = value.strict_upperbound(self.max())?;

        Ok((state_self, state_value))
    }
    fn greater_or_equal_than(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_lowerbound(value.min())?;
        let state_value = value.weak_upperbound(self.max())?;

        Ok((state_self, state_value))
    }
    fn equal_bounds(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        // invalide atm
        let _ = value.less_or_equal_than(self)?;
        self.less_or_equal_than(value)
    }
}

pub trait PrunableDomain: FiniteDomain
where
    Self::Type: Ord + Eq,
{
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    fn not_equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    fn in_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>,
    {
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        self.in_sorted_values(values.into_iter())
    }
    fn in_sorted_values<Values: Iterator<Item = Self::Type>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>;
    fn remove_value(&mut self, value: Self::Type)
        -> Result<VariableState, VariableError>;
    fn remove_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool;
    fn retains_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool;
}
