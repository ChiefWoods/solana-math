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

#[cfg(feature = "u64")]
convert_impl!(u64);
#[cfg(feature = "u128")]
convert_impl!(u128);
#[cfg(feature = "u256")]
convert_impl!(ruint::aliases::U256);
#[cfg(feature = "u512")]
convert_impl!(ruint::aliases::U512);
#[cfg(feature = "i64")]
convert_impl!(i64);
#[cfg(feature = "i128")]
convert_impl!(i128);

#[cfg(all(test, feature = "u256"))]
mod u256_tests {
    use ruint::aliases::U256;

    use super::SafeConvert;
    use crate::SafeMathError;

    #[test]
    fn u256_safe_convert_returns_checked_results() {
        assert_eq!(U256::from(u16::MAX).safe_to_u16(), Ok(u16::MAX));
        assert_eq!(U256::from(u64::MAX).safe_to_u64(), Ok(u64::MAX));
    }

    #[test]
    fn u256_safe_convert_returns_overflow_on_truncation() {
        assert_eq!(
            (U256::from(u16::MAX) + U256::from(1u8)).safe_to_u16(),
            Err(SafeMathError::Overflow)
        );
        assert_eq!(
            (U256::from(u64::MAX) + U256::from(1u8)).safe_to_u64(),
            Err(SafeMathError::Overflow)
        );
    }
}

#[cfg(all(test, feature = "u512"))]
mod u512_tests {
    use ruint::aliases::U512;

    use super::SafeConvert;
    use crate::SafeMathError;

    #[test]
    fn u512_safe_convert_returns_overflow_on_truncation() {
        assert_eq!(
            (U512::from(u64::MAX) + U512::from(1u8)).safe_to_u64(),
            Err(SafeMathError::Overflow)
        );
    }
}
