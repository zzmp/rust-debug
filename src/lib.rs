#![deny(missing_docs, warnings)]

//! `println!()` in debug builds, noop in release.

#[macro_export]
/// `println!()` in debug builds, noop in release.
macro_rules! debugln {
    () => { debugln!("(DEBUG)") };
    ($fmt:expr) => {
        if cfg!(ndebug) {
        } else {
            println!($fmt);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        if cfg!(ndebug) {
        } else {
            println!($fmt, $($arg)*);
        }
    };
}

