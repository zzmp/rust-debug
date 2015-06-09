#![deny(missing_docs, warnings)]

//! `println!()` in debug builds, noop in release.

#[macro_export]
/// `println!()` in debug builds, noop in release.
macro_rules! debugln {
    () => { debugln!("(DEBUG)") };
    ($fmt:expr) => {
        if cfg!(debug_assertions) {
            println!($fmt);
        } else {
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($fmt, $($arg)*);
        } else {
        }
    };
}

