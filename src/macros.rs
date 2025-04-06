#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! debug_dbg {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        {
            dbg!($val);
        }
    };
}
