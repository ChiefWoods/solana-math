use crate::error::SafeMathError;

pub trait SafeMath<Rhs = Self>: Sized {
    fn safe_add(self, rhs: Rhs) -> Result<Self, SafeMathError>;
    fn safe_sub(self, rhs: Rhs) -> Result<Self, SafeMathError>;
    fn safe_mul(self, rhs: Rhs) -> Result<Self, SafeMathError>;
    fn safe_div(self, rhs: Rhs) -> Result<Self, SafeMathError>;
}

macro_rules! math_impl {
    ($t:ty) => {
        math_impl!($t, $t);
    };
    ($t:ty, $rhs:ty) => {
        impl SafeMath<$rhs> for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add(self, rhs: $rhs) -> Result<$t, SafeMathError> {
                match self.checked_add(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math overflow");
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub(self, rhs: $rhs) -> Result<$t, SafeMathError> {
                match self.checked_sub(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math underflow");
                        Err(SafeMathError::Underflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul(self, rhs: $rhs) -> Result<$t, SafeMathError> {
                match self.checked_mul(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math overflow");
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div(self, rhs: $rhs) -> Result<$t, SafeMathError> {
                match self.checked_div(rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Division error");
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "quasar")]
macro_rules! zeropod_math_impl {
    ($pod:ty, $native:ty) => {
        math_impl!($pod);
        math_impl!($pod, $native);
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
#[cfg(feature = "basis-points")]
math_impl!(basis_points::BasisPoints);

/// `brine-fp` checked ops take `&Self`, unlike integer/`Decimal` `checked_*`.
#[cfg(feature = "brine-fp")]
macro_rules! brine_math_impl {
    ($t:ty) => {
        impl SafeMath for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_add(&rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math overflow");
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_sub(&rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math underflow");
                        Err(SafeMathError::Underflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_mul(&rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Math overflow");
                        Err(SafeMathError::Overflow)
                    }
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div(self, rhs: $t) -> Result<$t, SafeMathError> {
                match self.checked_div(&rhs) {
                    Some(result) => Ok(result),
                    None => {
                        crate::debug_log::log("Division error");
                        Err(SafeMathError::Overflow)
                    }
                }
            }
        }
    };
}

#[cfg(feature = "brine-fp")]
brine_math_impl!(brine_fp::UnsignedNumeric);
#[cfg(feature = "brine-fp")]
brine_math_impl!(brine_fp::SignedNumeric);

#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodU16, u16);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodU32, u32);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodU64, u64);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodU128, u128);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodI16, i16);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodI32, i32);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodI64, i64);
#[cfg(feature = "quasar")]
zeropod_math_impl!(zeropod::pod::PodI128, i128);

#[cfg(all(test, feature = "brine-fp"))]
mod brine_fp_tests {
    use brine_fp::{SignedNumeric, UnsignedNumeric};

    use super::SafeMath;
    use crate::SafeMathError;

    #[test]
    fn unsigned_numeric_safe_math_returns_checked_results() {
        let a = UnsignedNumeric::new(40);
        let b = UnsignedNumeric::new(2);
        assert_eq!(a.clone().safe_add(b.clone()), Ok(UnsignedNumeric::new(42)));
        assert_eq!(a.clone().safe_mul(b.clone()), Ok(UnsignedNumeric::new(80)));
        assert_eq!(a.clone().safe_div(b.clone()), Ok(UnsignedNumeric::new(20)));
        assert_eq!(a.safe_sub(b), Ok(UnsignedNumeric::new(38)));
    }

    #[test]
    fn signed_numeric_safe_math_returns_checked_results() {
        let a = SignedNumeric::new(40);
        let b = SignedNumeric::new(2);
        assert_eq!(a.clone().safe_add(b.clone()), Ok(SignedNumeric::new(42)));
        assert_eq!(a.clone().safe_mul(b.clone()), Ok(SignedNumeric::new(80)));
        assert_eq!(a.clone().safe_div(b.clone()), Ok(SignedNumeric::new(20)));
        assert_eq!(a.safe_sub(b), Ok(SignedNumeric::new(38)));
    }

    #[test]
    fn unsigned_numeric_safe_math_returns_boundary_errors() {
        assert_eq!(
            UnsignedNumeric::new(1).safe_sub(UnsignedNumeric::new(2)),
            Err(SafeMathError::Underflow)
        );
        assert_eq!(
            UnsignedNumeric::new(1).safe_div(UnsignedNumeric::zero()),
            Err(SafeMathError::Overflow)
        );
    }
}

#[cfg(all(test, feature = "quasar"))]
mod zeropod_tests {
    use zeropod::pod::{PodI16, PodI32, PodI64, PodI128, PodU16, PodU32, PodU64, PodU128};

    use super::SafeMath;
    use crate::SafeMathError;

    #[test]
    fn pod_numeric_safe_math_returns_checked_results() {
        macro_rules! assert_safe_math {
            ($pod:ty, $native:ty) => {
                assert_eq!(
                    <$pod>::from(40 as $native).safe_add(2 as $native),
                    Ok(<$pod>::from(42 as $native))
                );
            };
        }

        assert_safe_math!(PodU16, u16);
        assert_safe_math!(PodU32, u32);
        assert_safe_math!(PodU64, u64);
        assert_safe_math!(PodU128, u128);
        assert_safe_math!(PodI16, i16);
        assert_safe_math!(PodI32, i32);
        assert_safe_math!(PodI64, i64);
        assert_safe_math!(PodI128, i128);
    }

    #[test]
    fn pod_numeric_safe_math_returns_boundary_errors() {
        assert_eq!(PodU64::MAX.safe_add(1_u64), Err(SafeMathError::Overflow));
        assert_eq!(PodU64::ZERO.safe_sub(1_u64), Err(SafeMathError::Underflow));
        assert_eq!(
            PodU64::from(1).safe_div(0_u64),
            Err(SafeMathError::Overflow)
        );
    }
}
