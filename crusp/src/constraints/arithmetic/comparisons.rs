use constraints::{Constraint, PropagationState};
use variables::domains::OrderedDomain;
use variables::handlers::{
    VariableContainerHandler, VariableContainerView, VariablesHandler,
};
use variables::{Variable, VariableError, VariableId, VariableState};

macro_rules! compare_constraint_impl {
    ($name:ident; $method:ident) => {
        #[derive(Debug, Clone)]
        pub struct $name<Var, View>
        where
            View: VariableContainerView,
            //View::Container: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            lhs: View,
            rhs: View,
            output: Option<Vec<(VariableId, VariableState)>>,
        }

        impl<Var, View> $name<Var, View>
        where
            View: VariableContainerView,
            //View::Container: OrderedDomain<Type = Var>,
            Var: Eq + Ord + Clone + 'static,
        {
            pub fn new(lhs: View, rhs: View) -> $name<Var, View> {
                $name {
                    lhs: lhs,
                    rhs: rhs,
                    output: None,
                }
            }
        }

        impl<Var, Handler> Constraint<Handler> for $name<Var, Handler::View>
        where
            Handler: VariablesHandler + VariableContainerHandler<Var> + Clone,
            //View: VariableContainerView + 'static,
            Var: OrderedDomain<Type = Var>,
            //Var::Type: Eq + Ord + Clone + 'static,
        {
            fn box_clone(&self) -> Box<Constraint<Handler>> {
                let ref_self: &$name<Var, Handler::View> = &self;
                let cloned: $name<Var, Handler::View> =
                    <$name<Var, Handler::View> as Clone>::clone(ref_self);

                Box::new(cloned) as Box<Constraint<Handler>>
            }
            fn propagate(
                &mut self,
                variables_handler: &mut Handler,
            ) -> Result<PropagationState, VariableError> {
                let mut output = vec![];
                self.output = None;
                unsafe {
                    let lhs: &mut Var =
                        unsafe_from_raw_point!(variables_handler.get_mut(&self.lhs));
                    let rhs: &mut Var =
                        unsafe_from_raw_point!(variables_handler.get_mut(&self.rhs));
                    let (lhs_state, rhs_state) = lhs.$method(rhs)?;
                    match lhs_state {
                        VariableState::NoChange => {}
                        state => {
                            output.push((lhs.id(), state));
                        }
                    }
                    match rhs_state {
                        VariableState::NoChange => {}
                        state => {
                            output.push((rhs.id(), state));
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
            fn prepare(&mut self, states: Box<Iterator<Item = VariableId>>) {
                // Do nothing
            }
            fn result(&mut self) -> Box<Iterator<Item = (VariableId, VariableState)>> {
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
                variables_handler: &Handler,
            ) -> Box<Iterator<Item = (VariableId, VariableState)>> {
                Box::new(
                    vec![
                        (
                            variables_handler.get(&self.lhs).id(),
                            VariableState::ValuesChange,
                        ),
                        (
                            variables_handler.get(&self.rhs).id(),
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
    };
}

compare_constraint_impl!(LessThan; less_than);
compare_constraint_impl!(LessOrEqualThan; less_or_equal_than);
compare_constraint_impl!(GreaterThan; greater_than);
compare_constraint_impl!(GreaterOrEqualThan; greater_or_equal_than);
