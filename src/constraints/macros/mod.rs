// TODO return (nuplet views, constraint)
// TODO many new
// TODO derive clone
#[macro_export]
macro_rules! constraint_build {
    (
        $(#[$outer:meta])*
        struct Propagator = $propagator: ty;
        fn $fnnew: ident($( $param: ident: $tparam: ty),*);
        fn $fnpropagate: ident($( $var: ident: $tvar: ty),+) -> $state: ty;
    ) => {
        use $crate::variables::{VariableView};
        use $crate::variables::handlers::{VariablesHandler,SpecificVariablesHandler,get_mut_from_handler};
        use $crate::constraints::{ConstraintState};
        use $crate::constraints;
        use std::cell::RefCell;
        use std::sync::Arc;


        struct StructVars<'a> {
            $($var: &'a mut $tvar),+
        }

        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        struct StructViews<$($var: VariableView),+> {
            $($var: $var),+
        }

        #[allow(non_camel_case_types)]
        impl<$($var: VariableView),+> StructViews<$($var),+> {
            #[allow(non_camel_case_types)]
            pub fn retrieve_variables<'a, H>(
                &self,
                variables_handler: &'a mut H,
                ) -> StructVars<'a>
                where H: VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+ {
                        unsafe {
                            StructVars {
                                $(
                                    $var: get_mut_from_handler(&mut *(variables_handler as *mut _), &self.$var)
                                 ),+
                            }
                        }
                }
        }


        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct Constraint<$($var: VariableView),+> {
            variables: StructViews<$($var),+>,
            propagator: $propagator,
            //state: $state,
        }

        #[allow(non_camel_case_types)]
        impl<$($var: 'static + Clone + VariableView),+,
        H: 'static + Clone + VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+
            > constraints::Constraint<H>
            for Constraint<$($var),+> {
                fn propagate(&mut self, variables_handler: &mut H) {
                    let variables = self.variables.retrieve_variables(variables_handler);
                    let _ = self.propagator.$fnpropagate($(variables.$var),+);
                }

                fn try_propagate(&mut self, _variables: Arc<RefCell<H>>) -> ConstraintState {
                    unimplemented!()
                }

                fn box_clone(&self) -> Box<constraints::Constraint<H>> {
                    let ref_self: &Constraint<$($var),+> = &self;
                    let cloned: Constraint<$($var),+> =
                        <Constraint<$($var),+> as Clone>::clone(ref_self);

                    Box::new(cloned) as Box<constraints::Constraint<H>>
                }
            }

        #[allow(non_camel_case_types)]
        impl<$($var: VariableView),+> Constraint<$($var),+> {

            #[allow(non_camel_case_types)]
            pub fn $fnnew($($var: &$var),+,$($param: $tparam),*) -> Constraint<$($var),+> {
                let mut ids = vec![$($var.get_id()),+];
                ids.sort();
                let ids = ids;
                let first = *ids.first().unwrap();
                let valid = ids.iter().skip(1)
                    .scan(first, |state, &x| {
                        let equals = *state == x;
                        *state = x;
                        Some(equals)
                    }).all(|x| !x);
                if !valid {
                    panic!("All views must refer to different variables.");
                }

                Constraint {
                    propagator: <$propagator>::$fnnew($($param),*),
                    variables: StructViews {
                        $($var: $var.clone()),+
                    },
                    //state: Default
                }
            }
        }

        #[allow(non_camel_case_types)]
        pub fn $fnnew<$($var: VariableView),+>(
            $($var: &$var),+,$($param: $tparam),*) -> Constraint<$($var),+> {
            Constraint::$fnnew($($var),+,$($param),*)
        }
    };
}

#[macro_export]
macro_rules! constraints {
    () => {};
    (handler = $handler: ident;) => {};
    (handler = $handler: ident; constraint increasing($x:ident); $($tail:tt)*) => {
        {
            $handler.add(Box::new($crate::constraints::increasing::new(&$x)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident < $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new($crate::constraints::arithmetic::less_than::new(&$x, &$y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
}
