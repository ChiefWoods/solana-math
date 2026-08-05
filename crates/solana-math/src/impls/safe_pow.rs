use std::panic::Location;

use crate::error::SafeMathError;

pub trait SafePow: Sized {
    fn safe_pow(self, exp: u32) -> Result<Self, SafeMathError>;
}

macro_rules! pow_impl {
    ($t:ty) => {
        impl SafePow for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_pow(self, exp: u32) -> Result<Self, SafeMathError> {
                match self.checked_pow(exp) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        println!("Pow overflow at {}:{}", caller.file(), caller.line());
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "u8")]
pow_impl!(u8);
#[cfg(feature = "u16")]
pow_impl!(u16);
#[cfg(feature = "u32")]
pow_impl!(u32);
#[cfg(feature = "u64")]
pow_impl!(u64);
#[cfg(feature = "u128")]
pow_impl!(u128);
#[cfg(feature = "i8")]
pow_impl!(i8);
#[cfg(feature = "i16")]
pow_impl!(i16);
#[cfg(feature = "i32")]
pow_impl!(i32);
#[cfg(feature = "i64")]
pow_impl!(i64);
#[cfg(feature = "i128")]
pow_impl!(i128);
