//! Here are some macros that deal with the changing calculation precision along the mandelbrot system.


// precision.rs
use std::sync::atomic::{AtomicU32};

pub static DYNAMIC_PRECISION: AtomicU32 = AtomicU32::new(32);

#[macro_export]
macro_rules! precision {
    () => {
        $crate::precision::DYNAMIC_PRECISION.load(std::sync::atomic::Ordering::Relaxed)
    };
}

#[macro_export]
macro_rules! float {
    ($val:expr) => {
        rug::Float::with_val($crate::precision!(), $val)
    };
}