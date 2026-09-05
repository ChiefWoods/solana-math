use crate::error::SafeMathError;

pub trait SafePow: Sized {
    fn safe_pow(self, exp: u32) -> Result<Self, SafeMathError>;
}

#[cfg(any(
    feature = "u8",
    feature = "u16",
    feature = "u32",
    feature = "u64",
    feature = "u128",
    feature = "i8",
    feature = "i16",
    feature = "i32",
    feature = "i64",
    feature = "i128",
))]
macro_rules! pow_impl {
    ($t:ty) => {
        impl SafePow for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_pow(self, exp: u32) -> Result<Self, SafeMathError> {
                match self.checked_pow(exp) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Pow overflow");
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

/// Convert `exp` because `U256`/`U512::checked_pow` takes `Self`, not `u32`.
#[cfg(any(feature = "u256", feature = "u512"))]
macro_rules! ruint_pow_impl {
    ($t:ty) => {
        impl SafePow for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_pow(self, exp: u32) -> Result<Self, SafeMathError> {
                match self.checked_pow(<$t>::from(exp)) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Pow overflow");
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "u256")]
ruint_pow_impl!(ruint::aliases::U256);
#[cfg(feature = "u512")]
ruint_pow_impl!(ruint::aliases::U512);

#[cfg(all(test, feature = "u256"))]
mod u256_tests {
    use ruint::aliases::U256;

    use super::SafePow;
    use crate::SafeMathError;

    #[test]
    fn u256_safe_pow_returns_checked_results() {
        assert_eq!(U256::from(2u8).safe_pow(8), Ok(U256::from(256u16)));
        assert_eq!(U256::from(2u8).safe_pow(256), Err(SafeMathError::Overflow));
    }
}

#[cfg(all(test, feature = "u512"))]
mod u512_tests {
    use ruint::aliases::U512;

    use super::SafePow;
    use crate::SafeMathError;

    #[test]
    fn u512_safe_pow_returns_checked_results() {
        assert_eq!(U512::from(2u8).safe_pow(8), Ok(U512::from(256u16)));
        assert_eq!(U512::from(2u8).safe_pow(512), Err(SafeMathError::Overflow));
    }
}
