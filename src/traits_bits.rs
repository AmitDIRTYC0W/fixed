// Copyright © 2018–2022 Trevor Spiteri

// This library is free software: you can redistribute it and/or
// modify it under the terms of either
//
//   * the Apache License, Version 2.0 or
//   * the MIT License
//
// at your option.
//
// You should have recieved copies of the Apache License and the MIT
// License along with the library. If not, see
// <https://www.apache.org/licenses/LICENSE-2.0> and
// <https://opensource.org/licenses/MIT>.

use az_crate::{
    Cast, CastFrom, CheckedCast, CheckedCastFrom, OverflowingCast, OverflowingCastFrom,
    SaturatingCast, SaturatingCastFrom, UnwrappedCast, UnwrappedCastFrom, WrappingCast,
    WrappingCastFrom,
};
use bytemuck::Pod;
use core::{
    fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex},
    hash::Hash,
    iter::{Product, Sum},
    num::ParseIntError,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::FromStr,
};

macro_rules! impl_bits {
    ($Bits:ident) => {
        impl FixedBits for $Bits {}
        impl Sealed for $Bits {}
        impl FixedBitsCast<i8> for $Bits {}
        impl FixedBitsCast<i16> for $Bits {}
        impl FixedBitsCast<i32> for $Bits {}
        impl FixedBitsCast<i64> for $Bits {}
        impl FixedBitsCast<i128> for $Bits {}
        impl FixedBitsCast<isize> for $Bits {}
        impl FixedBitsCast<u8> for $Bits {}
        impl FixedBitsCast<u16> for $Bits {}
        impl FixedBitsCast<u32> for $Bits {}
        impl FixedBitsCast<u64> for $Bits {}
        impl FixedBitsCast<u128> for $Bits {}
        impl FixedBitsCast<usize> for $Bits {}
    };
}

/// This trait is implemented for <code>[Fixed]::[Bits]</code> for all
/// fixed-point numbers.
///
/// This provides some facilities to manipulate bits in generic functions.
///
/// This trait is sealed and cannot be implemented for more types; it is
/// implemented for [`i8`], [`i16`], [`i32`], [`i64`], [`i128`], [`u8`],
/// [`u16`], [`u32`], [`u64`], and [`u128`].
///
/// # Examples
///
/// ```rust
/// # use az_crate::OverflowingAs;
/// # #[cfg(skip_this)]
/// use az::OverflowingAs;
/// use fixed::{traits::Fixed, types::*};
/// fn limited_positive_bits<F: Fixed>(fixed: F) -> Option<u32> {
///     let bits = fixed.to_bits();
///     match bits.overflowing_as::<u32>() {
///         (wrapped, false) => Some(wrapped),
///         (_, true) => None,
///     }
/// }
/// assert_eq!(limited_positive_bits(I16F16::from_bits(100)), Some(100));
/// assert_eq!(limited_positive_bits(I16F16::from_bits(-100)), None);
/// ```
///
/// [Bits]: crate::traits::Fixed::Bits
/// [Fixed]: crate::traits::Fixed
pub trait FixedBits
where
    Self: Default + Hash + Ord,
    Self: Pod,
    Self: Debug + Display + Binary + Octal + LowerHex + UpperHex,
    Self: FromStr<Err = ParseIntError>,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Mul<Output = Self> + MulAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Rem<Output = Self> + RemAssign,
    Self: Not<Output = Self>,
    Self: BitAnd<Output = Self> + BitAndAssign,
    Self: BitOr<Output = Self> + BitOrAssign,
    Self: BitXor<Output = Self> + BitXorAssign,
    Self: Shl<u32, Output = Self> + ShlAssign<u32>,
    Self: Shr<u32, Output = Self> + ShrAssign<u32>,
    Self: Sum + Product,
    Self: FixedBitsCast<i8> + FixedBitsCast<i16> + FixedBitsCast<i32>,
    Self: FixedBitsCast<i64> + FixedBitsCast<i128> + FixedBitsCast<isize>,
    Self: FixedBitsCast<u8> + FixedBitsCast<u16> + FixedBitsCast<u32>,
    Self: FixedBitsCast<u64> + FixedBitsCast<u128> + FixedBitsCast<usize>,
    Self: Sealed,
{
}

/// This trait is used to provide supertraits to the [`FixedBits`] trait, and
/// should not be used directly.
///
/// With this trait, [`FixedBits`] constraints are made cleaner.
pub trait FixedBitsCast<Prim>
where
    Self: TryInto<Prim> + TryFrom<Prim>,
    Self: Cast<Prim> + CastFrom<Prim>,
    Self: CheckedCast<Prim> + CheckedCastFrom<Prim>,
    Self: SaturatingCast<Prim> + SaturatingCastFrom<Prim>,
    Self: WrappingCast<Prim> + WrappingCastFrom<Prim>,
    Self: UnwrappedCast<Prim> + UnwrappedCastFrom<Prim>,
    Self: OverflowingCast<Prim> + OverflowingCastFrom<Prim>,
    Self: Sealed,
{
}

pub trait Sealed {}

impl_bits! { i8 }
impl_bits! { i16 }
impl_bits! { i32 }
impl_bits! { i64 }
impl_bits! { i128 }
impl_bits! { u8 }
impl_bits! { u16 }
impl_bits! { u32 }
impl_bits! { u64 }
impl_bits! { u128 }
