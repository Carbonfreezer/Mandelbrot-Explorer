//! Here are some macros that deal with the changing calculation precision along the mandelbrot system.

// precision.rs
use std::sync::atomic::AtomicU32;

/// Contains the current calculation precision in bits as an atomic to be efficiently modifiable.
pub static DYNAMIC_PRECISION: AtomicU32 = AtomicU32::new(32);

/// Gets the current floating point calculation in bits.
#[macro_export]
macro_rules! precision {
    () => {
        $crate::precision::DYNAMIC_PRECISION.load(std::sync::atomic::Ordering::Relaxed)
    };
}

/// Sets the current floating point calculation in bits.
#[macro_export]
macro_rules! float {
    ($val:expr) => {
        rug::Float::with_val($crate::precision!(), $val)
    };
}
