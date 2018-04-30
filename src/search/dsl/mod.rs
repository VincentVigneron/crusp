// More generic rules
// Replacing variables =..; constraints =..; by space =..;
// Allow rules with only cosntraints and variables.
// Don't call finalize without calling solve befor...
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
        #[allow(unused_imports)]
        use $crate::constraints::handlers::*;
        #[allow(unused_imports)]
        use $crate::branchers::*;
        #[allow(unused_imports)]
        use $crate::branchers::brancher::*;
        #[allow(unused_imports)]
        use $crate::branchers::values_selector::*;
        #[allow(unused_imports)]
        use $crate::branchers::variables_selector::*;
        #[allow(unused_imports)]
        use $crate::constraints::*;
        #[allow(unused_imports)]
        use $crate::constraints::handlers::*;
        #[allow(unused_imports)]
        use $crate::search::*;
        #[allow(unused_imports)]
        use $crate::spaces::*;
        #[allow(unused_imports)]
        use $crate::variables::*;
        #[allow(unused_imports)]
        use $crate::variables::handlers::*;
        #[allow(unused_imports)]
        use $crate::variables::int_var::*;
        #[allow(unused_imports)]
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
        let $x = Array::new($len, SetIntVar::new($min, $max).unwrap()).unwrap();
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
    (@ListBuilder = $list: ident; ) => {};
    (@ListBuilder = $list: ident; $x: ident) => {
        $list.push($x.clone());
    };
    (@ListBuilder = $list: ident; $x: ident [$i: expr]) => {
        $list.push($x.get(i));
    };
    (@ListBuilder = $list: ident; $x: ident, $($views: tt),*) => {
        $list.push($x.clone());
        cp_model!(@ListBuilder = $list; $($views),*);
    };
    (@ListBuilder = $list: ident; $x: ident [$i: expr], $($views: tt),*) => {
        $list.push($x.get(i));
        cp_model!(@ListBuilder = $list; $($views),*);
    };
    (@List in $variables: ident; $($views: tt),*) => {{
        let mut list = Vec::new();
        cp_model!(@ListBuilder = list; $($views),+);
        let list = $variables.add(list);
        list
    }};
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = [$($views:tt),+];
        $($tail:tt)*
    ) => {
        let $x = cp_model!(@List in $variables; $($views),+);

        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint increasing($x:ident); $($tail:tt)*) => {
        {
            $constraints.add(Box::new($crate::constraints::increasing::new(&$x)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint increasing([$($x:tt),+]); $($tail:tt)*) => {
        {
            {
                let list = cp_model!(@List in $variables; $($x),*);
                $constraints.add(Box::new($crate::constraints::increasing::new(&list)));
            }
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
        constraint $res:ident :: $coefs:ident * $vars: ident;
        $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::sum::new(&$res, &$vars, $coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident = sum([$($y: ident),+]*[$($a:expr),+]); $($tail:tt)*) => {
        {
            let coefs = vec![$($a),*];
            let vars = vec![$($y.clone()),*];
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    $crate::constraints::sum::new(&$x, &vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint $r:ident = ($a:tt * $x:ident + $($rem: tt)*);
        $($tail:tt)*) => {
        {
            let mut coefs = vec![expr!($a)];
            let mut vars = vec![$x.clone()];
            cp_model!(coefs = coefs; vars = vars; ($($rem)*));
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    $crate::constraints::sum::new(&$r, &vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint $r:ident =  ($x:ident + $($rem: tt)*);
        $($tail:tt)*
    ) => {
        {
            let mut coefs = vec![1];
            let mut vars = vec![$x.clone()];
            cp_model!(coefs = coefs; vars = vars; ($($rem)*));
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    $crate::constraints::sum::new(&$r, &vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (coefs = $coefs: ident; vars = $vars: ident;) => {};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($x:ident)
    ) => {{
        $coefs.push(1);
        $vars.push($x.clone());
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($a:tt * $x:ident)
    ) => {{
        $coefs.push(expr!($a));
        $vars.push($x.clone());
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($x:ident + $($rem:tt)*)
    ) => {{
        $coefs.push(1);
        $vars.push($x.clone());
        cp_model!(coefs = $coefs; vars = $vars; ($($rem)+));
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($a:tt * $x:ident + $($rem:tt)*)
    ) => {{
        $coefs.push(expr!($a));
        $vars.push($x.clone());

        cp_model!(coefs = $coefs; vars = $vars; ($($rem)+));
    }};
}
