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
