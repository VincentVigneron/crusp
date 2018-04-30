use variables::List;
use variables::int_var::BoundsIntVar;

constraint_build!(
    struct Propagator = propagator::SumPropagator;
    fn new(coefs: Vec<i32>);
    fn propagate(res: VarType, vars: Array)
        where
            VarType: BoundsIntVar<Type=i32>,
            Array: List<VarType>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::List;
    use variables::int_var::BoundsIntVar;
    #[derive(Debug, Clone)]

    // !!!!! COEFS > 0
    pub struct SumPropagator {
        coefs: Vec<i32>,
    }
    impl Propagator for SumPropagator {}
    impl SumPropagator {
        pub fn new(coefs: Vec<i32>) -> SumPropagator {
            SumPropagator { coefs: coefs }
        }

        // adding to propagator/constraint information about change view
        // add iter to array and size => len
        // [HarveySchimpf02]
        pub fn propagate<VarType, Array>(
            &self,
            res: &mut VarType,
            vars: &mut Array,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: BoundsIntVar<Type = i32>,
            Array: List<VarType>,
        {
            use variables::VariableState;
            let mut change = false;
            let _contributions: Vec<_> = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * (var.max() - var.min()))
                .collect();
            let min: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.min())
                .sum();
            let max: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.max())
                .sum();
            let r = res.weak_upperbound(max)?;

            change = change || (r != VariableState::NoChange);
            let r = res.weak_lowerbound(min)?;
            change = change || (r != VariableState::NoChange);

            let f = res.max() - min;
            //if f < 0 {
            //return Err(VariableError::DomainWipeout);
            //}
            let vars = vars.iter_mut().zip(self.coefs.iter());
            for (var, coef) in vars {
                let bound = (f / coef) + var.min();
                let r = var.weak_upperbound(bound)?;
                change = change || (r != VariableState::NoChange);
            }

            if change {
                Ok(PropagationState::FixPoint)
            } else {
                Ok(PropagationState::NoChange)
            }
        }
    }
}

pub mod new_version {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::List;
    use variables::int_var::BoundsIntVar;

    /*
    #[allow(non_snake_case)]
    struct StructVars<'a, $($var_type: 'a + Variable),+> {
        $($var: &'a mut $tvar),+,
        $($var_type: PhantomData<$var_type>),+
    }

    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    struct StructViews<$($var: VariableView + Into<ViewIndex> + 'static),+> {
        $($var: $var),+
    }
    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl<$($var: VariableView + Into<ViewIndex>),+> StructViews<$($var),+> {
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub fn retrieve_variables<'a, $($var_type: 'a + Variable),+, H>(
            &self,
            variables_handler: &'a mut H,
            ) -> StructVars<'a, $($var_type),+>
            where H: VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+,
                  {
                      unsafe {
                          StructVars {
                              $(
                                  $var: get_mut_from_handler(
                                      &mut *(variables_handler as *mut _),
                                      &self.$var
                                      )
                               ),+,
                               $($var_type: PhantomData),+
                          }
                      }
                  }
    }

    */
    // !!!!! COEFS > 0
    #[derive(Debug, Clone)]
    pub struct SumPropagator {
        coefs: Vec<i32>,
    }

    impl Propagator for SumPropagator {}
    impl SumPropagator {
        pub fn new(coefs: Vec<i32>) -> SumPropagator {
            SumPropagator { coefs: coefs }
        }

        // adding to propagator/constraint information about change view
        // add iter to array and size => len
        // [HarveySchimpf02]
        pub fn propagate<VarType, Array>(
            &self,
            res: &mut VarType,
            vars: &mut Array,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: BoundsIntVar<Type = i32>,
            Array: List<VarType>,
        {
            use variables::VariableState;
            let mut change = false;
            let _contributions: Vec<_> = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * (var.max() - var.min()))
                .collect();
            let min: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.min())
                .sum();
            let max: i32 = vars.iter()
                .zip(self.coefs.iter())
                .map(|(var, coef)| coef * var.max())
                .sum();
            let r = res.weak_upperbound(max)?;

            change = change || (r != VariableState::NoChange);
            let r = res.weak_lowerbound(min)?;
            change = change || (r != VariableState::NoChange);

            let f = res.max() - min;
            //if f < 0 {
            //return Err(VariableError::DomainWipeout);
            //}
            let vars = vars.iter_mut().zip(self.coefs.iter());
            for (var, coef) in vars {
                let bound = (f / coef) + var.min();
                let r = var.weak_upperbound(bound)?;
                change = change || (r != VariableState::NoChange);
            }

            if change {
                Ok(PropagationState::FixPoint)
            } else {
                Ok(PropagationState::NoChange)
            }
        }
    }
}
