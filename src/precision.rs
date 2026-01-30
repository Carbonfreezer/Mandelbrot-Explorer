//! Here are some macros that deal with the changing calculation precision along the mandelbrot system.


pub static mut DYNAMIC_PRECISION: u32 = 32;



#[macro_export]
macro_rules! precision {
    () => {
        {
            let safe_copy;
            unsafe {
                safe_copy = $crate::precision::DYNAMIC_PRECISION;
            }
            safe_copy
        }
    };
}


#[macro_export]
macro_rules! float {
    ($val:expr) => {
        rug::Float::with_val($crate::precision!(), $val)
    };
}
