pub use more_asserts::*;

pub const APPROX_TOLERANCE: f64 = 1e-4;

#[macro_export]
macro_rules! assert_approx {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !((*left_val as f64 - *right_val as f64).abs() < $crate::macros::APPROX_TOLERANCE) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`"#, &*left_val, &*right_val)
                }
            }
        }
    });
    ($left:expr, $right:expr,) => ({
        assert_approx!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !((*left_val as f64 - *right_val as f64).abs() < $crate::macros::APPROX_TOLERANCE) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(r#"assertion failed: `(left ≈ right)`
  left: `{:?}`,
 right: `{:?}`: {}"#, &*left_val, &*right_val, format_args!($($arg)+))
                }
            }
        }
    });
}

#[macro_export]
macro_rules! debug_assert_approx {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { $crate::assert_approx!($($arg)*); })
}

#[cfg(test)]
mod test {
    #[test]
    fn approx_eq() {
        let a = 0.1;
        let b = 0.1000001;
        debug_assert_approx!(a, b,);
        assert_approx!(a, b, "not approx_eq!");
    }

    #[test]
    #[should_panic]
    fn approx_ne() {
        assert_approx!(0.1, 0.2, "Ok!");
    }
}
