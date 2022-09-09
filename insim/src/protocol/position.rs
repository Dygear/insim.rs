//! Utility functions for working with positions.

use deku::prelude::*;
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "uom")]
use crate::units;

#[cfg(feature = "uom")]
use uom;

pub trait PointKindTrait:
    Copy
    + Into<f64>
    + for<'a> deku::DekuRead<'a, deku::ctx::Endian>
    + deku::DekuWrite<deku::ctx::Endian>
{
}

impl PointKindTrait for i32 {}
impl PointKindTrait for f32 {}
impl PointKindTrait for u16 {}

/// A X, Y, Z position
#[derive(Debug, Eq, PartialEq, DekuRead, DekuWrite, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[deku(
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Little",
    endian = "endian"
)]
pub struct Point<T>
where
    T: PointKindTrait,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point<T>
where
    T: PointKindTrait,
{
    #[cfg(feature = "uom")]
    pub fn to_uom(
        &self,
    ) -> (
        uom::si::f64::Length,
        uom::si::f64::Length,
        uom::si::f64::Length,
    ) {
        (
            uom::si::f64::Length::new::<units::length::game>(self.x.into()),
            uom::si::f64::Length::new::<units::length::game>(self.y.into()),
            uom::si::f64::Length::new::<units::length::game>(self.z.into()),
        )
    }
}