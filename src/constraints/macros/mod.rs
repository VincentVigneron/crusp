#[macro_export]
macro_rules! constraints {
    () => {};
    (handler = $handler: ident;) => {};
    (handler = $handler: ident; constraint increasing($x:ident); $($tail:tt)*) => {
        {
            $handler.add(Box::new($crate::constraints::Increasing::new($x)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident < $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::LessThan::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident <= $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::LessOrEqualThan::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident > $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::GreaterThan::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident >= $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::GreaterOrEqualThan::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident == $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::Equal::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
    (handler = $handler: ident; constraint $x:ident |==| $y: ident; $($tail:tt)*) => {
        {
            $handler.add(Box::new(
                    $crate::constraints::arithmetic::EqualBounds::new($x, $y)));
            constraints!(handler = $handler; $($tail)*);
        }
    };
}
