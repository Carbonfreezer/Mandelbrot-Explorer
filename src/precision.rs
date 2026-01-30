//! Here are some macros that deal with the changing calculation precision along the mandelbrot system.

// precision.rs
use std::sync::RwLock;

pub static DYNAMIC_PRECISION: RwLock<u32> = RwLock::new(32);

#[macro_export]
macro_rules! precision {
    () => {
        *$crate::precision::DYNAMIC_PRECISION.read().unwrap()
    };
}

#[macro_export]
macro_rules! float {
    ($val:expr) => {
        rug::Float::with_val($crate::precision!(), $val)
    };
}
