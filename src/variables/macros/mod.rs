// TODO acces to array view
#[macro_export]
macro_rules! variables {
    () => {};
    (handler = $handler: ident;) => {};
    (
        handler = $handler: ident;
        $x: ident = var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
        let $x = $handler.add(SetIntVar::new($min, $max).unwrap());
            variables!(handler = $handler; $($tail)*);
    };
    (
        handler = $handler: ident;
        $x: ident = array[$len: tt] of var int($min:tt .. $max:tt);
        $($tail:tt)*
    ) => {
            let $x = Array::new(10, SetIntVar::new($min, $max).unwrap()).unwrap();
            let $x = $handler.add($x);
            variables!(handler = $handler; $($tail)*);
    };
}

#[macro_export]
macro_rules! array_get_mut {
    ($array: ident[$idx: expr]) => {
        &mut *($array.get_mut($idx) as *mut _)
    }
}
