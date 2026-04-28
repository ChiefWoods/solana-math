use std::panic::Location;

use crate::error::SafeMathError;

pub trait SafeMath: Sized {
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_mul(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_div(self, rhs: Self) -> Result<Self, SafeMathError>;
}

macro_rules! math_impl {
    ($t:ty) => {
        impl SafeMath for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_add(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        println!("Math overflow at {}:{}", caller.file(), caller.line());
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_sub(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        println!("Math underflow at {}:{}", caller.file(), caller.line());
                        Err(SafeMathError::Underflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_mul(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        println!("Math overflow at {}:{}", caller.file(), caller.line());
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_div(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        println!("Division error at {}:{}", caller.file(), caller.line());
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "u8")]
math_impl!(u8);
#[cfg(feature = "u16")]
math_impl!(u16);
#[cfg(feature = "u32")]
math_impl!(u32);
#[cfg(feature = "u64")]
math_impl!(u64);
#[cfg(feature = "u128")]
math_impl!(u128);
#[cfg(feature = "i8")]
math_impl!(i8);
#[cfg(feature = "i16")]
math_impl!(i16);
#[cfg(feature = "i32")]
math_impl!(i32);
#[cfg(feature = "i64")]
math_impl!(i64);
#[cfg(feature = "i128")]
math_impl!(i128);
#[cfg(feature = "decimal")]
math_impl!(rust_decimal::Decimal);
