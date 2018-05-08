#[macro_export]
macro_rules! variables {
    () => {};
    (handler = $handler: ident;) => {};
    (
        handler = $handler: ident;
        let $x: ident = var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
        let $x = $handler.add(IntVarValues::new($min, $max).unwrap());
            variables!(handler = $handler; $($tail)*);
    };
    (
        handler = $handler: ident;
        let $x: ident = array[$len: tt] of var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
            let $x = ArrayOfVars::new(10, IntVarValues::new($min, $max).unwrap()).unwrap();
            let $x = $handler.add_array($x);
            variables!(handler = $handler; $($tail)*);
    };
    (
        handler = $handler: ident;
        let $x: ident = $array: ident[$idx: expr];
        $($tail:tt)*
    ) => {
            let $x = $array.get($idx);
            variables!(handler = $handler; $($tail)*);
    };
}

#[macro_export]
macro_rules! array_get_mut {
    ($array: ident[$idx: expr]) => {
        &mut *($array.get_unchecked_mut($idx) as *mut _)
    }
}
