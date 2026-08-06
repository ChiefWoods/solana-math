use crate::{SafeMath, error::SafeMathError};

pub trait SafeMathAssign<Rhs = Self>: Sized {
    fn safe_add_assign(&mut self, rhs: Rhs) -> Result<(), SafeMathError>;
    fn safe_sub_assign(&mut self, rhs: Rhs) -> Result<(), SafeMathError>;
    fn safe_mul_assign(&mut self, rhs: Rhs) -> Result<(), SafeMathError>;
    fn safe_div_assign(&mut self, rhs: Rhs) -> Result<(), SafeMathError>;
}

macro_rules! math_assign_impl {
    ($t:ty) => {
        math_assign_impl!($t, $t);
    };
    ($t:ty, $rhs:ty) => {
        impl SafeMathAssign<$rhs> for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add_assign(&mut self, rhs: $rhs) -> Result<(), SafeMathError> {
                *self = self.safe_add(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub_assign(&mut self, rhs: $rhs) -> Result<(), SafeMathError> {
                *self = self.safe_sub(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_assign(&mut self, rhs: $rhs) -> Result<(), SafeMathError> {
                *self = self.safe_mul(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_assign(&mut self, rhs: $rhs) -> Result<(), SafeMathError> {
                *self = self.safe_div(rhs)?;
                Ok(())
            }
        }
    };
}

#[cfg(feature = "quasar")]
macro_rules! zeropod_math_assign_impl {
    ($pod:ty, $native:ty) => {
        math_assign_impl!($pod);
        math_assign_impl!($pod, $native);
    };
}

#[cfg(feature = "u8")]
math_assign_impl!(u8);
#[cfg(feature = "u16")]
math_assign_impl!(u16);
#[cfg(feature = "u32")]
math_assign_impl!(u32);
#[cfg(feature = "u64")]
math_assign_impl!(u64);
#[cfg(feature = "u128")]
math_assign_impl!(u128);
#[cfg(feature = "i8")]
math_assign_impl!(i8);
#[cfg(feature = "i16")]
math_assign_impl!(i16);
#[cfg(feature = "i32")]
math_assign_impl!(i32);
#[cfg(feature = "i64")]
math_assign_impl!(i64);
#[cfg(feature = "i128")]
math_assign_impl!(i128);
#[cfg(feature = "decimal")]
math_assign_impl!(rust_decimal::Decimal);
#[cfg(feature = "basis-points")]
math_assign_impl!(basis_points::BasisPoints);

/// `brine-fp` numerics are `Clone` but not `Copy`, so assign must clone before
/// consuming `self` in `SafeMath`.
#[cfg(feature = "brine-fp")]
macro_rules! brine_math_assign_impl {
    ($t:ty) => {
        impl SafeMathAssign for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.clone().safe_add(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.clone().safe_sub(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.clone().safe_mul(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.clone().safe_div(rhs)?;
                Ok(())
            }
        }
    };
}

#[cfg(feature = "brine-fp")]
brine_math_assign_impl!(brine_fp::UnsignedNumeric);
#[cfg(feature = "brine-fp")]
brine_math_assign_impl!(brine_fp::SignedNumeric);

#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodU16, u16);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodU32, u32);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodU64, u64);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodU128, u128);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodI16, i16);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodI32, i32);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodI64, i64);
#[cfg(feature = "quasar")]
zeropod_math_assign_impl!(zeropod::pod::PodI128, i128);

#[cfg(all(test, feature = "brine-fp"))]
mod brine_fp_tests {
    use brine_fp::UnsignedNumeric;

    use super::SafeMathAssign;
    use crate::SafeMathError;

    #[test]
    fn unsigned_numeric_safe_math_assign_updates_only_on_success() {
        let mut value = UnsignedNumeric::new(40);
        assert_eq!(value.safe_add_assign(UnsignedNumeric::new(2)), Ok(()));
        assert_eq!(value, UnsignedNumeric::new(42));

        let mut small = UnsignedNumeric::new(1);
        assert_eq!(
            small.safe_sub_assign(UnsignedNumeric::new(2)),
            Err(SafeMathError::Underflow)
        );
        assert_eq!(small, UnsignedNumeric::new(1));
    }
}

#[cfg(all(test, feature = "quasar"))]
mod zeropod_tests {
    use zeropod::pod::PodU64;

    use super::SafeMathAssign;
    use crate::SafeMathError;

    #[test]
    fn pod_numeric_safe_math_assign_updates_only_on_success() {
        let mut value = PodU64::from(40);
        assert_eq!(value.safe_add_assign(2_u64), Ok(()));
        assert_eq!(value, PodU64::from(42));

        let mut max = PodU64::MAX;
        assert_eq!(max.safe_add_assign(1_u64), Err(SafeMathError::Overflow));
        assert_eq!(max, PodU64::MAX);
    }
}
