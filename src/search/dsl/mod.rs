// More generic rules
// Replacing variables =..; constraints =..; by space =..;
// Allow rules with only cosntraints and variables.
// D'on't finalize without solve...
#[macro_export]
macro_rules! cp_model {
    (
        model {
            $($tail:tt)*
        }
        branch [
            $($var: ident),+
        ];
        solve;
        output (
            $($out: ident),+
        );
    ) => {{
        use $crate::constraints::handlers::*;
        use $crate::branchers::*;
        use $crate::branchers::brancher::*;
        use $crate::branchers::values_selector::*;
        use $crate::branchers::variables_selector::*;
        use $crate::constraints::*;
        use $crate::constraints::handlers::*;
        use $crate::search::*;
        use $crate::search::*;
        use $crate::spaces::*;
        use $crate::spaces::*;
        use $crate::variables::*;
        use $crate::variables::handlers::*;
        use $crate::variables::int_var::*;
        use $crate::variables::int_var::values_int_var::*;

        let mut variables_handler = default_handler::Builder::new();
        let mut constraints_handler = SequentialConstraintsHandler::new();
        let mut branchers_handler = BranchersHandler::new();

        cp_model!(variables = variables_handler; constraints = constraints_handler; $($tail)*);

        let variables_selector = SequentialVariableSelector::new(
            vec![$($var.clone()),+].into_iter(),
        ).unwrap();
        let values_selector = MinValueSelector::new();
        let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
        branchers_handler.add_specific_brancher(Box::new(brancher));


        let variables_handler = variables_handler.finalize();

        let space = Space::new(variables_handler, constraints_handler, branchers_handler);
        let mut solver = Solver::new(space);
        if solver.solve() {
            let solution = solver.solution().unwrap();
            Some(($(
                solution.get_variable(&$out).value().unwrap()
            ),+))
        } else {
            None
        }
    }};
    () => {};
    (variables = $variables: ident; constraints = $constraints: ident;) => {};
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
        let $x = $variables.add(SetIntVar::new($min, $max).unwrap());
        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = array[$len: tt] of var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
        let $x = Array::new(10, SetIntVar::new($min, $max).unwrap()).unwrap();
        let $x = $variables.add($x);

        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = $array: ident[$idx: expr];
        $($tail:tt)*
    ) => {
        let $x = $array.get($idx);

        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint increasing($x:ident); $($tail:tt)*) => {
        {
            $constraints.add(Box::new($crate::constraints::increasing::new(&$x)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident < $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::less_than::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident <= $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::less_or_equal_than::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident > $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::greater_than::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident >= $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::greater_or_equal_than::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident == $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::equal::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident |==| $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::equal_on_bounds::new(&$x, &$y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
         constraint $res:ident = $coefs:ident * $vars: ident;
         $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::sum::new(&$res, &$vars, $coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    //(
        //variables = $variables: ident; constraints = $constraints: ident;
         //constraint $r:ident = $a:ident * $x:ident + $b: ident * $y: ident $(+ $c: ident * $z : ident)+;
         //$($tail:tt)*) => {
        //{
            //let mut coeffs
            //$constraints.add(Box::new(
                    //$crate::constraints::arithmetic::equal_on_bounds::new(&$x, &$y)));

            //cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        //}
    //};
}
