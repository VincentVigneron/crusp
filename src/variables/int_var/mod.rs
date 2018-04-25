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

pub trait IntVar: Variable + Eq {
    type Type: Ord + PartialOrd;
    fn size(&self) -> usize;
    fn min(&self) -> Self::Type;
    fn max(&self) -> Self::Type;
    fn value(&self) -> Option<Self::Type>;
    fn iter<'a>(&'a self) -> Box<Iterator<Item = &Self::Type> + 'a>;
}

pub trait BoundsIntVar: IntVar + Variable {
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

    fn equal_bounds(
        &mut self,
        value: &mut Self,
    ) -> Result<(VariableState, VariableState), VariableError> {
        // TODO keep record of first update
        // invalide atm
        let _ = value.less_or_equal_than(self)?;
        self.less_or_equal_than(value)
    }
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
        let mut values: Vec<_> = values.collect();
        values.sort();
        self.in_sorted_values(values.into_iter())
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
        Predicate: Fn(&Self::Type) -> bool;
    fn retains_if<Predicate>(
        &mut self,
        pred: Predicate,
    ) -> Result<VariableState, VariableError>
    where
        Predicate: Fn(&Self::Type) -> bool;
}

// More macros for generating test assert especially)
#[allow(unused_macros)]
macro_rules! assert_domain_eq{
    ($var: ident, $exp: ident, $name: ident) => {
        assert!(
        $var.iter().eq($exp.iter()),
        "Expected {:?} domain for {:?} found {:?}",
        $exp,
        $name,
        $var.iter().collect::<Vec<_>>()
        )
    }
}

#[allow(unused_macros)]
macro_rules! check_domain_and_invariants{
    ($var: ident,
     $fn: ident,
     $param: ident,
     $exp_res: ident,
     $exp_domain: ident,
     $name: ident)
    => {
        let var_clone = $var.clone();
        let res = $var.$fn($param);
        assert_eq!(
            res, $exp_res,
            "Result expected {:?} for {:?}.{}({}) found {:?}",
            $exp_res,
            var_clone, stringify!($fn),
            $param, res);
        if $exp_res.is_err() {
            continue;
        }
        assert_eq!(
            unwrap_first!($exp_domain), $var.min(),
            "Min expected {:?} for {:?}.{}({}) found {:?}",
            unwrap_first!($exp_domain),
            var_clone, stringify!($fn),
            $param, $var.min());
        assert_eq!(
            unwrap_last!($exp_domain), $var.max(),
            "Max expected {:?} for {:?}.{}({}) found {:?}",
            unwrap_last!($exp_domain),
            var_clone, stringify!($fn),
            $param, $var.max());
        assert_eq!(
            $exp_domain.len(), $var.size(),
            "Size expected {:?} for {:?}.{}({}) found {:?}",
            $exp_domain.len(),
            var_clone, stringify!($fn),
            $param, $var.size());
        assert_domain_eq!($var, $exp_domain, $name);
    }
}

#[allow(unused_macros)]
macro_rules! assert_var_eq{
    ($x: ident, $y: ident) => {
        assert!(
            $x == $y,
            "Expected {:?} equals to {:?}",
            $x,
            $y
            )
    }
}

#[allow(unused_macros)]
macro_rules! assert_result_binary_constraint {
    ($lhs: ident, $rhs: ident, $res: ident, $exp_res: ident, $constraint: expr) => {
        assert!($res == $exp_res,
            "Expected {:?} for {:?}.{}({:?}) found {:?}",
            $exp_res,
            $lhs,
            $constraint,
            $rhs,
            $res
           );
    }
}

#[allow(unused_macros)]
macro_rules! expr {
    ($e: expr) => {
        $e
    }
}

#[allow(unused_macros)]
macro_rules! bound_test {
    ($testname: ident, $var: ty, $fnbound: ident, $op: tt, $min: expr => $max: expr) => {
        #[test]
        fn $testname() {
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                //vec![1],
            ];
            let bounds: Vec<_> = ($min..($max+1)).collect();
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
                .zip(names.into_iter());
            for (domain, name) in tests {
                for bound in bounds.iter().map(|val| *val) {
                    let mut var =
                        <$var>::new_from_values(domain.clone().into_iter()).unwrap();
                    let exp_domain: Vec<_> = domain.iter()
                        .map(|val| *val)
                        .filter(|&val| expr!(val $op bound))
                        .collect();
                    let exp_res = if domain == exp_domain {
                        Ok(VariableState::NoChange)
                    } else {
                        Ok(VariableState::BoundChange)
                    };
                    check_domain_and_invariants!(
                        var,
                        $fnbound,
                        bound,
                        exp_res,
                        exp_domain,
                        name);
                }
            }
        }

    }
}

#[allow(unused_macros)]
macro_rules! bound_test_error {
    ($testname: ident, $var: ty, $fnbound: ident, $op: tt, $min: expr => $max: expr) => {
        #[test]
        fn $testname() {
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                //vec![1],
            ];
            let bounds: Vec<_> = ($min..($max+1)).collect();
            let names = vec![
                "consectuive sorted values",
                "middle isolated value",
                "last isolated",
                "first isolated",
                "only isolated values",
                "singleton domain",
            ];
            let exp_res = Err(VariableError::DomainWipeout);
            let tests = domains
                .into_iter()
                .zip(names.into_iter());
            for (domain, name) in tests {
                for bound in bounds.iter().map(|val| *val) {
                    let mut var =
                        <$var>::new_from_values(domain.clone().into_iter()).unwrap();
                    let var_clone = var.clone();
                    let res = var.$fnbound(bound);
                    assert_eq!(
                        res, exp_res,
                        "Result expected {:?} for {:?}.{}({}) found {:?}",
                        exp_res,
                        var_clone, stringify!($fnbound),
                        bound, res);
                }
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! test_int_var{
    ($var: ty) => {
        use super::*;

        // TODO test maxvalue
        #[test]
        fn test_new_from_range() {
            let vars = vec![(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)];
            let name = "range";
            for (min, max) in vars.into_iter() {
                let var = <$var>::new_from_range(min, max).unwrap();
                let domain: Vec<_> = (min..(max+1)).collect();
                assert_eq!(var.min(), min, "min false for: \"{:?}\"", var);
                assert_eq!(var.max(), max, "max false for: \"{:?}\"", var);
                assert_domain_eq!(var, domain, name);
            }
        }

        #[test]
        fn test_new_from_range_error() {
            let vars = vec![(1, 0), (100, 22), (10, 5), (15, 9), (3, 2)];
            for (min, max) in vars.into_iter() {
                let var = <$var>::new_from_range(min, max);
                match var {
                    None => {}
                    _ => unreachable!("Expected none for: \"{:?}\"", var),
                }
            }
        }

        // TODO refactoring
        #[test]
        fn test_new_from_values() {
            use rand::{thread_rng, Rng};
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                vec![1],
            ];
            let expected_domains = domains.clone();
            let names = vec![
                "consectuive sorted values",
                "middle isolated value",
                "last isolated",
                "first isolated",
                "only isolated values",
                "singleton domain",
            ];
            let mut rng = thread_rng();

            for _ in 0..100 {
                let tests = domains
                    .clone()
                    .into_iter()
                    .zip(expected_domains.clone().into_iter())
                    .zip(names.clone().into_iter())
                    .map(|((domain, expected_domain), name)|
                         (domain, expected_domain, name));
                for (mut domain, expected_domain, name) in tests {
                    rng.shuffle(&mut domain);
                    let var = <$var>::new_from_values(domain.into_iter());
                    match var {
                        Some(var) => assert_domain_eq!(var, expected_domain, name),
                        _ => {
                            unreachable!(
                                "Expected some variable for: \"{:?}\"", name)
                        }
                    }
                }
            }
        }

        #[test]
        fn test_new_from_values_error() {
            let domain: Vec<<$var as IntVar>::Type> = Vec::new();
            assert!(
                <$var>::new_from_values(domain.into_iter()).is_none(),
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
                let var = <$var>::new_from_values(domain.into_iter()).unwrap();
                assert!(
                    var.size() == exp_size,
                    "Expected size {:?} for {:?} found {:?}.",
                    exp_size,
                    var,
                    var.size()
                    );
            }
        }


        //let domains = vec![
        //vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        //vec![1, 2, 3, 5, 7, 8, 9],
        //vec![1, 2, 3, 5, 6, 9],
        //vec![1, 3, 4, 5, 6, 7, 8, 9],
        //vec![1, 5, 7, 9],
        ////vec![1],
        //];
        bound_test!(
            test_weak_upperbound,
            $var,
            weak_upperbound,
            <=, 1 => 10);
        bound_test_error!(
            test_weak_upperbound_error,
            $var,
            weak_upperbound,
            <=,
            -1 => 0);

        bound_test!(
            test_strict_upperbound,
            $var,
            strict_upperbound,
            <,
            2 => 10);
        bound_test_error!(
            test_strict_upperbound_error,
            $var,
            strict_upperbound,
            <,
            -1 => 1);

        bound_test!(
            test_weak_lowerbound,
            $var,
            weak_lowerbound,
            >=,
            0 => 9);
        bound_test_error!(
            test_weak_lowerbound_error,
            $var,
            weak_lowerbound,
            >=,
            10 => 11);

        bound_test!(
            test_strict_lowerbound,
            $var,
            strict_lowerbound,
            >,
            0 => 8);
        bound_test_error!(
            test_strict_lowerbound_error,
            $var,
            strict_lowerbound,
            >,
            9 => 11);

        #[test]
        fn test_remove_value() {
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                vec![1],
            ];
            let (min, max) = (0,11);
            let expected_domains = domains.clone();
            let names = vec![
                "consectuive sorted values",
                "middle isolated value",
                "last isolated",
                "first isolated",
                "only isolated values",
                "singleton domain",
            ];
            for (domain, name) in domains.into_iter().zip(names.into_iter()) {
                for value in min..max {
                    let mut var =
                        <$var>::new_from_values(domain.clone().into_iter()).unwrap();
                    let exp_domain: Vec<_> = domain.iter()
                        .map(|val| *val)
                        .filter(|&val| val != value)
                        .collect();
                    let exp_res = if exp_domain.is_empty() {
                        Err(VariableError::DomainWipeout)
                    } else if exp_domain == domain {
                        Ok(VariableState::NoChange)
                    } else if unwrap_first!(domain) != unwrap_first!(exp_domain) {
                        Ok(VariableState::BoundChange)
                    } else if unwrap_last!(domain) != unwrap_last!(exp_domain) {
                        Ok(VariableState::BoundChange)
                    } else {
                        Ok(VariableState::ValuesChange)
                    };
                    check_domain_and_invariants!(
                        var,
                        remove_value,
                        value,
                        exp_res,
                        exp_domain,
                        name);
                }
            }
        }

        #[test]
        fn test_less_than() {
            unreachable!();
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                vec![1],
            ];
            let (min, max) = (0,11);
            let expected_domains = domains.clone();
            let names = vec![
                "consectuive sorted values",
                "middle isolated value",
                "last isolated",
                "first isolated",
                "only isolated values",
                "singleton domain",
            ];
            for (domain, name) in domains.into_iter().zip(names.into_iter()) {
                for value in min..max {
                    let mut var =
                        <$var>::new_from_values(domain.clone().into_iter()).unwrap();
                    let exp_domain: Vec<_> = domain.iter()
                        .map(|val| *val)
                        .filter(|&val| val != value)
                        .collect();
                    let exp_res = if exp_domain.is_empty() {
                        Err(VariableError::DomainWipeout)
                    } else if exp_domain == domain {
                        Ok(VariableState::NoChange)
                    } else if unwrap_first!(domain) != unwrap_first!(exp_domain) {
                        Ok(VariableState::BoundChange)
                    } else if unwrap_last!(domain) != unwrap_last!(exp_domain) {
                        Ok(VariableState::BoundChange)
                    } else {
                        Ok(VariableState::ValuesChange)
                    };
                    check_domain_and_invariants!(
                        var,
                        remove_value,
                        value,
                        exp_res,
                        exp_domain,
                        name);
                }
            }
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
                        <$var>::new_from_values(domain1.clone().into_iter()).unwrap();
                    let mut var2 =
                        <$var>::new_from_values(domain2.clone().into_iter()).unwrap();
                    let var1_base = var1.clone();
                    let var2_base = var2.clone();
                    let res = var1.equal(&mut var2);
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
                            var1_base,
                            var2_base,
                            res
                            );
                    } else {
                        let var_res =
                            <$var>::new_from_values(dom_eq.clone().into_iter())
                            .unwrap();
                        assert_var_eq!(var1, var_res);
                        assert_var_eq!(var2, var_res);
                        let ok1 = if domain1.iter().eq(var1.iter()) {
                            VariableState::NoChange
                        } else if domain1.first() != dom_eq.first() {
                            VariableState::BoundChange
                        } else if domain1.last() != dom_eq.last() {
                            VariableState::BoundChange
                        } else {
                            VariableState::ValuesChange
                        };
                        let ok2 = if domain2.iter().eq(var2.iter()) {
                            VariableState::NoChange
                        } else if domain2.first() != dom_eq.first() {
                            VariableState::BoundChange
                        } else if domain2.last() != dom_eq.last() {
                            VariableState::BoundChange
                        } else {
                            VariableState::ValuesChange
                        };
                        let exp_res = Ok((ok1, ok2));
                        assert_result_binary_constraint!(
                            var1_base,
                            var2_base,
                            res,
                            exp_res,
                            "equals");
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
                let var = <$var>::new_from_values(domain.into_iter()).unwrap();
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
                        <$var>::new_from_values(vec![value].into_iter()).unwrap();
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
                let var = <$var>::new_from_values(domain.into_iter()).unwrap();
                for value in values.into_iter() {
                    let mut var = var.clone();
                    let res = var.set_value(value);
                    assert_eq!(
                        res, Err(VariableError::DomainWipeout),
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
        fn test_from_range_iter() {
            let vars = [(0, 1), (-1, 22), (3, 5), (5, 9), (2, 2)]
                .into_iter()
                .map(|&(min, max)| <$var>::new_from_range(min, max))
                .map(Option::unwrap)
                .collect::<Vec<_>>();
            let domains = vec![
                (0..2).collect::<Vec<_>>(),
                (-1..23).collect::<Vec<_>>(),
                (3..6).collect::<Vec<_>>(),
                (5..10).collect::<Vec<_>>(),
                (2..3).collect::<Vec<_>>(),
            ];
            for (domain, expected) in vars.into_iter().zip(domains.into_iter()) {
                let tmp_domain = domain.clone();
                let tmp_expected = expected.clone();
                let name = "dom iter";
                assert_domain_eq!(domain, expected, name);
            }
        }

        #[test]
        fn test_from_values_iter() {
            let domains = vec![
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 2, 3, 5, 7, 8, 9],
                vec![1, 2, 3, 5, 6, 9],
                vec![1, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 5, 7, 9],
                vec![1],
            ];
            let expected_domains = domains.clone();
            let domains = domains.into_iter()
                .map(|values| <$var>::new_from_values(values.into_iter()))
                .map(Option::unwrap)
                .collect::<Vec<_>>();
            let names = vec![
                "consectuive sorted values",
                "middle isolated value",
                "last isolated",
                "first isolated",
                "only isolated values",
                "singleton domain",
            ];
            let tests = domains.into_iter()
                .zip(expected_domains.into_iter())
                .zip(names.into_iter())
                .map(|((domain,expected),name)| (domain, expected, name));
            for (domain, expected, name) in tests {
                assert_domain_eq!(domain, expected, name);
            }
        }
    }
}

pub mod bounds_int_var;
pub mod values_int_var;
