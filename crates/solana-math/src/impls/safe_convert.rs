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
                match u16::try_from(self) {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        crate::debug_log::log("Conversion to u16 failed");
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_to_u64(self) -> Result<u64, SafeMathError> {
                match u64::try_from(self) {
                    Ok(value) => Ok(value),
                    Err(_) => {
                        crate::debug_log::log("Conversion to u64 failed");
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "u128")]
convert_impl!(u128);
#[cfg(feature = "i128")]
convert_impl!(i128);
