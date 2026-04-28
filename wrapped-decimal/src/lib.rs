use bytemuck::{Pod, Zeroable};
use rust_decimal::Decimal;

#[derive(Clone, Copy, Zeroable, Pod)]
#[cfg_attr(
    feature = "anchor",
    derive(
        anchor_lang::InitSpace,
        anchor_lang::prelude::AnchorSerialize,
        anchor_lang::prelude::AnchorDeserialize
    )
)]
#[repr(C)]
pub struct WrappedDecimal([u8; 16]);

impl From<Decimal> for WrappedDecimal {
    fn from(d: Decimal) -> Self {
        Self(d.serialize())
    }
}

impl From<WrappedDecimal> for Decimal {
    fn from(w: WrappedDecimal) -> Self {
        Decimal::deserialize(w.0)
    }
}
