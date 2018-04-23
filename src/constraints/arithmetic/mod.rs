use super::Propagator;
use variables::int_var::BoundsIntVar;

#[derive(Debug, Clone)]
struct ArithmeticComparatorPropagator {}
impl Propagator for ArithmeticComparatorPropagator {}
impl ArithmeticComparatorPropagator {
    pub fn new() -> ArithmeticComparatorPropagator {
        ArithmeticComparatorPropagator {}
    }

    pub fn less_than<VarType: BoundsIntVar>(&self, lhs: &mut VarType, rhs: &mut VarType) {
        let _ = lhs.less_than(rhs);
    }

    pub fn greater_or_equal_than<VarType: BoundsIntVar>(
        &self,
        lhs: &mut VarType,
        rhs: &mut VarType,
    ) {
        let _ = rhs.greater_or_equal_than(lhs);
    }
}

pub mod less_than {
    use constraints;
    use constraints::ConstraintState;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::sync::Arc;
    use variables::{Variable, VariableView};
    use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                              VariablesHandler};
    use variables::int_var::*;
    struct StructVars<'a, VarType: 'a + Variable> {
        x: &'a mut VarType,
        y: &'a mut VarType,
    }
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    struct StructViews<x: VariableView, y: VariableView> {
        x: x,
        y: y,
    }
    #[allow(non_camel_case_types)]
    impl<x: VariableView, y: VariableView> StructViews<x, y> {
        #[allow(non_camel_case_types)]
        pub fn retrieve_variables<'a, VarType: 'a + Variable, H>(
            &self,
            variables_handler: &'a mut H,
        ) -> StructVars<'a, VarType>
        where
            H: VariablesHandler
                + SpecificVariablesHandler<VarType, x>
                + SpecificVariablesHandler<VarType, y>,
        {
            unsafe {
                StructVars {
                    x: get_mut_from_handler(&mut *(variables_handler as *mut _), &self.x),
                    y: get_mut_from_handler(&mut *(variables_handler as *mut _), &self.y),
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct Constraint<x: VariableView, y: VariableView, VarType: BoundsIntVar> {
        variables: StructViews<x, y>,
        propagator: super::ArithmeticComparatorPropagator,
        VarType: PhantomData<VarType>,
    }
    #[allow(non_camel_case_types)]
    impl<
        x: 'static + Clone + VariableView,
        y: 'static + Clone + VariableView,
        VarType: 'static + BoundsIntVar,
        H: 'static
            + Clone
            + VariablesHandler
            + SpecificVariablesHandler<VarType, x>
            + SpecificVariablesHandler<VarType, y>,
    > constraints::Constraint<H> for Constraint<x, y, VarType> {
        fn propagate(&mut self, variables_handler: &mut H) {
            let variables = self.variables.retrieve_variables(variables_handler);
            let _ = self.propagator
                .less_than::<VarType>(variables.x, variables.y);
        }
        //fn try_propagate(
        //&mut self,
        //_variables_handler: Arc<RefCell<H>>,
        //) -> ConstraintState {
        //unreachable!()
        //let variables = self.variables.retrieve_variables(variables_handler);
        //let _ = self.propagator.less_than(variables.x, variables.y);
        //}
        fn box_clone(&self) -> Box<constraints::Constraint<H>> {
            let ref_self: &Constraint<x, y, VarType> = &self;
            let cloned: Constraint<x, y, VarType> =
                <Constraint<x, y, VarType> as Clone>::clone(ref_self);
            Box::new(cloned) as Box<constraints::Constraint<H>>
        }
    }
    #[allow(non_camel_case_types)]
    impl<x: VariableView, y: VariableView, VarType: BoundsIntVar> Constraint<x, y, VarType> {
        #[allow(non_camel_case_types)]
        pub fn new(x: &x, y: &y) -> Constraint<x, y, VarType> {
            let mut ids = vec![x.get_id(), y.get_id()];
            ids.sort();
            let ids = ids;
            let first = *ids.first().unwrap();
            let valid = ids.iter()
                .skip(1)
                .scan(first, |state, &x| {
                    let equals = *state == x;
                    *state = x;
                    Some(equals)
                })
                .all(|x| !x);
            if !valid {
                panic!("All views must refer to different variables.");
            }
            Constraint {
                propagator: <super::ArithmeticComparatorPropagator>::new(),
                variables: StructViews {
                    x: x.clone(),
                    y: y.clone(),
                },
                VarType: PhantomData,
            }
        }
    }
    #[allow(non_camel_case_types)]
    pub fn new<x: VariableView, y: VariableView, VarType: BoundsIntVar>(
        x: &x,
        y: &y,
    ) -> Constraint<x, y, VarType> {
        Constraint::new(x, y)
    }
}

//pub mod less_than {
//use variables::int_var::BoundsIntVar;

//constraint_build!(
//struct Propagator = super::ArithmeticComparatorPropagator;
//fn new();
//fn less_than<VarType>(x: VarType, y: VarType) -> ()
//where VarType: BoundsIntVar;
//);

//}

//pub mod greater_or_equal_than {
//use variables::int_var::BoundsIntVar;

//constraint_build!(
//struct Propagator = super::ArithmeticComparatorPropagator;
//fn new();
//fn greater_or_equal_than<VarType>(x: VarType, y: VarType) -> ()
//where VarType: BoundsIntVar;
//);
//}
