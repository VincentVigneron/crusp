#[macro_export]
macro_rules! value {
    ($x:ident) => {{
        #[allow(unused_imports)]
        use $crate::variables::Variable;
        $x.value().unwrap()
    }};
}
