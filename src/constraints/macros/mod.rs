// TODO return (nuplet views, constraint)
// TODO many new
// TODO derive clone

//fn $fnpropagate: ident($( $var: ident: $tvar: ty),+) -> $state: ty;

//macro_rules! as_expr { ($e:expr) => {$e} }
//macro_rules! as_item { ($i:item) => {$i} }
//macro_rules! as_pat  { ($p:pat) =>  {$p} }
//macro_rules! as_stmt { ($s:stmt) => {$s} }
//macro_rules! as_ident { ($i:ident) => {$i} }

#[macro_export]
macro_rules! constraint_build {
    (
        @Vars struct<$($var_type: ident : $var_bound: ty),+> {
            $( $var: ident: $tvar: ty),+
        }
    ) => {
        struct StructVars<'a, $($var_type: 'a + Variable),+> {
            $($var: &'a mut $tvar),+
        }
    };
    (
        @Retrieve
        struct<$($var_type: ident),+> {
            $( $var: ident: $tvar: ident),+
        }
        where $($var_type_bound: ident : $var_bound: ty),+;
    ) => {
        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        impl<$($var: VariableView),+> StructViews<$($var),+> {
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub fn retrieve_variables<'a, $($var_type_bound: 'a + Variable),+, H>(
                &self,
                variables_handler: &'a mut H,
                ) -> StructVars<'a, $($var_type),+>
                where H: VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+,
                      $($var_type: 'a+ Variable),+
                {
                        unsafe {
                            StructVars {
                                $(
                                    $var: get_mut_from_handler(&mut *(variables_handler as *mut _), &self.$var)
                                 ),+
                            }
                        }
                }
        }
    };
    (
        $(#[$outer:meta])*
        struct Propagator = $propagator: ty;
        fn $fnnew: ident($( $param: ident: $tparam: ty),*);
        fn $fnpropagate: ident($( $var: ident: $tvar: ty),+) -> $state: ty
        where  $($var_type_bound: ident: $var_bound: ident),+;
    ) => {
        //use variables::int_var::BoundsIntVar;
        use std::marker::PhantomData;
        use $crate::variables::{VariableView,Variable};
        use $crate::variables::handlers::{VariablesHandler,SpecificVariablesHandler,get_mut_from_handler};
        //use $crate::constraints::{ConstraintState};
        use $crate::constraints;
        //use std::cell::RefCell;
        //use std::sync::Arc;



        //struct StructVars<'a, $($var_type: 'a + $var_bound),+> {
        //struct StructVars<'a, $($var_type: 'a + Variable),+> {
        //struct StructVars<'a, $($var_type: 'a + Variable),+> {
            //$($var: &'a mut $tvar),+
        //}
        constraint_build!(@Vars
            struct<$($var_type_bound : $var_bound),+> {
                $($var: $tvar),+
            });

        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        struct StructViews<$($var: VariableView),+> {
            $($var: $var),+
        }
        //constraint_build!(@Retrieve
            //struct<$($var_type),+> {
                //$($var: $tvar),+
            //}
            //where $($var_type_bound: $var_bound),+;
        //);
        #[allow(non_snake_case)]
        #[allow(non_camel_case_types)]
        impl<$($var: VariableView),+> StructViews<$($var),+> {
        #[allow(non_snake_case)]
            #[allow(non_camel_case_types)]
            pub fn retrieve_variables<'a, $($var_type_bound: 'a + Variable),+, H>(
                &self,
                variables_handler: &'a mut H,
                ) -> StructVars<'a, $($var_type_bound),+>
                where H: VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+,
                {
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
        #[allow(non_snake_case)]
        #[derive(Clone)]
        pub struct Constraint<$($var: VariableView),+,$($var_type_bound: $var_bound),+> {
            variables: StructViews<$($var),+>,
            propagator: $propagator,
            $($var_type_bound: PhantomData<$var_type_bound>),+
            //state: $state,
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        impl<$($var: 'static + Clone + VariableView),+,
        $($var_type_bound: 'static + $var_bound),+,
        H: 'static + Clone + VariablesHandler $(+SpecificVariablesHandler<$tvar, $var>)+
            > constraints::Constraint<H>
            for Constraint<$($var),+,$($var_type_bound),+>
            {
                fn propagate(&mut self, variables_handler: &mut H) {
                    let variables = self.variables.retrieve_variables(variables_handler);
                    let _ = self.propagator.$fnpropagate::<$($var_type_bound),+>($(variables.$var),+);
                }

                //fn try_propagate(&mut self, _variables: Arc<RefCell<H>>) -> ConstraintState {
                    //unimplemented!()
                //}

                fn box_clone(&self) -> Box<constraints::Constraint<H>> {
                    let ref_self: &Constraint<$($var),+, $($var_type_bound),+> = &self;
                    let cloned: Constraint<$($var),+, $($var_type_bound),+> =
                        <Constraint<$($var),+,$($var_type_bound),+> as Clone>::clone(ref_self);

                    Box::new(cloned) as Box<constraints::Constraint<H>>
                }
            }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        impl<$($var: VariableView),+, $($var_type_bound: $var_bound),+> Constraint<$($var),+, $($var_type_bound),+> {

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            pub fn $fnnew($($var: &$var),+,$($param: $tparam),*) -> Constraint<$($var),+, $($var_type_bound),+> {
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
                        $($var: $var.clone()),+,
                    },
                    $($var_type_bound: PhantomData),+
                    //state: Default
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[allow(non_snake_case)]
        pub fn $fnnew<$($var: VariableView),+,$($var_type_bound: $var_bound),+>(
            $($var: &$var),+,$($param: $tparam),*) -> Constraint<$($var),+,$($var_type_bound),+> {
            Constraint::$fnnew($($var),+,$($param),*)
        }
    };
}

#[macro_export]
macro_rules! constraints {
    () => {};
    (handler = $handler: ident;) => {};
    //(handler = $handler: ident; constraint increasing($x:ident); $($tail:tt)*) => {
        //{
            //$handler.add(Box::new($crate::constraints::increasing::new(&$x)));
            //constraints!(handler = $handler; $($tail)*);
        //}
    //};
    (handler = $handler: ident; constraint $x:ident < $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new($crate::constraints::arithmetic::less_than::new(&$x, &$y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
}
