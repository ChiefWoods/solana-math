use std::panic::Location;

use crate::error::SafeMathError;

pub trait SafeConvert {
    fn safe_to_u16(self) -> Result<u16, SafeMathError>;
    fn safe_to_u64(self) -> Result<u64, SafeMathError>;
}

macro_rules! convert_impl {
    ($t:ty) => {
        impl SafeConvert for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_to_u16(self) -> Result<u16, SafeMathError> {
                u16::try_from(self).map_err(|_| {
                    let caller = Location::caller();
                    println!(
                        "Conversion to u16 failed at {}:{}",
                        caller.file(),
                        caller.line()
                    );
                    SafeMathError::ArithmeticOverflow
                })
            }

            #[track_caller]
            #[inline(always)]
            fn safe_to_u64(self) -> Result<u64, SafeMathError> {
                u64::try_from(self).map_err(|_| {
                    let caller = Location::caller();
                    println!(
                        "Conversion to u64 failed at {}:{}",
                        caller.file(),
                        caller.line()
                    );
                    SafeMathError::ArithmeticOverflow
                })
            }
        }
    };
}

#[cfg(feature = "u128")]
convert_impl!(u128);
#[cfg(feature = "i128")]
convert_impl!(i128);
