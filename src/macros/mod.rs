#[macro_export]
macro_rules! unwrap_or_break {
    ($val: expr) => {
        {
            let opt = $val;
            if opt.is_none() {
                break;
            }
            opt.unwrap()
        }
    };
}

#[macro_export]
macro_rules! unwrap_first {
    ($vec: ident) => {
        *$vec.first().unwrap()
    };
}

#[macro_export]
macro_rules! unwrap_last {
    ($vec: ident) => {
        *$vec.last().unwrap()
    };
}

#[allow(unused_macros)]
macro_rules! expr {
    ($e: expr) => {
        $e
    }
}
