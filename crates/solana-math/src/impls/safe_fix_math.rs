use crate::error::SafeMathError;

/// Checked addition and subtraction for upstream `fix` values with the same scale.
#[cfg(feature = "fix")]
pub trait SafeFixAddSub: Sized {
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError>;
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError>;
}

#[cfg(feature = "fix")]
impl<Bits, Base, Exp> SafeFixAddSub for fix::Fix<Bits, Base, Exp>
where
    Bits: num_traits::CheckedAdd + num_traits::CheckedSub,
{
    #[track_caller]
    #[inline(always)]
    fn safe_add(self, rhs: Self) -> Result<Self, SafeMathError> {
        match num_traits::CheckedAdd::checked_add(&self.bits, &rhs.bits) {
            Some(result) => Ok(Self::new(result)),
            None => {
                crate::debug_log::log("Math overflow");
                Err(SafeMathError::Overflow)
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    fn safe_sub(self, rhs: Self) -> Result<Self, SafeMathError> {
        match num_traits::CheckedSub::checked_sub(&self.bits, &rhs.bits) {
            Some(result) => Ok(Self::new(result)),
            None => {
                crate::debug_log::log("Math underflow");
                Err(SafeMathError::Underflow)
            }
        }
    }
}

/// Checked multiplication and division for upstream `fix` values.
///
/// The output types retain `fix`'s type-level exponent arithmetic.
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
    Bits: num_traits::CheckedMul + num_traits::CheckedDiv,
    LExp: core::ops::Add<RExp> + core::ops::Sub<RExp>,
{
    type MulOutput = fix::Fix<Bits, Base, fix::typenum::operator_aliases::Sum<LExp, RExp>>;
    type DivOutput = fix::Fix<Bits, Base, fix::typenum::operator_aliases::Diff<LExp, RExp>>;

    #[track_caller]
    #[inline(always)]
    fn safe_mul(self, rhs: fix::Fix<Bits, Base, RExp>) -> Result<Self::MulOutput, SafeMathError> {
        match num_traits::CheckedMul::checked_mul(&self.bits, &rhs.bits) {
            Some(result) => Ok(Self::MulOutput::new(result)),
            None => {
                crate::debug_log::log("Math overflow");
                Err(SafeMathError::Overflow)
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    fn safe_div(self, rhs: fix::Fix<Bits, Base, RExp>) -> Result<Self::DivOutput, SafeMathError> {
        match num_traits::CheckedDiv::checked_div(&self.bits, &rhs.bits) {
            Some(result) => Ok(Self::DivOutput::new(result)),
            None => {
                crate::debug_log::log("Division error");
                Err(SafeMathError::Overflow)
            }
        }
    }
}

#[cfg(all(test, feature = "fix"))]
mod tests {
    use fix::aliases::decimal::{IFix128, UFix128};
    use fix::aliases::si::Pico;

    use super::{SafeFixAddSub, SafeFixMulDiv};
    use crate::SafeMathError;

    #[test]
    fn fixed_point_addition_and_subtraction_preserve_scale() {
        let value = Pico::new(40_u128);
        let two = Pico::new(2_u128);

        assert_eq!(value.safe_add(two), Ok(Pico::new(42)));
        assert_eq!(value.safe_sub(two), Ok(Pico::new(38)));
    }

    #[test]
    fn fixed_point_multiplication_and_division_preserve_type_level_scale() {
        assert_eq!(
            Pico::new(40_u128)
                .safe_mul(Pico::new(2_u128))
                .and_then(|product| product.safe_div(Pico::new(2_u128))),
            Ok(Pico::new(40))
        );
    }

    #[test]
    fn fixed_point_operations_return_boundary_errors() {
        assert_eq!(
            Pico::new(u128::MAX).safe_add(Pico::new(1)),
            Err(SafeMathError::Overflow)
        );
        assert_eq!(
            Pico::new(0_u128).safe_sub(Pico::new(1)),
            Err(SafeMathError::Underflow)
        );
        assert_eq!(
            Pico::new(u128::MAX).safe_mul(Pico::new(2)),
            Err(SafeMathError::Overflow)
        );
        assert_eq!(
            Pico::new(1_u128).safe_div(Pico::new(0)),
            Err(SafeMathError::Overflow)
        );
    }

    #[test]
    fn fixed_point_i128_feature_exposes_128_bit_aliases() {
        let unsigned: UFix128<fix::typenum::N12> = UFix128::new(40);
        assert_eq!(unsigned.safe_add(UFix128::new(2)), Ok(UFix128::new(42)));

        let signed: IFix128<fix::typenum::N12> = IFix128::new(-40);
        assert_eq!(signed.safe_add(IFix128::new(2)), Ok(IFix128::new(-38)));
    }
}
