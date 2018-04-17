use super::Variable;
use snowflake::ProcessUniqueId;

// TODO min & max update

// binf -> lowerbound
// bsup -> upperbound

#[derive(Debug, Clone)]
pub struct IntVar {
    id: ProcessUniqueId,
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
            let id = ProcessUniqueId::new();
            Some(IntVar {
                id: id,
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

    pub fn get_id(&self) -> ProcessUniqueId {
        self.id
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

    fn unsafe_update_bsup(&mut self, rev_index: Option<usize>, new_bsup: i32) -> () {
        use std::cmp::min;
        match rev_index {
            Some(rev_index) => {
                let index = (self.domain.len() - 1) - rev_index;
                self.domain[index].1 = min(new_bsup, self.domain[index].1);
                self.domain.truncate(index + 1);
            }
            None => {}
        }
    }

    pub fn unsafe_update_strict_bsup(&mut self, bsup: i32) -> () {
        let rev_index = self.domain.iter().rev().position(|&(min, _)| min > bsup);
        self.unsafe_update_bsup(rev_index, bsup - 1)
    }

    pub fn unsafe_update_weak_bsup(&mut self, bsup: i32) -> () {
        let rev_index = self.domain.iter().rev().position(|&(min, _)| min >= bsup);
        self.unsafe_update_bsup(rev_index, bsup - 1)
    }

    fn unsafe_update_binf(&mut self, index: Option<usize>, new_binf: i32) -> () {
        use std::cmp::max;
        match index {
            Some(index) => {
                self.domain[index].0 = max(new_binf, self.domain[index].0);
                if index > 0 {
                    let new_domain = self.domain.drain(0..index).collect();
                    self.domain = new_domain;
                }
            }
            None => {}
        }
    }

    pub fn unsafe_update_strict_binf(&mut self, binf: i32) -> () {
        let index = self.domain.iter().rev().position(|&(min, _)| min > binf);
        self.unsafe_update_binf(index, binf + 1)
    }

    pub fn unsafe_update_weak_binf(&mut self, binf: i32) -> () {
        let index = self.domain.iter().rev().position(|&(min, _)| min >= binf);
        self.unsafe_update_binf(index, binf + 1)
    }

    pub fn unsafe_set_value(&mut self, val: i32) -> () {
        self.min = val;
        self.max = val;
        self.domain = vec![(val, val)];
    }

    fn unsafe_remove_value(&mut self, value: i32) -> () {
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
    }

    pub fn less_than(&mut self, value: &mut IntVar) -> Option<i32> {
        //use std::cmp::{max,min};

        if self.min > value.max {
            return None;
        }
        if self.min >= value.min {
            value.unsafe_update_strict_binf(self.min);
            value.min = value.domain[0].0;
        }
        if self.max >= value.max {
            self.unsafe_update_strict_bsup(value.max);
            self.max = self.domain[self.domain.len() - 1].1;
        }

        None
    }

    pub fn remove_value(&mut self, value: i32) -> Option<i32> {
        if self.min <= value && value <= self.max {
            self.unsafe_remove_value(value);
        }
        None
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
    fn test_unsafe_update_strict_binf() {
        unimplemented!()
    }

    #[test]
    fn test_unsafe_update_weak_binf() {
        unimplemented!()
    }

    #[test]
    fn test_unsafe_update_strict_bsup() {
        unimplemented!()
    }

    #[test]
    fn test_unsafe_update_weak_bsup() {
        unimplemented!()
    }

    #[test]
    fn test_unsafe_remove_value() {
        unimplemented!()
    }

    #[test]
    fn test_less_than() {
        unimplemented!()
    }

    #[test]
    fn test_less_or_equal_than() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than() {
        unimplemented!()
    }

    #[test]
    fn test_greater_or_equal_than() {
        unimplemented!()
    }

    #[test]
    fn test_equal() {
        unimplemented!()
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
