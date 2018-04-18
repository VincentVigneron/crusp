use super::{Variable, VariableError, VariableState};

// TODO min & max update

// binf -> lowerbound
// bsup -> upperbound
// prefix with unsafe for n checking already invalid var

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntVar {
    min: i32,
    max: i32,
    domain: Vec<(i32, i32)>,
}

impl Variable for IntVar {
    fn is_fixed(&self) -> bool {
        return self.min == self.max;
    }
}

impl IntVar {
    pub fn is_fixed(&self) -> bool {
        return self.min == self.max;
    }
    pub fn new(min: i32, max: i32) -> Option<IntVar> {
        let domain = vec![(min, max)];

        if min > max {
            None
        } else {
            Some(IntVar {
                min: min,
                max: max,
                domain: domain,
            })
        }
    }

    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn domain(&self) -> &Vec<(i32, i32)> {
        &self.domain
    }

    pub fn value(&self) -> Option<i32> {
        if self.domain.is_empty() {
            None
        } else if self.min == self.max {
            Some(self.min)
        } else {
            None
        }
    }

    // macros ?
    fn update_bsup(
        &mut self,
        rev_index: Option<usize>,
        new_bsup: i32,
    ) -> Result<VariableState, VariableError> {
        use std::cmp::min;
        match rev_index {
            Some(rev_index) => {
                let index = (self.domain.len() - 1) - rev_index;
                self.domain[index].1 = min(new_bsup, self.domain[index].1);
                if self.domain[index].1 < self.domain[index].0 {
                    self.domain.truncate(index);
                    if self.domain.is_empty() {
                        return Err(VariableError::DomainWipeout);
                    }
                } else {
                    self.domain.truncate(index + 1);
                }
                self.max = self.domain[self.domain.len() - 1].1;
                Ok(VariableState::BoundChange)
            }
            None => Ok(VariableState::NoChange),
        }
    }

    pub fn update_strict_bsup(
        &mut self,
        bsup: i32,
    ) -> Result<VariableState, VariableError> {
        if bsup <= self.min() {
            self.domain.clear();
            self.min = i32::max_value();
            self.max = i32::min_value();
            return Err(VariableError::DomainWipeout);
        }
        let rev_index = self.domain
            .iter()
            .rev()
            .take_while(|&&(_, max)| bsup <= max)
            .position(|&(min, _)| min <= bsup);
        self.update_bsup(rev_index, bsup - 1)
    }

    pub fn update_weak_bsup(
        &mut self,
        bsup: i32,
    ) -> Result<VariableState, VariableError> {
        if bsup < self.min() {
            self.domain.clear();
            self.min = i32::max_value();
            self.max = i32::min_value();
            return Err(VariableError::DomainWipeout);
        }
        //let rev_index = self.domain.iter().rev().position(|&(min, _)| min >= bsup);
        let rev_index = self.domain
            .iter()
            .rev()
            .take_while(|&&(_, max)| bsup <= max)
            .position(|&(min, _)| min <= bsup);
        self.update_bsup(rev_index, bsup)
    }

    fn update_binf(
        &mut self,
        index: Option<usize>,
        new_binf: i32,
    ) -> Result<VariableState, VariableError> {
        use std::cmp::max;
        match index {
            Some(index) => {
                self.domain[index].0 = max(new_binf, self.domain[index].0);
                if index > 0 {
                    let new_domain = self.domain.drain(0..index).collect();
                    self.domain = new_domain;
                }
                self.min = self.domain[0].0;
                Ok(VariableState::BoundChange)
            }
            None => Ok(VariableState::NoChange),
        }
    }

    pub fn update_strict_binf(
        &mut self,
        binf: i32,
    ) -> Result<VariableState, VariableError> {
        if binf >= self.max() {
            self.domain.clear();
            self.min = i32::max_value();
            self.max = i32::min_value();
            return Err(VariableError::DomainWipeout);
        }
        let index = self.domain.iter().rev().position(|&(min, _)| min > binf);
        self.update_binf(index, binf + 1)
    }

    pub fn update_weak_binf(
        &mut self,
        binf: i32,
    ) -> Result<VariableState, VariableError> {
        if binf > self.max() {
            self.domain.clear();
            self.min = i32::max_value();
            self.max = i32::min_value();
            return Err(VariableError::DomainWipeout);
        }
        let index = self.domain.iter().rev().position(|&(min, _)| min >= binf);
        self.update_binf(index, binf + 1)
    }

    // TODO macros ?
    pub fn less_than(
        &mut self,
        value: &mut IntVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.update_strict_bsup(value.max)?;
        let state_value = value.update_strict_binf(self.min)?;

        Ok((state_self, state_value))
    }

    pub fn less_or_equal_than(
        &mut self,
        value: &mut IntVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.update_weak_bsup(value.max)?;
        let state_value = value.update_weak_binf(self.min)?;

        Ok((state_self, state_value))
    }

    pub fn greater_than(
        &mut self,
        value: &mut IntVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.update_strict_binf(value.min)?;
        let state_value = value.update_strict_bsup(self.max)?;

        Ok((state_self, state_value))
    }

    pub fn greater_or_equal_than(
        &mut self,
        value: &mut IntVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        let state_self = self.update_weak_binf(value.min)?;
        let state_value = value.update_weak_bsup(self.max)?;

        Ok((state_self, state_value))
    }

    pub fn unsafe_set_value(&mut self, val: i32) -> () {
        self.min = val;
        self.max = val;
        self.domain = vec![(val, val)];
    }

    pub fn set_value(&mut self, val: i32) -> Result<VariableState, VariableError> {
        let in_domain = self.domain
            .iter()
            .skip_while(|&&(min, _)| val >= min)
            .take_while(|&&(_, max)| val <= max)
            .any(|&(min, max)| (val >= min) && (val <= max));
        if in_domain {
            self.min = val;
            self.max = val;
            self.domain = vec![(val, val)];
            Ok(VariableState::BoundChange)
        } else {
            Err(VariableError::DomainWipeout)
        }
    }

    pub fn equal(
        &mut self,
        value: &mut IntVar,
    ) -> Result<(VariableState, VariableState), VariableError> {
        unimplemented!()
    }

    fn unsafe_remove_value(
        &mut self,
        value: i32,
    ) -> Result<VariableState, VariableError> {
        let index = self.domain
            .iter()
            .rev()
            .position(|&(min, max)| min <= value && value <= max);
        match index {
            Some(index) => {
                if self.min == self.max {
                    self.domain.remove(index);
                } else if self.min == value {
                    self.domain[index].0 = value + 1;
                } else if self.max == value {
                    self.domain[index].1 = value - 1;
                } else {
                    self.domain[index].1 = value - 1;
                    let max_interval = (value + 1, self.max);
                    self.domain.insert(index + 1, max_interval);
                }
            }
            None => {}
        }
        unimplemented!()
    }

    pub fn remove_value(&mut self, value: i32) -> Result<VariableState, VariableError> {
        if self.min <= value && value <= self.max {
            return self.unsafe_remove_value(value);
        }
        Err(VariableError::DomainWipeout)
    }

    pub fn domain_iter(&self) -> IntVarDomainIterator {
        IntVarDomainIterator::new(self.domain.clone().into_iter())
    }
}

use std::vec;
pub struct IntVarDomainIterator {
    domain: vec::IntoIter<(i32, i32)>, //Vec<(i32, i32)>::Iterator,
    element: Option<(i32, i32)>,
}

impl IntVarDomainIterator {
    fn new(domain: vec::IntoIter<(i32, i32)>) -> IntVarDomainIterator {
        let mut domain = domain;
        let element = domain.next();
        IntVarDomainIterator {
            domain: domain,
            element: element,
        }
    }
}

impl Iterator for IntVarDomainIterator {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let val = match self.element {
            Some((min, max)) if min == max => {
                self.element = self.domain.next();
                min
            }
            Some((min, max)) => {
                self.element = Some((min + 1, max));
                min
            }
            _ => return None,
        };
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let vars = vec![(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)];
        for (min, max) in vars.into_iter() {
            let var = IntVar::new(min, max).unwrap();
            let domain = vec![(min, max)];
            assert!(var.min() == min, "min false for: \"{:?}\"", var);
            assert!(var.max() == max, "max false for: \"{:?}\"", var);
            assert!(*var.domain() == domain, "domain false for: \"{:?}\"", var);
        }
    }

    #[test]
    fn test_new_error() {
        let vars = vec![(1, 0), (100, 22), (10, 5), (15, 9), (3, 2)];
        for (min, max) in vars.into_iter() {
            let var = IntVar::new(min, max);
            match var {
                None => {}
                _ => assert!(false, "Expected error for: \"{:?}\"", var),
            }
        }
    }

    #[test]
    fn test_update_strict_binf() {
        //unimplemented!()
    }

    #[test]
    fn test_update_weak_binf() {
        //unimplemented!()
    }

    // edge case when bsup = (min=bsup,max=bsup) => remove last ellement
    #[test]
    fn test_update_valid_strict_bsup() {
        let vars = [(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)]
            .into_iter()
            .map(|&(min, max)| IntVar::new(min, max))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        let bsups = vec![1, 10, 4, 10, 3];
        let expected = [(0, 0), (-1, 9), (3, 3), (5, 9), (2, 2)]
            .into_iter()
            .map(|&(min, max)| IntVar::new(min, max))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        let results = vec![
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::NoChange),
            Ok(VariableState::NoChange),
        ];
        let iter = vars.into_iter()
            .zip(bsups.into_iter())
            .zip(expected.into_iter())
            .zip(results.into_iter())
            .map(|(((var, bsup), exp), res)| (var, bsup, exp, res));
        for (mut var, bsup, exp_var, exp_res) in iter {
            let res = var.update_strict_bsup(bsup);
            assert!(res == exp_res, "Unexpected result.");
            assert!(var == exp_var, "Unexpected domain.");
        }
    }

    #[test]
    fn test_update_invalid_strict_bsup() {
        let vars = [(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)]
            .into_iter()
            .map(|&(min, max)| IntVar::new(min, max))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        let bsups = vec![0, -5, 3, 4, 2];
        let results = vec![
            Err(VariableError::DomainWipeout),
            Err(VariableError::DomainWipeout),
            Err(VariableError::DomainWipeout),
            Err(VariableError::DomainWipeout),
            Err(VariableError::DomainWipeout),
        ];
        let iter = vars.into_iter()
            .zip(bsups.into_iter())
            .zip(results.into_iter())
            .map(|((var, bsup), res)| (var, bsup, res));
        for (mut var, bsup, exp_res) in iter {
            let res = var.update_strict_bsup(bsup);
            assert!(res == exp_res, "Unexpected result.");
        }
    }

    #[test]
    fn test_update_weak_bsup() {
        //unimplemented!()
    }

    #[test]
    fn test_unsafe_remove_value() {
        //unimplemented!()
    }

    #[test]
    fn test_less_than() {
        //unimplemented!()
    }

    #[test]
    fn test_less_or_equal_than() {
        //unimplemented!()
    }

    #[test]
    fn test_greater_than() {
        //unimplemented!()
    }

    #[test]
    fn test_greater_or_equal_than() {
        //unimplemented!()
    }

    #[test]
    fn test_equal() {
        //unimplemented!()
    }

    #[test]
    fn test_domain_iterator() {
        let vars = [(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)]
            .into_iter()
            .map(|&(min, max)| IntVar::new(min, max))
            .map(Option::unwrap)
            .collect::<Vec<_>>();
        let domains = vec![
            vec![0, 1],
            vec![
                -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
                20, 21, 22,
            ],
            vec![3, 4, 5],
            vec![5, 6, 7, 8, 9],
            vec![2],
        ];
        for (domain, expected) in vars.into_iter().zip(domains.into_iter()) {
            let tmp_domain = domain.clone();
            let tmp_expected = expected.clone();
            assert!(
                domain.domain_iter().eq(expected.into_iter()),
                "expected: {:?}for{:?}",
                tmp_expected,
                tmp_domain
            )
        }
    }

}
