use variables::Array;
use variables::int_var::BoundsIntVar;

constraint_build!(
    struct Propagator = propagator::SumPropagator;
    fn new(coefs: Vec<i32>);
    fn propagate(res: VarType, vars: ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars)
        where
            VarType: BoundsIntVar<Type=i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>;
    );

pub mod propagator {
    use constraints::{PropagationState, Propagator, VariableError};
    use variables::Array;
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
        pub fn propagate<VarType, ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars>(
            &self,
            res: &mut VarType,
            vars: &mut ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: BoundsIntVar<Type = i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>,
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

/*
pub mod new_version {
    use constraints::{PropagationState, VariableError};
    use std::marker::PhantomData;
    use variables::{Array, Variable, VariableView, ViewIndex};
    use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                              VariablesHandler};
    use variables::int_var::BoundsIntVar;

    #[allow(non_snake_case)]
    struct Variables<'a, Var: 'a + Variable> {
        x: &'a mut Var,
        Var: PhantomData<Var>,
    }

    #[derive(Debug, Clone)]
    #[allow(non_snake_case)]
    struct VariableViews<x: VariableView + Into<ViewIndex> + 'static> {
        x: x,
    }

    #[allow(non_snake_case)]
    #[allow(non_camel_case_types)]
    impl<x> VariableViews<x>
    where
        x: VariableView + Into<ViewIndex>,
    {
        pub fn new(x: x) -> Self {
            VariableViews { x: x }
        }
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        pub fn retrieve_variables<'a, Var, Handler>(
            &self,
            variables_handler: &'a mut Handler,
        ) -> Variables<'a, Var>
        where
            Var: 'a + Variable,
            Handler: VariablesHandler + SpecificVariablesHandler<Var, x>,
        {
            unsafe {
                Variables {
                    x: get_mut_from_handler(&mut *(variables_handler as *mut _), &self.x),
                    Var: PhantomData,
                }
            }
        }
    }

    // !!!!! COEFS > 0
    #[derive(Debug, Clone)]
    pub struct SumPropagator<View: VariableView + Into<ViewIndex> + 'static> {
        coefs: Vec<i32>,
        variable_views: VariableViews<View>,
    }

    //impl Propagator for SumPropagator {}
    impl<View: VariableView + Into<ViewIndex> + 'static> SumPropagator<View> {
        pub fn new(coefs: Vec<i32>, x: View) -> SumPropagator<View> {
            SumPropagator {
                coefs: coefs,
                variable_views: VariableViews::new(x),
            }
        }

        // adding to propagator/constraint information about change view
        // add iter to array and size => len
        // [HarveySchimpf02]
        pub fn propagate<VarType, ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars>(
            &self,
            res: &mut VarType,
            vars: &mut ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars,
        ) -> Result<PropagationState, VariableError>
        where
            VarType: BoundsIntVar<Type = i32>,
            ArrayOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVarsOfVars: Array<VarType>,
        {
            use variables::VariableState;
            //let mut vars = self.variable_views.retrieve_variable
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
*/
