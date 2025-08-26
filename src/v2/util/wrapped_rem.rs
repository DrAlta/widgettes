use super::Const;
use std::cmp::Ordering;
use std::ops::{Add, Rem, Sub};

/// Returns lhs modulo rhs, always in the range [0, rhs).
/// Handles negative lhs by wrapping into the positive cycle.
/// Works for any signed integer type.
pub fn wrapped_rem<T>(lhs: T, rhs: T) -> T
where
    T: Copy + Ord + Add<Output = T> + Rem<Output = T> + Sub<Output = T> + Const,
{
    match lhs.cmp(&T::ZERO) {
        Ordering::Less => rhs + ((lhs + T::ONE) % rhs) - T::ONE,
        Ordering::Equal => T::ZERO,
        Ordering::Greater => lhs % rhs,
    }
}
