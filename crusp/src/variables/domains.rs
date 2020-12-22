use super::{Variable, VariableError, VariableState};




/// Trait that defines variables with finite domains. In other words the number of elements
/// of the domain is countable). Every variable should have a finite domain.
pub trait FiniteDomain<Type>: Variable<Type> {
    /// The number of elements of the domain.
    fn size(&self) -> Type;
}

/// Trait that definies variable allowing to iter through the elements of its domain.
pub trait IterableDomain<Type>: FiniteDomain<Type> {
    /// Returns an `Iterator` over the elements of the domain.
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Type> + 'a>;
}

/// Trait that defines variable that can be assigned to a specific value.
pub trait AssignableDomainEvents<Type, Events>: Variable<Type>
    // deduced from the space variable events
    // events is required as a type parameter
    where Events: VariableEvents,
{
    /// Change the value of the variable.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if value is not inside the domain, otherwise returns the correct `VariableState`;
    ///
    /// # Argument
    /// * `value` - The value to assign.
    fn set_value(&mut self, events: &mut Events, value: Type) -> Result<VariableState, VariableError> {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.set_value_register(&mut events, value))
    }
}

/// Trait that defines variable that can be assigned to a specific value.
pub trait AssignableDomain<Type>
{
    /// Change the value of the variable.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if value is not inside the domain, otherwise returns the correct `VariableState`;
    ///
    /// # Argument
    /// * `value` - The value to assign.
    fn set_value(&mut self, value: Type) -> Result<VariableState, VariableError>;
}

impl<Type,Events> AssignableDomain<Type> for AssignableDomainEvents<Type, Events>
    where Events: VariableEvents,
{
    fn set_value(&mut self, value: Type) -> Result<VariableState, VariableError> {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.set_value_register(&mut events, value))
    }
}

/// Trait that defines variable which the underlying `Type` implements the `Ord`
/// trait (i.e. the underlying type is totally ordered).
pub trait OrderedDomainEvents<Type, Events>: FiniteDomainEvents<Type, Events>
where
    Type: Ord + Eq,
    Events: VariableEvents,
{
    /// Returns the minimal value of the domain.
    fn min(&self) -> Option<Type>;
    /// Returns the maximal value of the domain.
    fn max(&self) -> Option<Type>;
    fn unchecked_min(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.min().expect(&error)
    }
    fn unchecked_max(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.max().expect(&error)
    }
    /// Forces the upperbound of the variable to be strictly lesser than `ub`.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if the new upperbound is lesser than the minimal value of the domain, otherwise
    /// returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `ub` - The strict upperbound implied to the domain.
    fn strict_upperbound(
        &mut self,
        events: &mut Events,
        ub: Type,
    ) -> Result<VariableState, VariableError>;
    /// Forces the upperbound of the variable to be lesser than `ub`.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if the new upperbound is strictly lesser than the minimal value of the domain, otherwise
    /// returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `ub` - The weak upperbound implied to the domain.
    fn weak_upperbound(
        &mut self,
        events: &mut Events,
        ub: Type,
    ) -> Result<VariableState, VariableError>;
    /// Forces the lowerbound of the variable to be strictly greater than `lb`.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if the new lowerbound is greater than the maximal value of the domain, otherwise
    /// returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `lb` - The strict lowerbound implied to the domain.

    fn strict_lowerbound(
        &mut self,
        events: &mut Events,
        lb: Type,
    ) -> Result<VariableState, VariableError>;

    /// Forces the lowerbound of the variable to be greater than `lb`.
    /// Returns an error of type `VariableError::DomainWipeout`
    /// if the new lowerbound is strictly lesser than the maximal value of the domain, otherwise
    /// returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `lb` - The weak lowerbound implied to the domain.
    fn weak_lowerbound(
        &mut self,
        events: &mut Events,
        lb: Type,
    ) -> Result<VariableState, VariableError>;
}

/// Trait that defines variable which the underlying `Type` implements the `Ord`
/// trait (i.e. the underlying type is totally ordered).
pub trait OrderedDomain<Type>: FiniteDomain<Type>
where
    Type: Ord + Eq,
{
    /// Returns the minimal value of the domain.
    fn min(&self) -> Option<Type>;
    /// Returns the maximal value of the domain.
    fn max(&self) -> Option<Type>;
    fn unchecked_min(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.min().expect(&error)
    }
    fn unchecked_max(&self) -> Type {
        let error = format!(
            "Call unchecked_min on a variable with an empty domain (line {}).",
            line!()
        );
        self.max().expect(&error)
    }
    fn strict_upperbound(
        &mut self,
        ub: Type,
    ) -> Result<VariableState, VariableError>;
    fn weak_upperbound(
        &mut self,
        ub: Type,
    ) -> Result<VariableState, VariableError>;
    fn strict_lowerbound(
        &mut self,
        lb: Type,
    ) -> Result<VariableState, VariableError>;
    fn weak_lowerbound(
        &mut self,
        lb: Type,
    ) -> Result<VariableState, VariableError>;
}

impl<Type> OrderedDomain<Type, Events> for OrderedDomainEvents<Type,Events>
where
    Type: Ord + Eq,
    Events: VariableEvents,
{
    /// Returns the minimal value of the domain.
    fn min(&self) -> Option<Type> {
        (self as OrderedDomainEvents<Type,Events>).min()
    }
    /// Returns the maximal value of the domain.
    fn max(&self) -> Option<Type> {
        (self as OrderedDomainEvents<Type,Events>).max()
    }
    fn unchecked_min(&self) -> Type {
        (self as OrderedDomainEvents<Type,Events>).unchecked_min()
    }
    fn unchecked_max(&self) -> Type {
        (self as OrderedDomainEvents<Type,Events>).unchecked_max()
    }
    fn strict_upperbound(
        &mut self,
        ub: Type,
    ) -> Result<VariableState, VariableError>  {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.strict_upperbound(&mut events, ub))
    }
    fn weak_upperbound(
        &mut self,
        ub: Type,
    ) -> Result<VariableState, VariableError>  {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.weak_upperbound(&mut events, ub))
    }
    fn strict_lowerbound(
        &mut self,
        lb: Type,
    ) -> Result<VariableState, VariableError>  {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.strict_lowerbound(&mut events, ub))
    }
    fn weak_lowerbound(
        &mut self,
        lb: Type,
    ) -> Result<VariableState, VariableError>  {
        let mut events = NoOpVariableEvents{};
        events.push(self.id(), self.weak_lowerbound(&mut events, ub))
    }
}

trait BoundedDomainEvents<Type, Events, Other=Self>: OrderedDomainEvents<Type, Events> where
    Type: Ord + Eq,
    Other: OrderedDomain<Type>,
    Events: VariableEvents,
{
    /// Forces the domain of `self` to satisfies a precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater or equal to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_than(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(), VariableError>
    {
        let state_self = events.push(self.id(), self.strict_upperbound(value.unchecked_max()))?;
        let state_value = events.push(value.id(), value.strict_lowerbound(self.unchecked_min()))?;
        Ok((state_self, state_value))
    }
    /// Forces the domain of `self` to satisfies a weak precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_or_equal_than(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(), VariableError>
    {
        let state_self = events.push(self.id(), self.strict_upperbound(value.unchecked_max()))?;
        let state_value = events.push(value.id(), value.strict_lowerbound(self.unchecked_min()))?;
        Ok((state_self, state_value))
    }
    /// Forces the domain of `value` to satisfies a strict precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater or equal to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_than(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(), VariableError>
    {
        let state_self = events.push(self.id(), self.strict_lowerbound(value.unchecked_min()))?;
        let state_value = events.push(value.id(), value.strict_upperbound(self.unchecked_max()))?;
        Ok((state_self, state_value))
    }

    /// Forces the domain of `value` to satisfies a weak precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_or_equal_than(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(), VariableError>
    {
        let state_self = events.push(self.id(), self.weak_lowerbound(value.unchecked_min()))?;
        let state_value = events.push(value.id(), value.weak_upperbound(self.unchecked_max()))?;
        Ok((state_self, state_value))
    }
    /// Forces the domains of two variables two have the same bounds (the does not imply to have
    /// the same domain).
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the two variables can't have the same bounds (i.e. no common value),
    /// otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal_bounds_lazy(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError>
    {
        let (x1,y1) = self.less_or_equal_than_register(events, value)?;
        let (x2,y2) = self.greater_or_equal_than_register(events, value)?;

        Ok((x1|x2, y1|y2))
    }

    fn equal_bounds(
        &mut self,
        events: &mut Events,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError>
    {
        let mut x = VariableState::NoChange;
        let mut y = VariableState::NoChange;
        loop {
            todo!()
            // collect error into events here
            let (x1,y1) = self.less_or_equal_than(value)?;
            let (x2,y2) = self.greater_or_equal_than(value)?;
            let new_x = x1 | x2;
            let new_y = y1 | y2;
            if (new_x == VariableState::NoChange) && (new_y == VariableState::NoChange) {
                break;
            }
            x = x | new_x;
            y = y | new_y;
        }
        events.push_value(self.id(), x);
        events.push_value(value.id(), y);
        Ok((x,y))
    }
}


trait BoundedDomain<Type, Other=Self>: OrderedDomainEvents<Type> where
    Type: Ord + Eq,
    Other: OrderedDomain<Type>,
{
    /// Forces the domain of `self` to satisfies a precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater or equal to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_upperbound(value.unchecked_max())?;
        let state_value = value.strict_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    /// Forces the domain of `self` to satisfies a weak precedence relation
    /// with `value`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `self` is greater to the maximal
    /// value of `value`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn less_or_equal_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_upperbound(value.unchecked_max())?;
        let state_value = value.strict_lowerbound(self.unchecked_min())?;

        Ok((state_self, state_value))
    }
    /// Forces the domain of `value` to satisfies a strict precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater or equal to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.strict_lowerbound(value.unchecked_min())?;
        let state_value = value.strict_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }
    /// Forces the domain of `value` to satisfies a weak precedence relation
    /// with `self`.
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the minimal value of `value` is greater to the maximal
    /// value of `self`, otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn greater_or_equal_than(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.weak_lowerbound(value.unchecked_min())?;
        let state_value = value.weak_upperbound(self.unchecked_max())?;

        Ok((state_self, state_value))
    }
    /// Forces the domains of two variables two have the same bounds (the does not imply to have
    /// the same domain).
    /// Returns an error of type `VariableError::DomainWipeout` if
    /// the two variables can't have the same bounds (i.e. no common value),
    /// otherwise returns the correct `VariableState`.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal_bounds_lazy(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError>
    {
        let (x1,y1) = self.less_or_equal_than(value)?;
        let (x2,y2) = self.greater_or_equal_than(value)?;

        Ok((x1|x2, y1|y2))
    }
    fn equal_bounds(
        &mut self,
        value: &mut Other,
    ) -> Result<(VariableState, VariableState), VariableError>
    {
        let mut x = VariableState::NoChange;
        let mut y = VariableState::NoChange;
        loop {
            let (x1,y1) = self.less_or_equal_than(value)?;
            let (x2,y2) = self.greater_or_equal_than(value)?;
            let new_x = x1 | x2;
            let new_y = y1 | y2;
            if (new_x == VariableState::NoChange) && (new_y == VariableState::NoChange) {
                break;
            }
            x = x | new_x;
            y = y | new_y;
        }
        Ok((x,y))
    }
}


/// Trait that definies variable that allows to remove any values from its domains.
pub trait PrunableDomain: FiniteDomain<Type>
where
    Type: Eq,
{
    /// Forces the domain of two variables to be equal.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn equal<Events>(
        &mut self,
        events: &mut Events,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    /// Forces the value of two varaibles to be distinct.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn not_equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>;
    /// Remove the value from the domain of a variable.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_value(&mut self, value: Self::Type)
        -> Result<VariableState, VariableError>;
    /// Remove the values of the domain that satisfies the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn remove_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool;
    /// Remove the values of the domain that does not satisfy the predicate.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn retains_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool;
}

/// Trait that definies variable that allows to remove any values from its domains.
pub trait OrderedPrunableDomain: PrunableDomain + OrderedDomain
where
    Self::Type: Eq + Ord,
{
    /// Forces the domain of the variables to be in the values past has parameter.
    ///
    /// # Parameters
    /// * `value` - The variable to compare to.
    fn in_sorted_values<Values: Iterator<Item = Self::Type>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>;
}



/*
/// Trait that defines variableswhich the domain can be deduced from an interval.
pub trait FromRangeDomain: FiniteDomain {
    /// Returns a new variable from an interval or return `None` if the interval is not valid (max <
    /// min). The domain of the new created variable contains `min` and `max`.
    ///
    /// # Parameters
    /// * `min` - The minimal value of the interval.
    /// * `max` - The maximal value of the interval.
    fn new_from_range(min: Self::Type, max: Self::Type) -> Option<Self>;
}

/// Trait that defines variable which the domain can be deduced from a list of values.
pub trait FromValuesDomain: FiniteDomain + Sized {
    /// Returns a new variable from an `Iterator` of values or `None` if the list
    /// of values is empty.
    ///
    /// # Parameters
    /// * `values` - The values of the domain.
    fn new_from_values<Values>(values: Values) -> Option<Self>
    where
        Values: IntoIterator<Item = Self::Type>;
}
*/
