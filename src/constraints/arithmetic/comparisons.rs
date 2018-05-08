use constraints::{Constraint, PropagationState};
use variables::{VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{get_mut_from_handler, SpecificVariablesHandler,
                          VariablesHandler};

macro_rules! compare_constraint_impl {
    ($name: ident; $method: ident) => {
        #[derive(Debug, Clone)]
        pub struct $name<Var, View>
        where
            View: VariableView + Into<ViewIndex> + 'static,
            View::Variable: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            lhs: View,
            rhs: View,
        }

        impl<Var, View> $name<Var, View>
        where
            View: VariableView + Into<ViewIndex> + 'static,
            View::Variable: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            pub fn new(lhs: View, rhs: View) -> $name<Var, View> {
                $name { lhs: lhs, rhs: rhs }
            }
        }

        impl<Var, View, Handler> Constraint<Handler> for $name<Var, View>
        where
            Handler: VariablesHandler + SpecificVariablesHandler<View> + Clone,
            View: VariableView + Into<ViewIndex> + 'static,
            View::Variable: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            fn box_clone(&self) -> Box<Constraint<Handler>> {
                let ref_self: &$name<Var, View> = &self;
                let cloned: $name<Var, View> = <$name<Var, View> as Clone>::clone(ref_self);

                Box::new(cloned) as Box<Constraint<Handler>>
            }
            fn propagate(
                &mut self,
                variables_handler: &mut Handler,
                ) -> Result<PropagationState, VariableError> {
                let mut change = false;
                unsafe {
                    let lhs: &mut View::Variable =
                        unsafe_get_mut_from_handler!(variables_handler, self.lhs);
                    let rhs: &mut View::Variable =
                        unsafe_get_mut_from_handler!(variables_handler, self.rhs);
                    let r = lhs.$method(rhs)?;
                    change = change || (r != (VariableState::NoChange, VariableState::NoChange));
                }
                //if lhs.max() < rhs.min() {
                //Ok(PropagationState::Subsumed)
                //} else {
                //Ok(PropagationState::FixPoint)
                //}
                if change {
                    Ok(PropagationState::FixPoint)
                } else {
                    Ok(PropagationState::NoChange)
                }
            }
            #[allow(unused)]
            fn prepare(&mut self, states: Box<Iterator<Item = (ViewIndex, VariableState)>>) {
                unimplemented!()
            }
            fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                unimplemented!()
            }
            fn dependencies(&self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                unimplemented!()
            }
            #[allow(unused)]
            fn initialise(&mut self, variables_handler: &mut Handler) -> Result<(), ()> {
                unimplemented!()
            }
        }
    }
}

compare_constraint_impl!(LessThan; less_than);
compare_constraint_impl!(LessOrEqualThan; less_or_equal_than);
compare_constraint_impl!(GreaterThan; greater_than);
compare_constraint_impl!(GreaterOrEqualThan; greater_or_equal_than);
