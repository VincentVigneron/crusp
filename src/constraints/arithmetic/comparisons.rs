use constraints::{Constraint, PropagationState};
use variables::{VariableError, VariableState, VariableView, ViewIndex};
use variables::domains::OrderedDomain;
use variables::handlers::{SpecificVariablesHandler, VariablesHandler};

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
            output: Option<Vec<(ViewIndex, VariableState)>>,
        }

        impl<Var, View> $name<Var, View>
        where
            View: VariableView + Into<ViewIndex> + 'static,
            View::Variable: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            pub fn new(lhs: View, rhs: View) -> $name<Var, View> {
                $name { lhs: lhs, rhs: rhs, output: None}
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
                let mut output = vec![];
                self.output = None;
                unsafe {
                    let lhs: &mut View::Variable =
                        unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
                    let rhs: &mut View::Variable =
                        unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));
                    let (lhs, rhs) = lhs.$method(rhs)?;
                    match lhs {
                        VariableState::NoChange => {}
                        state => {
                            output.push((variables_handler.get_variable_id(&self.lhs), state));
                        }
                    }
                    match rhs {
                        VariableState::NoChange => {}
                        state => {
                            output.push((variables_handler.get_variable_id(&self.rhs), state));
                        }
                    }
                }
                //if lhs.max() < rhs.min() {
                //Ok(PropagationState::Subsumed)
                //} else {
                //Ok(PropagationState::FixPoint)
                //}
                if !output.is_empty() {
                    self.output = Some(output);
                    Ok(PropagationState::FixPoint)
                } else {
                    Ok(PropagationState::NoChange)
                }
            }
            #[allow(unused)]
            fn prepare(&mut self, states: Box<Iterator<Item = ViewIndex>>) {
                // Do nothing
            }
            fn result(&mut self) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                use std::mem;
                let mut res = None;
                mem::swap(&mut self.output, &mut res);
                match res {
                    None => Box::new(vec![].into_iter()),
                    Some(changes) => Box::new(changes.into_iter()),
                }
            }
            fn dependencies(
                &self,
                variables: &Handler,
            ) -> Box<Iterator<Item = (ViewIndex, VariableState)>> {
                Box::new(
                    vec![
                        (
                            variables.get_variable_id(&self.lhs),
                            VariableState::ValuesChange,
                        ),
                        (
                            variables.get_variable_id(&self.rhs),
                            VariableState::ValuesChange,
                        ),
                    ].into_iter(),
                )
            }
            #[allow(unused)]
            fn initialise(
                &mut self,
                variables_handler: &mut Handler,
            ) -> Result<PropagationState, VariableError> {
                self.propagate(variables_handler)
            }
        }
    }
}

compare_constraint_impl!(LessThan; less_than);
compare_constraint_impl!(LessOrEqualThan; less_or_equal_than);
compare_constraint_impl!(GreaterThan; greater_than);
compare_constraint_impl!(GreaterOrEqualThan; greater_or_equal_than);
