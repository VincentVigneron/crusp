use variables::{Variable, VariableError, VariableState};
use variables::domains::{AssignableDomain, FiniteDomain, FromRangeDomain,
                         FromValuesDomain, IterableDomain, OrderedDomain, PrunableDomain};

#[derive(Clone, Debug, Eq, PartialEq)]
enum Domain {
    True,
    False,
    Both,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoolVar {
    domain: Domain,
    state: VariableState,
}

impl BoolVar {
    pub fn new() -> Option<BoolVar> {
        Some(BoolVar {
            domain: Domain::Both,
            state: VariableState::NoChange,
        })
    }
}

impl IterableDomain for BoolVar {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a> {
        unimplemented!()
    }
}

impl AssignableDomain for BoolVar {
    fn set_value(&mut self, value: Self::Type) -> Result<VariableState, VariableError> {
        let value = match self.domain {
            Domain::Both => value,
            Domain::True if value => {
                return Ok(VariableState::NoChange);
            }
            Domain::False if !value => {
                return Ok(VariableState::NoChange);
            }
            _ => {
                return Err(VariableError::DomainWipeout);
            }
        };
        self.domain = if value { Domain::True } else { Domain::False };
        Ok(VariableState::BoundsChange)
    }
}

impl Variable for BoolVar {
    type Type = bool;
    fn is_affected(&self) -> bool {
        self.domain == Domain::True || self.domain == Domain::False
    }

    fn get_state(&self) -> &VariableState {
        &self.state
    }

    fn retrieve_state(&mut self) -> VariableState {
        use std::mem;
        let mut state = VariableState::NoChange;
        mem::swap(&mut self.state, &mut state);
        state
    }

    fn value(&self) -> Option<Self::Type> {
        match self.domain {
            Domain::True => Some(true),
            Domain::False => Some(false),
            _ => None,
        }
    }
}

impl FiniteDomain for BoolVar {
    fn size(&self) -> usize {
        match self.domain {
            Domain::True => 1,
            Domain::False => 1,
            Domain::Both => 2,
            _ => 0,
        }
    }
}

impl OrderedDomain for BoolVar {
    fn min(&self) -> Self::Type {
        unimplemented!()
    }

    fn max(&self) -> Self::Type {
        unimplemented!()
    }

    fn strict_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        unimplemented!()
    }

    fn weak_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        unimplemented!()
    }

    fn strict_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        unimplemented!()
    }

    fn weak_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        unimplemented!()
    }
}

impl PrunableDomain for BoolVar {
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }

    fn in_sorted_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>,
    {
        unimplemented!()
    }

    fn remove_value(
        &mut self,
        value: Self::Type,
    ) -> Result<VariableState, VariableError> {
        unimplemented!()
    }

    fn remove_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool,
    {
        unimplemented!()
    }

    fn retains_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool,
    {
        unimplemented!()
    }

    fn not_equal(
        &mut self,
        value: &mut BoolVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }
}
