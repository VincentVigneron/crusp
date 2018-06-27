// More generic rules
// Replacing variables =..; constraints =..; by space =..;
// Allow rules with only cosntraints and variables.
// Don't call finalize without calling solve befor...

// Care declaring variable inside macro can be dangerous (conflict with parameters)
#[macro_export]
macro_rules! cp_model {
    (
        model {
            $($tail:tt)*
        }
        branchers {
            $($branches: tt)*
        }
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
        use $crate::search::parallel::*;
        #[allow(unused_imports)]
        use $crate::search::path_recomputing::*;
        #[allow(unused_imports)]
        use $crate::spaces::*;
        #[allow(unused_imports)]
        use $crate::variables::*;
        #[allow(unused_imports)]
        use $crate::variables::handlers::*;
        #[allow(unused_imports)]
        use $crate::variables::domains::*;
        #[allow(unused_imports)]
        use $crate::variables::int_var::*;

        let mut variables_handler = default_handler::Builder::new();
        let mut constraints_handler = DefaultConstraintsHandlerBuilder::new();
        let mut branchers_handler = BranchersHandler::new();

        cp_model!(variables = variables_handler; constraints = constraints_handler; $($tail)*);

        cp_model!(variables = variables_handler; branchers = branchers_handler; $($branches)*);


        let mut variables_handler = variables_handler.finalize();
        let constraints_handler = constraints_handler.finalize(&mut variables_handler).unwrap();

        let space = Space::new(variables_handler, constraints_handler, branchers_handler);
        let mut solver = Solver::new(space);
        if solver.solve() {
            let solution = solver.solution().unwrap();
            Some(($(
                        solution.get_variable(&$out).clone()
                   ),+,))
        } else {
            None
        }
    }};
    (
        model {
            $($tail:tt)*
        }
        branchers {
            $($branches: tt)*
        }
        par_solve;
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
        use $crate::search::parallel::*;
        #[allow(unused_imports)]
        use $crate::search::path_recomputing::*;
        #[allow(unused_imports)]
        use $crate::spaces::*;
        #[allow(unused_imports)]
        use $crate::variables::*;
        #[allow(unused_imports)]
        use $crate::variables::handlers::*;
        #[allow(unused_imports)]
        use $crate::variables::domains::*;
        #[allow(unused_imports)]
        use $crate::variables::int_var::*;

        let mut variables_handler = default_handler::Builder::new();
        let mut constraints_handler = DefaultConstraintsHandlerBuilder::new();
        let mut branchers_handler = BranchersHandler::new();

        cp_model!(variables = variables_handler; constraints = constraints_handler; $($tail)*);

        cp_model!(variables = variables_handler; branchers = branchers_handler; $($branches)*);


        let mut variables_handler = variables_handler.finalize();
        let constraints_handler = constraints_handler.finalize(&mut variables_handler).unwrap();

        let space = Space::new(variables_handler, constraints_handler, branchers_handler);
        let mut solver = ParallelSolver::new(space);
        if solver.solve() {
            let solution = solver.solution().unwrap();
            Some(($(
                        solution.get_variable(&$out).clone()
                   ),+,))
        } else {
            None
        }
    }};
    (
        model {
            $($tail:tt)*
        }
        branchers {
            $($branches: tt)*
        }
        no_copy_solve;
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
        use $crate::search::parallel::*;
        #[allow(unused_imports)]
        use $crate::search::path_recomputing::*;
        #[allow(unused_imports)]
        use $crate::spaces::*;
        #[allow(unused_imports)]
        use $crate::variables::*;
        #[allow(unused_imports)]
        use $crate::variables::handlers::*;
        #[allow(unused_imports)]
        use $crate::variables::domains::*;
        #[allow(unused_imports)]
        use $crate::variables::int_var::*;

        let mut variables_handler = default_handler::Builder::new();
        let mut constraints_handler = DefaultConstraintsHandlerBuilder::new();
        let mut branchers_handler = BranchersHandler::new();

        cp_model!(variables = variables_handler; constraints = constraints_handler; $($tail)*);

        cp_model!(variables = variables_handler; branchers = branchers_handler; $($branches)*);


        let mut variables_handler = variables_handler.finalize();
        let constraints_handler = constraints_handler.finalize(&mut variables_handler).unwrap();

        let space = Space::new(variables_handler, constraints_handler, branchers_handler);
        let mut solver = SolverPathRecomputing::new(space);
        if solver.solve() {
            let solution = solver.solution().unwrap();
            Some(($(
                        solution.get_variable(&$out).clone()
                   ),+,))
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
        let $x = $variables.add(IntVarValuesBuilder::new($min, ($max-1)).unwrap());
        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = var int($min:tt ..= $max:tt);
        $($tail:tt)*
    ) => {
        let $x = $variables.add(IntVarValuesBuilder::new($min, $max).unwrap());
        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = array[$len: tt] of var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
        let $x = ArrayOfVarsBuilder::new($len, IntVarValuesBuilder::new(expr!($min), expr!($max-1)).unwrap()).unwrap();
        let $x = $variables.add($x);

        cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        let $x: ident = array[$len: tt] of var int($min:tt ..= $max:tt);
        $($tail:tt)*
    ) => {
        let $x = ArrayOfVarsBuilder::new($len, IntVarValuesBuilder::new(expr!($min), expr!($max)).unwrap()).unwrap();
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
            $constraints.add(Box::new($crate::constraints::Increasing::new($x)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint increasing([$($x:tt),+]); $($tail:tt)*) => {
        {
            {
                let list = cp_model!(@List in $variables; $($x),*);
                $constraints.add(Box::new($crate::constraints::Increasing::new(list)));
            }
            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident < $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::LessThan::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident <= $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::LessOrEqualThan::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident > $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::GreaterThan::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident >= $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::GreaterOrEqualThan::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident == $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::Equal::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident |==| $y: ident; $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::arithmetic::EqualBounds::new($x, $y)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint all_different($vars: ident);
        $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::AllDifferent::new($vars)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint all_different([$($y: tt),+]); $($tail:tt)*) => {
        {
            {
            let list = cp_model!(@List in $variables; $($y),+);
            $constraints.add(Box::new(
                    $crate::constraints::AllDifferent::new(list)));
            }

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        constraints = $constraints: ident;
        for $i: ident in $min:tt .. $max:tt {
            $($cons:tt)*
        }
        $($tail:tt)*
    ) => {
                for $i in $min .. $max {
                    cp_model!(
                        variables =  $variables;
                        constraints = $constraints;
                        $($cons)*
                    );
                }

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    };
    (
        variables = $variables: ident;
        constraints = $constraints: ident;
        constraint   add($res:tt ,$var:tt,$coef:expr);
        $($tail:tt)*
    ) => {{
            {
            $constraints.add(Box::new(
                $crate::constraints::arithmetic::AddConstant::new(
                    cp_model!(@Var; $res),
                    cp_model!(@Var; $var),
                    $coef
                )));
            }

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
    }};
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint $res:ident :: $coefs:ident * $vars: ident;
        $($tail:tt)*) => {
        {
            $constraints.add(Box::new(
                    $crate::constraints::SumConstraint::new($res, $vars, $coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (variables = $variables: ident; constraints = $constraints: ident; constraint $x:ident = sum([$($y: ident),+]*[$($a:expr),+]); $($tail:tt)*) => {
        {
            let coefs = vec![$($a),*];
            let vars = vec![$($y.clone()),*];
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    $crate::constraints::SumConstraint::new($x, vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (@Var; $x:ident) => {
        $x
    };
    (@Var; $x:ident[$i:expr]) => {
        $x.get($i)
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint $r:tt = ($a:tt * $x:tt + $($rem: tt)*);
        $($tail:tt)*) => {
        {
            let mut coefs = vec![expr!($a)];
            let mut vars = vec![cp_model!(@Var;$x).clone()];
            cp_model!(coefs = coefs; vars = vars; ($($rem)*));
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    $crate::constraints::SumConstraint::new($r, vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (
        variables = $variables: ident; constraints = $constraints: ident;
        constraint $r:tt =  ($x:tt + $($rem: tt)*);
        $($tail:tt)*
    ) => {
        {
            let mut coefs = vec![1];
            let mut vars = vec![cp_model!(@Var;$x).clone()];
            cp_model!(coefs = coefs; vars = vars; ($($rem)*));
            let vars = $variables.add(vars);
            $constraints.add(Box::new(
                    //$crate::constraints::SumConstraint::new($r, vars, coefs)));
                    $crate::constraints::SumConstraint::new(cp_model!(@Var;$r), vars, coefs)));

            cp_model!(variables = $variables; constraints = $constraints; $($tail)*);
        }
    };
    (coefs = $coefs: ident; vars = $vars: ident;) => {};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($x:tt)
    ) => {{
        $coefs.push(1);
        $vars.push(cp_model!(@Var;$x).clone());
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($a:tt * $x:tt)
    ) => {{
        $coefs.push(expr!($a));
        $vars.push(cp_model!(@Var; $x).clone());
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($x:tt + $($rem:tt)*)
    ) => {{
        $coefs.push(1);
        $vars.push(cp_model!(@Var; $x).clone());
        cp_model!(coefs = $coefs; vars = $vars; ($($rem)+));
    }};
    (
        coefs = $coefs: ident; vars = $vars: ident;
        ($a:tt * $x:tt + $($rem:tt)*)
    ) => {{
        $coefs.push(expr!($a));
        $vars.push(cp_model!(@Var; $x).clone());

        cp_model!(coefs = $coefs; vars = $vars; ($($rem)+));
    }};
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
    ) => {};
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], variables_order, domain_order);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SequentialVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = DomainOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$views: ident[$i:ident] for $ii:ident in $min:tt .. $max:tt], variables_order, domain_order);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            for $i in $min..$max {
                x.push($views.get($ii));
            }
            let variables_selector = SequentialVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = DomainOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], variables_order, domain_min);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SequentialVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = MinOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], variables_order, domain_max);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SequentialVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = MaxValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], smallest_domain, domain_order);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SmallestDomainVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = DomainOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$views: ident[$i:ident] for $ii:ident in $min:tt .. $max:tt], smallest_domain, domain_order);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            for $i in $min..$max {
                x.push($views.get($ii));
            }
            let variables_selector = SmallestDomainVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = DomainOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], smallest_domain, domain_min);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SmallestDomainVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = MinOrderValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (
        variables = $variables: ident;
        branchers = $branchers: ident;
        branch([$($views: tt),+], smallest_domain, domain_max);
        $($tail:tt)*
    ) => {
        {
            let mut x = vec![];
            cp_model!(@VecBuilder = x; $($views),+);
            let variables_selector = SmallestDomainVariableSelector::new(x.into_iter()).unwrap();
            let values_selector = MaxValueSelector::new();
            let brancher = DefaultBrancher::new(variables_selector, values_selector).unwrap();
            $branchers.add_specific_brancher(Box::new(brancher));

            cp_model!(variables = $variables; branchers = $branchers; $($tail)*);
        }
    };
    (@VecBuilder = $list: ident; ) => {};
    (@VecBuilder = $list: ident; $x: ident) => {
        $list.push($x.clone());
    };
    (@VecBuilder = $list: ident; $x: ident [$i: expr]) => {
        $list.push($x.get(i));
    };
    (@VecBuilder = $list: ident; $x: ident, $($views: tt),*) => {
        $list.push($x.clone());
        cp_model!(@ListBuilder = $list; $($views),*);
    };
    (@VecBuilder = $list: ident; $x: ident [$i: expr], $($views: tt),*) => {
        $list.push($x.get(i));
        cp_model!(@ListBuilder = $list; $($views),*);
    };
}
