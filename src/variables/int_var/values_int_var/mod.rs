use variables::{Variable, VariableError, VariableState};
use variables::int_var::{BoundsIntVar, IntVar, ValuesIntVar};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetIntVar {
    domain: Vec<i32>,
}

// TODO Add checking to avoid unecessary computation
impl SetIntVar {
    pub fn new(min: i32, max: i32) -> Option<SetIntVar> {
        if min > max {
            None
        } else {
            Some(SetIntVar {
                domain: (min..(max + 1)).collect(),
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
        } else if self.min() != prev_min {
            Ok(VariableState::BoundChange)
        } else if self.max() != prev_max {
            Ok(VariableState::BoundChange)
        } else {
            Ok(VariableState::ValuesChange)
        }
    }
}

impl Variable for SetIntVar {
    fn is_fixed(&self) -> bool {
        self.domain.len() == 1
    }
}

impl IntVar for SetIntVar {
    type Type = i32;
    fn size(&self) -> usize {
        self.domain.len()
    }

    fn min(&self) -> Self::Type {
        *self.domain.first().unwrap()
    }

    fn max(&self) -> Self::Type {
        *self.domain.last().unwrap()
    }

    fn value(&self) -> Option<Self::Type> {
        if self.domain.is_empty() {
            None
        } else if self.min() == self.max() {
            Some(self.min())
        } else {
            None
        }
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a> {
        Box::new(self.domain.iter())
    }
}

impl BoundsIntVar for SetIntVar {
    fn new_from_range(min: Self::Type, max: Self::Type) -> Option<SetIntVar> {
        if min > max {
            None
        } else {
            Some(SetIntVar {
                domain: (min..(max + 1)).collect(),
            })
        }
    }

    fn strict_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.max() < ub {
            Ok(VariableState::NoChange)
        } else if self.min() >= ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val < ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(VariableState::BoundChange)
        }
    }

    fn weak_upperbound(
        &mut self,
        ub: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.max() <= ub {
            Ok(VariableState::NoChange)
        } else if self.min() > ub {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().rposition(|&val| val <= ub).unwrap();
            self.domain.truncate(index + 1);
            Ok(VariableState::BoundChange)
        }
    }

    fn strict_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.min() > lb {
            Ok(VariableState::NoChange)
        } else if self.max() <= lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val > lb).unwrap();
            self.domain.truncate(index);
            Ok(VariableState::BoundChange)
        }
    }

    fn weak_lowerbound(
        &mut self,
        lb: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.min() <= lb {
            Ok(VariableState::NoChange)
        } else if self.max() > lb {
            Err(VariableError::DomainWipeout)
        } else {
            let index = self.domain.iter().position(|&val| val >= lb).unwrap();
            self.domain.truncate(index);
            Ok(VariableState::BoundChange)
        }
    }
}

impl ValuesIntVar for SetIntVar {
    fn new_from_values<Values: Iterator<Item = Self::Type>>(
        values: Values,
    ) -> Option<SetIntVar> {
        let mut domain = values.collect::<Vec<_>>();
        domain.sort();
        domain.dedup();
        let domain = domain;
        if domain.is_empty() {
            None
        } else {
            Some(SetIntVar { domain: domain })
        }
    }

    fn set_value(&mut self, value: Self::Type) -> Result<VariableState, VariableError> {
        if self.min() > value || self.max() < value {
            self.invalidate();
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
                        Ok(VariableState::BoundChange)
                    }
                    _ => {
                        self.invalidate();
                        Err(VariableError::DomainWipeout)
                    }
                }
            }
        }
    }

    // Distinction between ValuesChange and BoundChange
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
            let check_change = |var: &mut SetIntVar| {
                if var.size() == domain.len() {
                    VariableState::NoChange
                } else if var.min() != *domain.first().unwrap() {
                    VariableState::BoundChange
                } else if var.max() != *domain.last().unwrap() {
                    VariableState::BoundChange
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

    // Change to non-naive implementation
    fn in_sorted_values<Values: Iterator<Item = Self::Type>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError> {
        use std::collections::BTreeSet;
        let s1: BTreeSet<_> = self.iter().map(|&v| v).collect();
        let s2: BTreeSet<_> = values.collect();
        let domain: Vec<_> = s1.intersection(&s2).map(|val| *val).collect();

        if domain.is_empty() {
            self.invalidate();
            return Err(VariableError::DomainWipeout);
        }
        let ok_self = {
            let check_change = |var: &mut SetIntVar| {
                if var.size() == domain.len() {
                    VariableState::NoChange
                } else if var.min() != *domain.first().unwrap() {
                    VariableState::BoundChange
                } else if var.min() != *domain.first().unwrap() {
                    VariableState::BoundChange
                } else {
                    VariableState::ValuesChange
                }
            };
            check_change(self)
        };
        Ok(ok_self)
    }

    // check change function (equality, bounds, values, nochange...)
    fn remove_value(
        &mut self,
        value: Self::Type,
    ) -> Result<VariableState, VariableError> {
        if self.min() > value && self.max() < value {
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
                    Ok(VariableState::BoundChange)
                } else if self.max() != max {
                    Ok(VariableState::BoundChange)
                } else {
                    Ok(VariableState::ValuesChange)
                }
            }
            _ => Ok(VariableState::NoChange),
        }
    }

    fn remove_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: Fn(&Self::Type) -> bool,
    {
        let (min, max, size) = (self.min(), self.max(), self.size());
        self.domain.retain(|v| !pred(v));
        self.domain_change(min, max, size)
    }

    fn retains_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: Fn(&Self::Type) -> bool,
    {
        let (min, max, size) = (self.min(), self.max(), self.size());
        self.domain.retain(|v| pred(v));
        self.domain_change(min, max, size)
    }

    fn not_equal(
        &mut self,
        value: &mut SetIntVar,
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

    //fn into_iter(Self) -> Box<Iterator<Item = Self::Type>> {
    //Box::new(self.domain.into_iter())
    //unimplemented!()
    //}
}

#[cfg(test)]
mod tests {
    test_int_var!(SetIntVar);
}
