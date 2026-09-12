use crate::error::SafeMathError;

/// Checked addition and subtraction for `hylo-fix` values with the same scale.
#[cfg(feature = "fix")]
pub trait SafeFixAddSub: Sized {
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
}

#[cfg(feature = "fix")]
impl<Bits, Base, Exp> SafeFixAddSub for fix::Fix<Bits, Base, Exp>
where
    fix::Fix<Bits, Base, Exp>: fix::prelude::CheckedAdd + fix::prelude::CheckedSub,
{
    #[track_caller]
    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError> {
        match fix::prelude::CheckedAdd::checked_add(&self, &rhs) {
            Some(result) => Ok(result),
            None => {
                crate::debug_log::log("Math overflow");
                Err(SafeMathError::Overflow)
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError> {
        match fix::prelude::CheckedSub::checked_sub(&self, &rhs) {
            Some(result) => Ok(result),
            None => {
                crate::debug_log::log("Math underflow");
                Err(SafeMathError::Underflow)
            }
        }
    }
}

/// Checked multiplication and division for `hylo-fix` values.
///
/// The output types retain `hylo-fix`'s type-level exponent arithmetic.
#[cfg(feature = "fix")]
pub trait SafeFixMulDiv<Rhs = Self>: Sized {
    type MulOutput;
    type DivOutput;

    fn safe_mul(self, rhs: Rhs) -> Result<Self::MulOutput, SafeMathError>;
    fn safe_div(self, rhs: Rhs) -> Result<Self::DivOutput, SafeMathError>;
}

#[cfg(feature = "fix")]
impl<Bits, Base, LExp, RExp> SafeFixMulDiv<fix::Fix<Bits, Base, RExp>>
    for fix::Fix<Bits, Base, LExp>
where
    fix::Fix<Bits, Base, LExp>: fix::CheckedMulFix<fix::Fix<Bits, Base, RExp>>
        + fix::CheckedDivFix<fix::Fix<Bits, Base, RExp>>,
{
    type MulOutput =
        <fix::Fix<Bits, Base, LExp> as fix::CheckedMulFix<fix::Fix<Bits, Base, RExp>>>::Output;
    type DivOutput =
        <fix::Fix<Bits, Base, LExp> as fix::CheckedDivFix<fix::Fix<Bits, Base, RExp>>>::Output;

    #[track_caller]
    #[inline(always)]
    fn safe_mul(self, rhs: fix::Fix<Bits, Base, RExp>) -> Result<Self::MulOutput, SafeMathError> {
        match fix::CheckedMulFix::checked_mul(&self, &rhs) {
            Some(result) => Ok(result),
            None => {
                crate::debug_log::log("Math overflow");
                Err(SafeMathError::Overflow)
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    fn safe_div(self, rhs: fix::Fix<Bits, Base, RExp>) -> Result<Self::DivOutput, SafeMathError> {
        match fix::CheckedDivFix::checked_div(&self, &rhs) {
            Some(result) => Ok(result),
            None => {
                crate::debug_log::log("Division error");
                Err(SafeMathError::Overflow)
            }
        }
    }
}

#[cfg(all(test, feature = "fix"))]
mod tests {
    use fix::aliases::si::Pico;

    use super::{SafeFixAddSub, SafeFixMulDiv};
    use crate::SafeMathError;

    #[test]
    fn fixed_point_addition_and_subtraction_preserve_scale() {
        let value = Pico::new(40_u8);
        let two = Pico::new(2_u8);

        assert_eq!(value.safe_add(two), Ok(Pico::new(42)));
        assert_eq!(value.safe_sub(two), Ok(Pico::new(38)));
    }

    #[test]
    fn fixed_point_multiplication_and_division_preserve_type_level_scale() {
        assert_eq!(
            Pico::new(40_u8)
                .safe_mul(Pico::new(2_u8))
                .and_then(|product| product.safe_div(Pico::new(2_u8))),
            Ok(Pico::new(40))
        );
    }

    #[test]
    fn fixed_point_operations_return_boundary_errors() {
        assert_eq!(
            Pico::new(u8::MAX).safe_add(Pico::new(1)),
            Err(SafeMathError::Overflow)
        );
        assert_eq!(
            Pico::new(0_u8).safe_sub(Pico::new(1)),
            Err(SafeMathError::Underflow)
        );
        assert_eq!(
            Pico::new(u8::MAX).safe_mul(Pico::new(2)),
            Err(SafeMathError::Overflow)
        );
        assert_eq!(
            Pico::new(1_u8).safe_div(Pico::new(0)),
            Err(SafeMathError::Overflow)
        );
    }
}
