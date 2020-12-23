use variables::domains::{
    AssignableDomain, FiniteDomain, FromRangeDomain, FromValuesDomain, IterableDomain,
    OrderedDomain, OrderedPrunableDomain, PrunableDomain,
};
use variables::{
    Variable, VariableBuilder, VariableContainer, VariableError, VariableId,
    VariableState,
};

pub struct IntVarValues<T> {}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntVarValuesBuilder {
    domain: Vec<i32>,
}

impl IntVarValuesBuilder {
    pub fn new(min: i32, max: i32) -> Option<IntVarValuesBuilder> {
        if min > max {
            None
        } else {
            Some(IntVarValuesBuilder {
                domain: (min..(max + 1)).collect(),
            })
        }
    }
}

impl VariableBuilder for IntVarValuesBuilder {
    type Variable = IntVarValues;

    fn finalize(self, id: usize) -> IntVarValues {
        IntVarValues {
            domain: self.domain,
            id: id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntVarValues {
    domain: Vec<i32>,
    id: usize,
}
impl VariableContainer for IntVarValues {}

unsafe impl Sync for IntVarValues {}
unsafe impl Send for IntVarValues {}

impl IntVarValues {
    pub fn new(min: i32, max: i32) -> Option<IntVarValues> {
        if min > max {
            None
        } else {
            Some(IntVarValues {
                domain: (min..(max + 1)).collect(),
                id: 0,
            })
        }
    }

    fn invalidate(&mut self) {
        self.domain.clear();
    }

    fn domain_change(
        &mut self,
        prev_min: i32,
        prev_max: i32,
        prev_size: usize,
    ) -> Result<VariableState, VariableError> {
        if self.domain.is_empty() {
            self.invalidate();
            Err(VariableError::DomainWipeout)
        } else if self.size() == prev_size {
            Ok(VariableState::NoChange)
        } else if self.unchecked_min() != prev_min {
            Ok(VariableState::BoundsChange)
        } else if self.unchecked_max() != prev_max {
            Ok(VariableState::BoundsChange)
        } else {
            Ok(VariableState::ValuesChange)
        }
    }
}

impl IterableDomain for IntVarValues {
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a> {
        Box::new(self.domain.iter())
    }
}

impl FromRangeDomain for IntVarValues {
    fn new_from_range(min: Self::Type, max: Self::Type) -> Option<IntVarValues> {
        if min > max {
            None
        } else {
            Some(IntVarValues {
                domain: (min..(max + 1)).collect(),
                id: 0,
            })
        }
    }
}

impl FromValuesDomain for IntVarValues {
    fn new_from_values<Values>(values: Values) -> Option<IntVarValues>
    where
        Values: IntoIterator<Item = Self::Type>,
    {
        let mut domain = values.into_iter().collect::<Vec<_>>();
        domain.sort();
        domain.dedup();
        let domain = domain;
        if domain.is_empty() {
            None
        } else {
            Some(IntVarValues {
                domain: domain,
                id: 0,
            })
        }
    }
}

impl AssignableDomain for IntVarValues {
    fn set_value(&mut self, value: Self::Type) -> Result<VariableState, VariableError> {
        if self.unchecked_min() > value || self.unchecked_max() < value {
            //self.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let var_value = self.value();
        match var_value {
            Some(var_value) if var_value == value => Ok(VariableState::NoChange),
            _ => {
                let found_value = self.domain.binary_search(&value);
                match found_value {
                    Ok(_) => {
                        self.domain = vec![value];
                        Ok(VariableState::BoundsChange)
                    }
                    _ => {
                        self.invalidate();
                        Err(VariableError::DomainWipeout)
                    }
                }
            }
        }
    }
}

impl Variable for IntVarValues {
    type Type = i32;
    fn is_affected(&self) -> bool {
        self.domain.len() == 1
    }

    fn value(&self) -> Option<Self::Type> {
        if self.min() == self.max() {
            self.min()
        } else {
            None
        }
    }

    fn id(&self) -> VariableId {
        VariableId(self.id)
    }
}

impl FiniteDomain for IntVarValues {
    fn size(&self) -> usize {
        self.domain.len()
    }
}

impl OrderedDomain for IntVarValues {
    fn min(&self) -> Option<Self::Type> {
        self.domain.first().map(Clone::clone)
    }
    fn max(&self) -> Option<Self::Type> {
        self.domain.last().map(Clone::clone)
    }

    fn strict_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_max() < ub {
            Ok(VariableState::NoChange)
        } else if self.unchecked_min() >= ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val < ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(VariableState::BoundsChange)
        }
    }

    fn weak_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_max() <= ub {
            Ok(VariableState::NoChange)
        } else if self.unchecked_min() > ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val <= ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(VariableState::BoundsChange)
        }
    }

    fn strict_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_min() > lb {
            Ok(VariableState::NoChange)
        } else if self.unchecked_max() <= lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val > lb).unwrap();
            self.domain.drain(0..index);
            Ok(VariableState::BoundsChange)
        }
    }

    fn weak_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_min() >= lb {
            Ok(VariableState::NoChange)
        } else if self.unchecked_max() < lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val >= lb).unwrap();
            self.domain.drain(0..index);
            Ok(VariableState::BoundsChange)
        }
    }
}

impl PrunableDomain for IntVarValues {
    // Distinction between ValuesChange and BoundsChange
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.iter().map(|&v| v).collect();
        let s2: BTreeSet<_> = value.iter().map(|&v| v).collect();
        let domain: Vec<_> = s1.intersection(&s2).map(|val| *val).collect();

        if domain.is_empty() {
            self.invalidate();
            value.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let (ok_self, ok_value) = {
            let check_change = |var: &mut IntVarValues| {
                if var.size() == domain.len() {
                    VariableState::NoChange
                } else if var.unchecked_min() != unwrap_first!(domain) {
                    VariableState::BoundsChange
                } else if var.unchecked_max() != unwrap_last!(domain) {
                    VariableState::BoundsChange
                } else {
                    VariableState::ValuesChange
                }
            };
            (check_change(self), check_change(value))
        };

        self.domain = domain.clone();
        value.domain = domain;
        Ok((ok_self, ok_value))
    }

    fn in_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>,
    {
        let values: Vec<_> = values.into_iter().collect();
        let mut values: Vec<_> = values.into_iter().collect();
        values.sort();
        self.in_sorted_values(values.into_iter())
    }

    // check change function (equality, bounds, values, nochange...)
    fn remove_value(
        &mut self,
        value: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.unchecked_min() > value && self.unchecked_max() < value {
            return Ok(VariableState::NoChange);
        }
        let (min, max) = (self.min(), self.max());
        let found_value = self.domain.binary_search(&value);
        match found_value {
            Ok(index) => {
                self.domain.remove(index);
                if self.size() == 0 {
                    Err(VariableError::DomainWipeout)
                } else if self.min() != min {
                    Ok(VariableState::BoundsChange)
                } else if self.max() != max {
                    Ok(VariableState::BoundsChange)
                } else {
                    Ok(VariableState::ValuesChange)
                }
            }
            _ => Ok(VariableState::NoChange),
        }
    }

    fn remove_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool,
    {
        let (min, max, size) = (self.unchecked_min(), self.unchecked_max(), self.size());
        self.domain.retain(|v| !pred(v));
        self.domain_change(min, max, size)
    }

    fn retains_if<Predicate>(
        &mut self,
        mut pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: FnMut(&Self::Type) -> bool,
    {
        let (min, max, size) = (self.unchecked_min(), self.unchecked_max(), self.size());
        self.domain.retain(|v| pred(v));
        self.domain_change(min, max, size)
    }

    fn not_equal(
        &mut self,
        value: &mut IntVarValues,
    ) -> Result<(VariableState, VariableState), VariableError> {
        match self.value() {
            Some(val) => {
                let ok_value = value.remove_value(val)?;
                Ok((VariableState::NoChange, ok_value))
            }
            _ => match value.value() {
                Some(val) => {
                    let ok_self = self.remove_value(val)?;
                    Ok((ok_self, VariableState::NoChange))
                }
                _ => Ok((VariableState::NoChange, VariableState::NoChange)),
            },
        }
    }
}

impl OrderedPrunableDomain for IntVarValues {
    // Change to non-naive implementation
    fn in_sorted_values<Values>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>
    where
        Values: IntoIterator<Item = Self::Type>,
    {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.iter().map(|&v| v).collect();
        let s2: BTreeSet<_> = values.into_iter().collect();
        let domain: Vec<_> = s1.intersection(&s2).map(|val| *val).collect();

        if domain.is_empty() {
            self.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let ok_self = {
            let check_change = |var: &mut IntVarValues| {
                if var.size() == domain.len() {
                    VariableState::NoChange
                } else if var.unchecked_min() != unwrap_first!(domain) {
                    VariableState::BoundsChange
                } else if var.unchecked_max() != unwrap_last!(domain) {
                    VariableState::BoundsChange
                } else {
                    VariableState::ValuesChange
                }
            };
            check_change(self)
        };
        self.domain = domain;
        Ok(ok_self)
    }
}

//#[cfg(test)]
//mod tests {
//test_int_var!(IntVarValues);
//}
