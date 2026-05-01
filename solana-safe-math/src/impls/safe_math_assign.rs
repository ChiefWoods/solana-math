use crate::{SafeMath, error::SafeMathError};

pub trait SafeMathAssign: Sized {
    fn safe_add_assign(&mut self, rhs: Self) -> Result<(), SafeMathError>;
    fn safe_sub_assign(&mut self, rhs: Self) -> Result<(), SafeMathError>;
    fn safe_mul_assign(&mut self, rhs: Self) -> Result<(), SafeMathError>;
    fn safe_div_assign(&mut self, rhs: Self) -> Result<(), SafeMathError>;
}

macro_rules! math_assign_impl {
    ($t:ty) => {
        impl SafeMathAssign for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.safe_add(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.safe_sub(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.safe_mul(rhs)?;
                Ok(())
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_assign(&mut self, rhs: $t) -> Result<(), SafeMathError> {
                *self = self.safe_div(rhs)?;
                Ok(())
            }
        }
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
