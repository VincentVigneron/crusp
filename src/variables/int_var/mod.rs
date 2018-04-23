use super::{Variable, VariableError, VariableState};

// BoundedVar (min, max)
// SizedVar (size)
// OrderedVar (Actual BoundsIntVar)
// EqualityVar (Actual BoundsIntVar)
// DisequalityVar (Actual BoundsIntVar)
// remove && in ??

pub trait IntVar: Variable {
    type Type;
    fn size(&self) -> usize;
    fn min(&self) -> Self::Type;
    fn max(&self) -> Self::Type;
    fn value(&self) -> Option<Self::Type>;
}

pub trait BoundsIntVar: IntVar {
    fn new_from_range(min: Self::Type, max: Self::Type) -> Option<Self>;
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
        let state_self = self.strict_upperbound(value.max())?;
        let state_value = value.strict_lowerbound(self.min())?;

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
        let state_self = self.strict_lowerbound(value.min())?;
        let state_value = value.strict_upperbound(self.max())?;

        Ok((state_self, state_value))
    }

    //fn size(&self) -> usize {
    //IntVar::size(self)
    //}
    //fn min(&self) -> usize {
    //IntVar::min(self)
    //}
    //fn max(&self) -> usize {
    //IntVar::max(self)
    //}
    //fn value(&self) -> Option<Self::Type> {
    //IntVar::value(self)
    //}
}

pub trait ValuesIntVar: BoundsIntVar {
    fn new_from_values<Values: Iterator<Item = Self::Type>>(
        values: Values,
    ) -> Option<Self>;
    fn set_value(&mut self, value: Self::Type) -> Result<VariableState, VariableError>;
    fn equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    fn not_equal(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError>;
    fn in_values<Values: Iterator<Item = Self::Type>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError> {
        let values: Vec<_> = values.collect();
        //self.in_sorted_values(values.into_iter())
        //self.in_sorted_values(values.iter())
        unimplemented!()
    }
    fn in_sorted_values<Values: Iterator<Item = Self::Type>>(
        &mut self,
        values: Values,
    ) -> Result<VariableState, VariableError>;
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
    fn iter(&self) -> Box<Iterator<Item = &Self::Type>>;
    //fn into_iter(Self) -> Box<Iterator<Item = Self::Type>>;
    //fn strict_upperbound(&mut self, ub: Self::Type) -> Result<VariableState, VariableError> {
    //BoundsIntVar::strict_upperbound(self, ub)
    //}
    //fn weak_upperbound(&mut self, ub: Self::Type) -> Result<VariableState, VariableError> {
    //BoundsIntVar::weak_upperbound(self, ub)
    //}
    //fn strict_lowerbound(&mut self, lb: Self::Type) -> Result<VariableState, VariableError> {
    //BoundsIntVar::strict_lowerbound(self, lb)
    //}
    //fn weak_lowerbound(&mut self, lb: Self::Type) -> Result<VariableState, VariableError> {
    //BoundsIntVar::weak_lowerbound(self, lb)
    //}
    //fn less_than(
    //&mut self,
    //value: &mut Self,
    //) -> Result<(VariableState, VariableState), VariableError> {
    //BoundsIntVar::less_than(self, value)
    //}
    //fn less_or_equal_than(
    //&mut self,
    //value: &mut Self,
    //) -> Result<(VariableState, VariableState), VariableError> {
    //BoundsIntVar::less_or_equal_than(self, value)
    //}
    //fn greater_than(
    //&mut self,
    //value: &mut Self,
    //) -> Result<(VariableState, VariableState), VariableError> {
    //BoundsIntVar::greater_than(self, value)
    //}
    //fn greater_or_equal_than(
    //&mut self,
    //value: &mut Self,
    //) -> Result<(VariableState, VariableState), VariableError> {
    //BoundsIntVar::greater_or_equal_than(self, value)
    //}
    //fn size(&self) -> usize {
    //BoundsIntVar::size(self)
    //}
    //fn min(&self) -> usize {
    //BoundsIntVar::min(self)
    //}
    //fn max(&self) -> usize {
    //BoundsIntVar::max(self)
    //}
    //fn value(&self) -> Option<Self::Type> {
    //BoundsIntVar::value(self)
    //}
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    // TODO test maxvalue
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
                _ => assert!(false, "Expected none for: \"{:?}\"", var),
            }
        }
    }

    // TODO refactoring
    #[test]
    fn test_new_from_iterator() {
        use rand::{thread_rng, Rng};
        let domains = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 5, 7, 8, 9],
            vec![1, 2, 3, 5, 6, 9],
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 5, 7, 9],
            vec![1],
        ];
        let expected_domains = vec![
            vec![(1, 9)],
            vec![(1, 3), (5, 5), (7, 9)],
            vec![(1, 3), (5, 6), (9, 9)],
            vec![(1, 1), (3, 9)],
            vec![(1, 1), (5, 5), (7, 7), (9, 9)],
            vec![(1, 1)],
        ];
        let names = vec![
            "consectuive sorted values",
            "middle isolated value",
            "last isolated",
            "first isolated",
            "only isolated values",
            "singleton domain",
        ];
        let tests = domains
            .clone()
            .into_iter()
            .zip(expected_domains.clone().into_iter())
            .zip(names.clone().into_iter())
            .map(|((domain, expected_domain), name)| (domain, expected_domain, name));
        for (domain, expected_domain, name) in tests {
            let var = IntVar::new_from_iterator(domain.into_iter());
            match var {
                Some(var) => assert!(
                    *var.domain() == expected_domain,
                    "Expected {:?} domain for {:?} found {:?}",
                    expected_domain,
                    name,
                    var.domain()
                ),
                _ => assert!(false, "Expected some variable for: \"{:?}\"", name),
            }
        }
        let mut rng = thread_rng();

        for _ in 0..100 {
            let tests = domains
                .clone()
                .into_iter()
                .zip(expected_domains.clone().into_iter())
                .zip(names.clone().into_iter())
                .map(|((domain, expected_domain), name)| (domain, expected_domain, name));
            for (mut domain, expected_domain, name) in tests {
                rng.shuffle(&mut domain);
                let var = IntVar::new_from_iterator(domain.into_iter());
                match var {
                    Some(var) => assert!(
                        *var.domain() == expected_domain,
                        "Expected {:?} domain for {:?} found {:?}",
                        expected_domain,
                        name,
                        var.domain()
                    ),
                    _ => assert!(false, "Expected some variable for: \"{:?}\"", name),
                }
            }
        }
    }

    #[test]
    fn test_new_from_iterator_error() {
        let domain: Vec<Self::Type> = Vec::new();
        assert!(
            IntVar::new_from_iterator(domain.into_iter()).is_none(),
            "Expected for building from an empty iterator"
        )
    }

    #[test]
    fn test_size() {
        // comparaison between themselves
        let mut domains = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 5, 7, 8, 9],
            vec![1, 2, 3, 5, 6, 9],
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 5, 7, 9],
            vec![1],
            vec![8, 9],
            vec![0, 11],
        ];
        for domain in domains.into_iter() {
            let exp_size = domain.len();
            let var = IntVar::new_from_iterator(domain.into_iter()).unwrap();
            assert!(
                var.size() == exp_size,
                "Expected size {:?} for {:?} found {:?}.",
                exp_size,
                var,
                var.size()
            );
        }
    }

    #[test]
    fn test_strict_lowerbound() {
        unimplemented!()
    }

    #[test]
    fn test_strict_lowerbound() {
        unimplemented!()
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
            let res = var.strict_upperbound(bsup);
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
            let res = var.strict_upperbound(bsup);
            assert!(res == exp_res, "Unexpected result.");
        }
    }

    #[test]
    fn test_strict_upperbound() {
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
    fn test_equals() {
        // comparaison between themselves
        let mut domains = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 5, 7, 8, 9],
            vec![1, 2, 3, 5, 6, 9],
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 5, 7, 9],
            vec![1],
            vec![8, 9],
            vec![0, 11],
        ];
        for domain in domains.iter_mut() {
            domain.sort();
        }
        let domains = domains;
        for domain1 in domains.iter() {
            for domain2 in domains.iter() {
                let mut var1 =
                    IntVar::new_from_iterator(domain1.clone().into_iter()).unwrap();
                let mut var2 =
                    IntVar::new_from_iterator(domain2.clone().into_iter()).unwrap();
                let res = var1.equals(&mut var2);
                let dom_eq = domain1
                    .iter()
                    .filter(|&&val| domain2.contains(&val))
                    .map(|val| *val)
                    .collect::<Vec<_>>();
                if dom_eq.is_empty() {
                    let exp_res = Err(VariableError::DomainWipeout);
                    assert!(
                        res == exp_res,
                        "Expected {:?} for {:?}.equals({:?}) found {:?}",
                        exp_res,
                        var1,
                        var2,
                        res
                    );
                } else {
                    let var_res =
                        IntVar::new_from_iterator(dom_eq.clone().into_iter()).unwrap();
                    assert!(
                        var1 == var_res,
                        "Expected {:?} equals to {:?}",
                        var1,
                        var_res
                    );
                    assert!(
                        var2 == var_res,
                        "Expected {:?} equals to {:?}",
                        var2,
                        var_res
                    );
                    let ok1 = if domain1.iter().map(|val| *val).eq(var1.domain_iter()) {
                        VariableState::NoChange
                    } else if domain1.first() != dom_eq.first() {
                        VariableState::BoundChange
                    } else if domain1.last() != dom_eq.last() {
                        VariableState::BoundChange
                    } else {
                        VariableState::ValuesChange
                    };
                    let ok2 = if domain2.iter().map(|val| *val).eq(var2.domain_iter()) {
                        VariableState::NoChange
                    } else if domain2.first() != dom_eq.first() {
                        VariableState::BoundChange
                    } else if domain2.last() != dom_eq.last() {
                        VariableState::BoundChange
                    } else {
                        VariableState::ValuesChange
                    };
                    let exp_res = Ok((ok1, ok2));
                    assert!(
                        res == exp_res,
                        "Expected {:?} for {:?}.equals({:?}) found {:?}",
                        exp_res,
                        var1,
                        var2,
                        res
                    );
                }
            }
        }
    }

    #[test]
    fn test_set_value() {
        let domains = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 5, 7, 8, 9],
            vec![1, 2, 3, 5, 6, 9],
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 5, 7, 9],
            vec![1],
        ];
        let expected = vec![
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::BoundChange),
            Ok(VariableState::NoChange),
        ];
        let names = vec![
            "consectuive sorted values",
            "middle isolated value",
            "last isolated",
            "first isolated",
            "only isolated values",
            "singleton domain",
        ];
        let tests = domains
            .into_iter()
            .zip(expected.into_iter())
            .zip(names.into_iter())
            .map(|((domain, expected), name)| (domain, expected, name));
        for (domain, expected, name) in tests {
            let domain_clone = domain.clone();
            let var = IntVar::new_from_iterator(domain.into_iter()).unwrap();
            for value in domain_clone.into_iter() {
                let mut var = var.clone();
                let res = var.set_value(value);
                assert!(
                    res == expected,
                    "Expected {:?} for {:?} with value {:?} found {:?}.",
                    expected,
                    name,
                    value,
                    res
                );
                let expected_var =
                    IntVar::new_from_iterator(vec![value].into_iter()).unwrap();
                assert!(
                    var == expected_var,
                    "Expected {:?} for {:?} with value {:?} found {:?}.",
                    expected_var,
                    name,
                    value,
                    var
                );
            }
        }
    }

    #[test]
    fn test_set_value_error() {
        let domains = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 5, 7, 8, 9],
            vec![1, 2, 3, 5, 6, 9],
            vec![1, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 5, 7, 9],
            vec![1],
        ];
        let values = vec![
            vec![0, 10],
            vec![0, 4, 6, 10],
            vec![0, 4, 7, 8, 10],
            vec![0, 2, 10],
            vec![0, 2, 3, 4, 6, 8, 10],
            vec![0, 2],
        ];
        let names = vec![
            "consectuive sorted values",
            "middle isolated value",
            "last isolated",
            "first isolated",
            "only isolated values",
            "signleton domain",
        ];
        let tests = domains
            .into_iter()
            .zip(values.into_iter())
            .zip(names.into_iter())
            .map(|((domain, values), name)| (domain, values, name));
        for (domain, values, name) in tests {
            let var = IntVar::new_from_iterator(domain.into_iter()).unwrap();
            for value in values.into_iter() {
                let mut var = var.clone();
                let res = var.set_value(value);
                assert!(
                    res == Err(VariableError::DomainWipeout),
                    "Expected Error for {:?} with value {:?} found {:?}.",
                    name,
                    value,
                    res
                )
            }
        }
    }

    #[test]
    fn test_in_values() {
        unimplemented!()
    }

    #[test]
    fn test_in_sorted_values() {
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
*/

pub mod bounds_int_var;
pub mod values_int_var;
