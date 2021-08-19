// Copyright © 2018–2021 Trevor Spiteri

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

use crate::{
    helpers::IntHelper,
    traits::ToFixed,
    types::extra::{LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8},
    wide_div::WideDivRem,
    FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
    FixedU8,
};
use az_crate::WrappingAs;
use core::{
    iter::{Product, Sum},
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

macro_rules! refs {
    (impl $Imp:ident for $Fixed:ident$(($LeEqU:ident))* { $method:ident }) => {
        impl<Frac $(: $LeEqU)*> $Imp<$Fixed<Frac>> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                (*self).$method(rhs)
            }
        }

        impl<Frac $(: $LeEqU)*> $Imp<&$Fixed<Frac>> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Fixed<Frac>) -> $Fixed<Frac> {
                self.$method(*rhs)
            }
        }

        impl<Frac $(: $LeEqU)*> $Imp<&$Fixed<Frac>> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Fixed<Frac>) -> $Fixed<Frac> {
                (*self).$method(*rhs)
            }
        }
    };

    (impl $Imp:ident<$Inner:ty> for $Fixed:ident$(($LeEqU:ident))* { $method:ident }) => {
        impl<Frac $(: $LeEqU)*> $Imp<$Inner> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: $Inner) -> $Fixed<Frac> {
                (*self).$method(rhs)
            }
        }

        impl<Frac $(: $LeEqU)*> $Imp<&$Inner> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Inner) -> $Fixed<Frac> {
                self.$method(*rhs)
            }
        }

        impl<Frac $(: $LeEqU)*> $Imp<&$Inner> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Inner) -> $Fixed<Frac> {
                (*self).$method(*rhs)
            }
        }
    };
}

macro_rules! refs_assign {
    (impl $Imp:ident for $Fixed:ident$(($LeEqU:ident))* { $method:ident }) => {
        impl<Frac $(: $LeEqU)*> $Imp<&$Fixed<Frac>> for $Fixed<Frac> {
            #[inline]
            fn $method(&mut self, rhs: &$Fixed<Frac>) {
                self.$method(*rhs);
            }
        }
    };

    (impl $Imp:ident<$Inner:ty> for $Fixed:ident$(($LeEqU:ident))* { $method:ident }) => {
        impl<Frac $(: $LeEqU)*> $Imp<&$Inner> for $Fixed<Frac> {
            #[inline]
            fn $method(&mut self, rhs: &$Inner) {
                self.$method(*rhs);
            }
        }
    };
}

macro_rules! pass {
    (impl $Imp:ident for $Fixed:ident { $method:ident }) => {
        impl<Frac> $Imp<$Fixed<Frac>> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                Self::from_bits(self.to_bits().$method(rhs.to_bits()))
            }
        }

        refs! { impl $Imp for $Fixed { $method } }
    };
}

macro_rules! pass_assign {
    (impl $Imp:ident for $Fixed:ident { $method:ident }) => {
        impl<Frac> $Imp<$Fixed<Frac>> for $Fixed<Frac> {
            #[inline]
            fn $method(&mut self, rhs: $Fixed<Frac>) {
                self.bits.$method(rhs.to_bits())
            }
        }

        refs_assign! { impl $Imp for $Fixed { $method } }
    };
}

macro_rules! pass_one {
    (impl $Imp:ident for $Fixed:ident { $method:ident }) => {
        impl<Frac> $Imp for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self) -> $Fixed<Frac> {
                Self::from_bits(self.to_bits().$method())
            }
        }

        impl<Frac> $Imp for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self) -> $Fixed<Frac> {
                (*self).$method()
            }
        }
    };
}

macro_rules! shift {
    (impl $Imp:ident < $Rhs:ty > for $Fixed:ident { $method:ident }) => {
        impl<Frac> $Imp<$Rhs> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: $Rhs) -> $Fixed<Frac> {
                $Fixed::from_bits(self.to_bits().$method(rhs))
            }
        }

        impl<Frac> $Imp<$Rhs> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: $Rhs) -> $Fixed<Frac> {
                (*self).$method(rhs)
            }
        }

        impl<Frac> $Imp<&$Rhs> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Rhs) -> $Fixed<Frac> {
                self.$method(*rhs)
            }
        }

        impl<Frac> $Imp<&$Rhs> for &$Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn $method(self, rhs: &$Rhs) -> $Fixed<Frac> {
                (*self).$method(*rhs)
            }
        }
    };
}

macro_rules! shift_assign {
    (impl $Imp:ident < $Rhs:ty > for $Fixed:ident { $method:ident }) => {
        impl<Frac> $Imp<$Rhs> for $Fixed<Frac> {
            #[inline]
            fn $method(&mut self, rhs: $Rhs) {
                self.bits.$method(rhs)
            }
        }

        impl<Frac> $Imp<&$Rhs> for $Fixed<Frac> {
            #[inline]
            fn $method(&mut self, rhs: &$Rhs) {
                self.$method(*rhs)
            }
        }
    };
}

macro_rules! shift_all {
    (
        impl {$Imp:ident, $ImpAssign:ident}<{$($Rhs:ty),*}> for $Fixed:ident
        { $method:ident, $method_assign:ident }
    ) => { $(
        shift! { impl $Imp<$Rhs> for $Fixed { $method } }
        shift_assign! { impl $ImpAssign<$Rhs> for $Fixed { $method_assign } }
    )* };
}

macro_rules! fixed_arith {
    (
        $Fixed:ident($Inner:ty, $LeEqU:ident, $bits_count:expr $(, $NonZeroInner:ident)?),
        $Signedness:tt
    ) => {
        if_signed! {
            $Signedness;
            pass_one! { impl Neg for $Fixed { neg } }
        }

        pass! { impl Add for $Fixed { add } }
        pass_assign! { impl AddAssign for $Fixed { add_assign } }
        pass! { impl Sub for $Fixed { sub } }
        pass_assign! { impl SubAssign for $Fixed { sub_assign } }

        impl<Frac: $LeEqU> Mul<$Fixed<Frac>> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                let (ans, overflow) = self.to_bits().mul_overflow(rhs.to_bits(), Frac::U32);
                debug_assert!(!overflow, "overflow");
                Self::from_bits(ans)
            }
        }

        refs! { impl Mul for $Fixed($LeEqU) { mul } }

        impl<Frac, RhsFrac: $LeEqU> MulAssign<$Fixed<RhsFrac>> for $Fixed<Frac> {
            #[inline]
            fn mul_assign(&mut self, rhs: $Fixed<RhsFrac>) {
                let (ans, overflow) = self.to_bits().mul_overflow(rhs.to_bits(), RhsFrac::U32);
                debug_assert!(!overflow, "overflow");
                *self = Self::from_bits(ans);
            }
        }

        impl<Frac, RhsFrac: $LeEqU> MulAssign<&$Fixed<RhsFrac>> for $Fixed<Frac> {
            #[inline]
            fn mul_assign(&mut self, rhs: &$Fixed<RhsFrac>) {
                let (ans, overflow) = self.to_bits().mul_overflow(rhs.to_bits(), RhsFrac::U32);
                debug_assert!(!overflow, "overflow");
                *self = Self::from_bits(ans);
            }
        }

        impl<Frac: $LeEqU> Div<$Fixed<Frac>> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn div(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                let (ans, overflow) = self.to_bits().div_overflow(rhs.to_bits(), Frac::U32);
                debug_assert!(!overflow, "overflow");
                Self::from_bits(ans)
            }
        }

        refs! { impl Div for $Fixed($LeEqU) { div } }

        impl<Frac: $LeEqU> DivAssign<$Fixed<Frac>> for $Fixed<Frac> {
            #[inline]
            fn div_assign(&mut self, rhs: $Fixed<Frac>) {
                *self = (*self).div(rhs)
            }
        }

        refs_assign! { impl DivAssign for $Fixed($LeEqU) { div_assign } }

        // do not pass! { Rem }, as I::MIN % I::from(-1) should return 0, not panic
        impl<Frac> Rem<$Fixed<Frac>> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn rem(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                self.checked_rem(rhs).expect("division by zero")
            }
        }

        refs! { impl Rem for $Fixed { rem } }

        impl<Frac> RemAssign<$Fixed<Frac>> for $Fixed<Frac> {
            #[inline]
            fn rem_assign(&mut self, rhs: $Fixed<Frac>) {
                *self = (*self).rem(rhs)
            }
        }

        refs_assign! { impl RemAssign for $Fixed { rem_assign } }

        pass_one! { impl Not for $Fixed { not } }
        pass! { impl BitAnd for $Fixed { bitand } }
        pass_assign! { impl BitAndAssign for $Fixed { bitand_assign } }
        pass! { impl BitOr for $Fixed { bitor } }
        pass_assign! { impl BitOrAssign for $Fixed { bitor_assign } }
        pass! { impl BitXor for $Fixed { bitxor } }
        pass_assign! { impl BitXorAssign for $Fixed { bitxor_assign } }

        impl<Frac> Mul<$Inner> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: $Inner) -> $Fixed<Frac> {
                Self::from_bits(self.to_bits().mul(rhs))
            }
        }

        refs! { impl Mul<$Inner> for $Fixed($LeEqU) { mul } }

        impl<Frac: $LeEqU> MulAssign<$Inner> for $Fixed<Frac> {
            #[inline]
            fn mul_assign(&mut self, rhs: $Inner) {
                *self = (*self).mul(rhs);
            }
        }

        refs_assign! { impl MulAssign<$Inner> for $Fixed($LeEqU) { mul_assign } }

        impl<Frac: $LeEqU> Mul<$Fixed<Frac>> for $Inner {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                rhs.mul(self)
            }
        }

        impl<Frac: $LeEqU> Mul<&$Fixed<Frac>> for $Inner {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: &$Fixed<Frac>) -> $Fixed<Frac> {
                (*rhs).mul(self)
            }
        }

        impl<Frac: $LeEqU> Mul<$Fixed<Frac>> for &$Inner {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: $Fixed<Frac>) -> $Fixed<Frac> {
                rhs.mul(*self)
            }
        }

        impl<Frac: $LeEqU> Mul<&$Fixed<Frac>> for &$Inner {
            type Output = $Fixed<Frac>;
            #[inline]
            fn mul(self, rhs: &$Fixed<Frac>) -> $Fixed<Frac> {
                (*rhs).mul(*self)
            }
        }

        impl<Frac> Div<$Inner> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn div(self, rhs: $Inner) -> $Fixed<Frac> {
                Self::from_bits(self.to_bits().div(rhs))
            }
        }

        refs! { impl Div<$Inner> for $Fixed($LeEqU) { div } }

        impl<Frac: $LeEqU> DivAssign<$Inner> for $Fixed<Frac> {
            #[inline]
            fn div_assign(&mut self, rhs: $Inner) {
                *self = (*self).div(rhs);
            }
        }

        refs_assign! { impl DivAssign<$Inner> for $Fixed($LeEqU) { div_assign } }

        impl<Frac: $LeEqU> Rem<$Inner> for $Fixed<Frac> {
            type Output = $Fixed<Frac>;
            #[inline]
            fn rem(self, rhs: $Inner) -> $Fixed<Frac> {
                self.checked_rem_int(rhs).expect("division by zero")
            }
        }

        refs! { impl Rem<$Inner> for $Fixed($LeEqU) { rem } }

        impl<Frac: $LeEqU> RemAssign<$Inner> for $Fixed<Frac> {
            #[inline]
            fn rem_assign(&mut self, rhs: $Inner) {
                *self = (*self).rem(rhs);
            }
        }

        refs_assign! { impl RemAssign<$Inner> for $Fixed($LeEqU) { rem_assign } }

        shift_all! {
            impl {Shl, ShlAssign}<{
                i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
            }> for $Fixed {
                shl, shl_assign
            }
        }
        shift_all! {
            impl {Shr, ShrAssign}<{
                i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
            }> for $Fixed {
                shr, shr_assign
            }
        }

        impl<Frac> Sum<$Fixed<Frac>> for $Fixed<Frac> {
            fn sum<I>(iter: I) -> $Fixed<Frac>
            where
                I: Iterator<Item = $Fixed<Frac>>,
            {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl<'a, Frac: 'a> Sum<&'a $Fixed<Frac>> for $Fixed<Frac> {
            fn sum<I>(iter: I) -> $Fixed<Frac>
            where
                I: Iterator<Item = &'a $Fixed<Frac>>,
            {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl<Frac: $LeEqU> Product<$Fixed<Frac>> for $Fixed<Frac> {
            fn product<I>(mut iter: I) -> $Fixed<Frac>
            where
                I: Iterator<Item = $Fixed<Frac>>,
            {
                match iter.next() {
                    None => 1.to_fixed(),
                    Some(first) => iter.fold(first, Mul::mul),
                }
            }
        }

        impl<'a, Frac: 'a + $LeEqU> Product<&'a $Fixed<Frac>> for $Fixed<Frac> {
            fn product<I>(mut iter: I) -> $Fixed<Frac>
            where
                I: Iterator<Item = &'a $Fixed<Frac>>,
            {
                match iter.next() {
                    None => 1.to_fixed(),
                    Some(first) => iter.fold(*first, Mul::mul),
                }
            }
        }

        $(
            if_unsigned! {
                $Signedness;

                impl<Frac> Div<$NonZeroInner> for $Fixed<Frac> {
                    type Output = $Fixed<Frac>;
                    #[inline]
                    fn div(self, rhs: $NonZeroInner) -> $Fixed<Frac> {
                        Self::from_bits(self.to_bits() / rhs)
                    }
                }

                refs! { impl Div<$NonZeroInner> for $Fixed { div } }

                impl <Frac> DivAssign<$NonZeroInner> for $Fixed<Frac> {
                    #[inline]
                    fn div_assign(&mut self, rhs: $NonZeroInner) {
                        *self = (*self).div(rhs)
                    }
                }

                refs_assign! { impl DivAssign<$NonZeroInner> for $Fixed { div_assign } }

                impl<Frac: $LeEqU> Rem<$NonZeroInner> for $Fixed<Frac> {
                    type Output = $Fixed<Frac>;
                    #[inline]
                    fn rem(self, rhs: $NonZeroInner) -> $Fixed<Frac> {
                        // Hack to silence overflow operation error if we shift
                        // by Self::FRAC_NBITS directly.
                        let frac_nbits = Self::FRAC_NBITS;
                        if frac_nbits == <$Inner>::BITS {
                            // rhs > self, so the remainder is self
                            return self;
                        }
                        let rhs = rhs.get();
                        let rhs_fixed_bits = rhs << frac_nbits;
                        if (rhs_fixed_bits >> frac_nbits) != rhs {
                            // rhs > self, so the remainder is self
                            return self;
                        }
                        // SAFETY: rhs_fixed_bits must have some significant bits since
                        // rhs_fixed_bits >> frac_nbits is equal to a non-zero value.
                        let n = unsafe { $NonZeroInner::new_unchecked(rhs_fixed_bits) };
                        Self::from_bits(self.to_bits() % n)
                    }
                }

                refs! { impl Rem<$NonZeroInner> for $Fixed($LeEqU) { rem } }

                impl <Frac: $LeEqU> RemAssign<$NonZeroInner> for $Fixed<Frac> {
                    #[inline]
                    fn rem_assign(&mut self, rhs: $NonZeroInner) {
                        *self = (*self).rem(rhs)
                    }
                }

                refs_assign! { impl RemAssign<$NonZeroInner> for $Fixed($LeEqU) { rem_assign } }
            }
        )*
    };
}

fixed_arith! { FixedU8(u8, LeEqU8, 8, NonZeroU8), Unsigned }
fixed_arith! { FixedU16(u16, LeEqU16, 16, NonZeroU16), Unsigned }
fixed_arith! { FixedU32(u32, LeEqU32, 32, NonZeroU32), Unsigned }
fixed_arith! { FixedU64(u64, LeEqU64, 64, NonZeroU64), Unsigned }
fixed_arith! { FixedU128(u128, LeEqU128, 128, NonZeroU128), Unsigned }
fixed_arith! { FixedI8(i8, LeEqU8, 8), Signed }
fixed_arith! { FixedI16(i16, LeEqU16, 16), Signed }
fixed_arith! { FixedI32(i32, LeEqU32, 32), Signed }
fixed_arith! { FixedI64(i64, LeEqU64, 64), Signed }
fixed_arith! { FixedI128(i128, LeEqU128, 128), Signed }

pub(crate) trait MulDivOverflow: Sized {
    fn mul_overflow(self, rhs: Self, frac_nbits: u32) -> (Self, bool);
    // -NBITS <= frac_nbits <= 2 * NBITS
    fn mul_add_overflow(self, mul: Self, add: Self, frac_nbits: i32) -> (Self, bool);
    fn div_overflow(self, rhs: Self, frac_nbits: u32) -> (Self, bool);
}

macro_rules! mul_div_widen {
    ($Single:ty, $Double:ty, $Signedness:tt) => {
        impl MulDivOverflow for $Single {
            #[inline]
            fn mul_overflow(self, rhs: $Single, frac_nbits: u32) -> ($Single, bool) {
                const NBITS: u32 = <$Single>::NBITS;
                let int_nbits: u32 = NBITS - frac_nbits;
                let lhs2 = <$Double>::from(self);
                let rhs2 = <$Double>::from(rhs) << int_nbits;
                let (prod2, overflow) = lhs2.overflowing_mul(rhs2);
                ((prod2 >> NBITS) as $Single, overflow)
            }

            #[inline]
            fn mul_add_overflow(
                self,
                mul: $Single,
                add: $Single,
                mut frac_nbits: i32,
            ) -> ($Single, bool) {
                type Unsigned = <$Single as IntHelper>::Unsigned;
                const NBITS: i32 = <$Single>::NBITS as i32;
                let self2 = <$Double>::from(self);
                let mul2 = <$Double>::from(mul);
                let prod2 = self2 * mul2;
                let (prod2, overflow2) = if frac_nbits < 0 {
                    frac_nbits += NBITS;
                    debug_assert!(frac_nbits >= 0);
                    prod2.overflowing_mul(<$Double>::from(Unsigned::MAX) + 1)
                } else if frac_nbits > NBITS {
                    frac_nbits -= NBITS;
                    debug_assert!(frac_nbits <= NBITS);
                    (prod2 >> NBITS, false)
                } else {
                    (prod2, false)
                };
                let lo = (prod2 >> frac_nbits) as Unsigned;
                let hi = (prod2 >> frac_nbits >> NBITS) as $Single;
                if_signed_unsigned!(
                    $Signedness,
                    {
                        let (uns, carry) = lo.overflowing_add(add as Unsigned);
                        let ans = uns as $Single;
                        let expected_hi = if (ans.is_negative() != add.is_negative()) == carry {
                            0
                        } else {
                            -1
                        };
                        (ans, overflow2 || hi != expected_hi)
                    },
                    {
                        let (ans, overflow) = lo.overflowing_add(add);
                        (ans, overflow2 || overflow || hi != 0)
                    },
                )
            }

            #[inline]
            fn div_overflow(self, rhs: $Single, frac_nbits: u32) -> ($Single, bool) {
                const NBITS: u32 = <$Single>::NBITS;
                let lhs2 = <$Double>::from(self) << frac_nbits;
                let rhs2 = <$Double>::from(rhs);
                let quot2 = lhs2 / rhs2;
                let quot = quot2 as $Single;
                let overflow = if_signed_unsigned!(
                    $Signedness,
                    quot2 >> NBITS != if quot < 0 { -1 } else { 0 },
                    quot2 >> NBITS != 0
                );
                (quot, overflow)
            }
        }
    };
}

trait FallbackHelper: Sized {
    type Unsigned;
    fn hi_lo(self) -> (Self, Self);
    fn shift_lo_up(self) -> Self;
    fn shift_lo_up_unsigned(self) -> Self::Unsigned;
    fn combine_lo_then_shl(self, lo: Self::Unsigned, shift: u32) -> (Self, bool);
    fn combine_lo_then_shl_add(self, lo: Self::Unsigned, shift: u32, add: Self) -> (Self, bool);
    fn carrying_add(self, other: Self) -> (Self, Self);
}

impl FallbackHelper for u128 {
    type Unsigned = u128;
    #[inline]
    fn hi_lo(self) -> (u128, u128) {
        (self >> 64, self & !(!0 << 64))
    }

    #[inline]
    fn shift_lo_up(self) -> u128 {
        debug_assert!(self >> 64 == 0);
        self << 64
    }

    #[inline]
    fn shift_lo_up_unsigned(self) -> u128 {
        debug_assert!(self >> 64 == 0);
        self << 64
    }

    #[inline]
    fn combine_lo_then_shl(self, lo: u128, shift: u32) -> (u128, bool) {
        if shift == 128 {
            (self, false)
        } else if shift == 0 {
            (lo, self != 0)
        } else {
            let lo = lo >> shift;
            let hi = self << (128 - shift);
            (lo | hi, self >> shift != 0)
        }
    }

    #[inline]
    fn combine_lo_then_shl_add(self, lo: u128, shift: u32, add: u128) -> (u128, bool) {
        let (combine, overflow1) = if shift == 128 {
            (self, false)
        } else if shift == 0 {
            (lo, self != 0)
        } else {
            let lo = lo >> shift;
            let hi = self << (128 - shift);
            (lo | hi, self >> shift != 0)
        };
        let (ans, overflow2) = combine.overflowing_add(add);
        (ans, overflow1 || overflow2)
    }

    #[inline]
    fn carrying_add(self, rhs: u128) -> (u128, u128) {
        let (sum, overflow) = self.overflowing_add(rhs);
        let carry = if overflow { 1 } else { 0 };
        (sum, carry)
    }
}

impl FallbackHelper for i128 {
    type Unsigned = u128;
    #[inline]
    fn hi_lo(self) -> (i128, i128) {
        (self >> 64, self & !(!0 << 64))
    }

    #[inline]
    fn shift_lo_up(self) -> i128 {
        debug_assert!(self >> 64 == 0);
        self << 64
    }

    #[inline]
    fn shift_lo_up_unsigned(self) -> u128 {
        debug_assert!(self >> 64 == 0);
        (self << 64) as u128
    }

    #[inline]
    fn combine_lo_then_shl(self, lo: u128, shift: u32) -> (i128, bool) {
        if shift == 128 {
            (self, false)
        } else if shift == 0 {
            let ans = lo as i128;
            (ans, self != if ans < 0 { -1 } else { 0 })
        } else {
            let lo = (lo >> shift) as i128;
            let hi = self << (128 - shift);
            let ans = lo | hi;
            (ans, self >> shift != if ans < 0 { -1 } else { 0 })
        }
    }

    #[inline]
    fn combine_lo_then_shl_add(self, lo: u128, shift: u32, add: i128) -> (i128, bool) {
        let (combine_lo, combine_hi) = if shift == 128 {
            (self as u128, if self < 0 { -1 } else { 0 })
        } else if shift == 0 {
            (lo, self)
        } else {
            (
                (lo >> shift) | (self << (128 - shift)) as u128,
                self >> shift,
            )
        };
        let (uns, carry) = combine_lo.overflowing_add(add as u128);
        let ans = uns as i128;
        let mut expected_hi = if ans < 0 { -1 } else { 0 };
        if add < 0 {
            expected_hi += 1;
        }
        if carry {
            expected_hi -= 1;
        }
        (ans, combine_hi != expected_hi)
    }

    #[inline]
    fn carrying_add(self, rhs: i128) -> (i128, i128) {
        let (sum, overflow) = self.overflowing_add(rhs);
        let carry = if overflow {
            if sum < 0 {
                1
            } else {
                -1
            }
        } else {
            0
        };
        (sum, carry)
    }
}

macro_rules! mul_div_fallback {
    ($Single:ty, $Uns:ty, $Signedness:tt) => {
        impl MulDivOverflow for $Single {
            #[inline]
            fn mul_overflow(self, rhs: $Single, frac_nbits: u32) -> ($Single, bool) {
                if frac_nbits == 0 {
                    self.overflowing_mul(rhs)
                } else {
                    let (lh, ll) = self.hi_lo();
                    let (rh, rl) = rhs.hi_lo();
                    let ll_rl = ll.wrapping_mul(rl);
                    let lh_rl = lh.wrapping_mul(rl);
                    let ll_rh = ll.wrapping_mul(rh);
                    let lh_rh = lh.wrapping_mul(rh);

                    let col01 = ll_rl as <$Single as FallbackHelper>::Unsigned;
                    let (col01_hi, col01_lo) = col01.hi_lo();
                    let partial_col12 = lh_rl + col01_hi as $Single;
                    let (col12, carry_col3) = partial_col12.carrying_add(ll_rh);
                    let (col12_hi, col12_lo) = col12.hi_lo();
                    let (_, carry_col3_lo) = carry_col3.hi_lo();
                    let ans01 = col12_lo.shift_lo_up_unsigned() + col01_lo;
                    let ans23 = lh_rh + col12_hi + carry_col3_lo.shift_lo_up();
                    ans23.combine_lo_then_shl(ans01, frac_nbits)
                }
            }

            #[inline]
            fn mul_add_overflow(
                self,
                mul: $Single,
                add: $Single,
                mut frac_nbits: i32,
            ) -> ($Single, bool) {
                const NBITS: i32 = <$Single>::NBITS as i32;

                // l * r + a
                let (lh, ll) = self.hi_lo();
                let (rh, rl) = mul.hi_lo();
                let ll_rl = ll.wrapping_mul(rl);
                let lh_rl = lh.wrapping_mul(rl);
                let ll_rh = ll.wrapping_mul(rh);
                let lh_rh = lh.wrapping_mul(rh);

                let col01 = ll_rl as <$Single as FallbackHelper>::Unsigned;
                let (col01_hi, col01_lo) = col01.hi_lo();
                let partial_col12 = lh_rl + col01_hi as $Single;
                let (col12, carry_col3) = partial_col12.carrying_add(ll_rh);
                let (col12_hi, col12_lo) = col12.hi_lo();
                let (_, carry_col3_lo) = carry_col3.hi_lo();
                let mut ans01 = col12_lo.shift_lo_up_unsigned() + col01_lo;
                let mut ans23 = lh_rh + col12_hi + carry_col3_lo.shift_lo_up();

                let mut overflow2 = false;
                if frac_nbits < 0 {
                    frac_nbits += NBITS;
                    debug_assert!(frac_nbits >= 0);
                    let expected_ans23 = if_signed_unsigned!(
                        $Signedness,
                        ans01.wrapping_as::<$Single>() >> (NBITS - 1),
                        0,
                    );
                    overflow2 = ans23 != expected_ans23;
                    ans23 = ans01.wrapping_as::<$Single>();
                    ans01 = 0;
                } else if frac_nbits > NBITS {
                    frac_nbits -= NBITS;
                    debug_assert!(frac_nbits <= NBITS);
                    let sign_extension = if_signed_unsigned!($Signedness, ans23 >> (NBITS - 1), 0);
                    ans01 = ans23.wrapping_as::<$Uns>();
                    ans23 = sign_extension;
                }

                let (ans, overflow) = ans23.combine_lo_then_shl_add(ans01, frac_nbits as u32, add);
                (ans, overflow2 || overflow)
            }

            #[inline]
            fn div_overflow(self, rhs: $Single, frac_nbits: u32) -> ($Single, bool) {
                if frac_nbits == 0 {
                    self.overflowing_div(rhs)
                } else {
                    const NBITS: u32 = <$Single>::NBITS;
                    let lhs2 = (self >> (NBITS - frac_nbits), (self << frac_nbits) as $Uns);
                    let (quot2, _) = rhs.div_rem_from(lhs2);
                    let quot = quot2.1 as $Single;
                    let overflow = if_signed_unsigned!(
                        $Signedness,
                        quot2.0 != if quot < 0 { -1 } else { 0 },
                        quot2.0 != 0
                    );
                    (quot, overflow)
                }
            }
        }
    };
}

mul_div_widen! { u8, u16, Unsigned }
mul_div_widen! { u16, u32, Unsigned }
mul_div_widen! { u32, u64, Unsigned }
mul_div_widen! { u64, u128, Unsigned }
mul_div_fallback! { u128, u128, Unsigned }
mul_div_widen! { i8, i16, Signed }
mul_div_widen! { i16, i32, Signed }
mul_div_widen! { i32, i64, Signed }
mul_div_widen! { i64, i128, Signed }
mul_div_fallback! { i128, u128, Signed }

#[cfg(test)]
mod tests {
    use crate::{types::extra::Unsigned, *};

    #[test]
    fn fixed_u16() {
        use crate::types::extra::U7 as Frac;
        let frac = Frac::U32;
        let a = 12;
        let b = 5;
        for &(a, b) in &[(a, b), (b, a)] {
            let af = FixedU16::<Frac>::from_num(a);
            let bf = FixedU16::<Frac>::from_num(b);
            assert_eq!((af + bf).to_bits(), (a << frac) + (b << frac));
            if a > b {
                assert_eq!((af - bf).to_bits(), (a << frac) - (b << frac));
            }
            assert_eq!((af * bf).to_bits(), (a << frac) * b);
            assert_eq!((af / bf).to_bits(), (a << frac) / b);
            assert_eq!((af % bf).to_bits(), (a << frac) % (b << frac));
            assert_eq!((af & bf).to_bits(), (a << frac) & (b << frac));
            assert_eq!((af | bf).to_bits(), (a << frac) | (b << frac));
            assert_eq!((af ^ bf).to_bits(), (a << frac) ^ (b << frac));
            assert_eq!((!af).to_bits(), !(a << frac));
            assert_eq!((af << 4u8).to_bits(), (a << frac) << 4);
            assert_eq!((af >> 4i128).to_bits(), (a << frac) >> 4);
            assert_eq!((af * b).to_bits(), (a << frac) * b);
            assert_eq!((b * af).to_bits(), (a << frac) * b);
            assert_eq!((af / b).to_bits(), (a << frac) / b);
            assert_eq!((af % b).to_bits(), (a << frac) % (b << frac));
        }
    }

    #[test]
    fn fixed_i16() {
        use crate::types::extra::U7 as Frac;
        let frac = Frac::U32;
        let a = 12;
        let b = 5;
        for &(a, b) in &[
            (a, b),
            (a, -b),
            (-a, b),
            (-a, -b),
            (b, a),
            (b, -a),
            (-b, a),
            (-b, -a),
        ] {
            let af = FixedI16::<Frac>::from_num(a);
            let bf = FixedI16::<Frac>::from_num(b);
            assert_eq!((af + bf).to_bits(), (a << frac) + (b << frac));
            assert_eq!((af - bf).to_bits(), (a << frac) - (b << frac));
            assert_eq!((af * bf).to_bits(), (a << frac) * b);
            assert_eq!((af / bf).to_bits(), (a << frac) / b);
            assert_eq!((af % bf).to_bits(), (a << frac) % (b << frac));
            assert_eq!((af & bf).to_bits(), (a << frac) & (b << frac));
            assert_eq!((af | bf).to_bits(), (a << frac) | (b << frac));
            assert_eq!((af ^ bf).to_bits(), (a << frac) ^ (b << frac));
            assert_eq!((-af).to_bits(), -(a << frac));
            assert_eq!((!af).to_bits(), !(a << frac));
            assert_eq!((af << 4u8).to_bits(), (a << frac) << 4);
            assert_eq!((af >> 4i128).to_bits(), (a << frac) >> 4);
            assert_eq!((af * b).to_bits(), (a << frac) * b);
            assert_eq!((b * af).to_bits(), (a << frac) * b);
            assert_eq!((af / b).to_bits(), (a << frac) / b);
            assert_eq!((af % b).to_bits(), (a << frac) % (b << frac));
        }
    }

    #[test]
    fn fixed_u128() {
        use crate::types::{U0F128, U121F7, U128F0};

        let frac = U121F7::FRAC_NBITS;
        let a = 0x0003_4567_89ab_cdef_0123_4567_89ab_cdef_u128;
        let b = 5;
        for &(a, b) in &[(a, b), (b, a)] {
            let af = U121F7::from_num(a);
            let bf = U121F7::from_num(b);
            assert_eq!((af + bf).to_bits(), (a << frac) + (b << frac));
            if a > b {
                assert_eq!((af - bf).to_bits(), (a << frac) - (b << frac));
            }
            assert_eq!((af * bf).to_bits(), (a << frac) * b);
            assert_eq!((af / bf).to_bits(), (a << frac) / b);
            assert_eq!((af % bf).to_bits(), (a << frac) % (b << frac));
            assert_eq!((af & bf).to_bits(), (a << frac) & (b << frac));
            assert_eq!((af | bf).to_bits(), (a << frac) | (b << frac));
            assert_eq!((af ^ bf).to_bits(), (a << frac) ^ (b << frac));
            assert_eq!((!af).to_bits(), !(a << frac));
            assert_eq!((af << 4u8).to_bits(), (a << frac) << 4);
            assert_eq!((af >> 4i128).to_bits(), (a << frac) >> 4);
            assert_eq!((af * b).to_bits(), (a << frac) * b);
            assert_eq!((b * af).to_bits(), (a << frac) * b);
            assert_eq!((af / b).to_bits(), (a << frac) / b);
            assert_eq!((af % b).to_bits(), (a << frac) % (b << frac));

            let af = U0F128::from_bits(a);
            let bf = U0F128::from_bits(b);
            assert_eq!(af * bf, 0);
            assert_eq!(af * b, U0F128::from_bits(a * b));
            assert_eq!(a * bf, U0F128::from_bits(a * b));
            assert_eq!(bf * af, 0);

            let af = U128F0::from_num(a);
            let bf = U128F0::from_num(b);
            assert_eq!(af * bf, a * b);
            assert_eq!(af * b, a * b);
            assert_eq!(a * bf, a * b);
            assert_eq!(bf * af, a * b);
            assert_eq!(af / bf, a / b);
            assert_eq!(af / b, a / b);
            assert_eq!(af % bf, a % b);
            assert_eq!(af % b, a % b);
        }
    }

    #[test]
    fn fixed_i128() {
        use crate::types::{I0F128, I121F7, I128F0};

        let frac = I121F7::FRAC_NBITS;
        let a = 0x0003_4567_89ab_cdef_0123_4567_89ab_cdef_i128;
        let b = 5;
        for &(a, b) in &[
            (a, b),
            (a, -b),
            (-a, b),
            (-a, -b),
            (b, a),
            (b, -a),
            (-b, a),
            (-b, -a),
        ] {
            let af = I121F7::from_num(a);
            let bf = I121F7::from_num(b);
            assert_eq!((af + bf).to_bits(), (a << frac) + (b << frac));
            assert_eq!((af - bf).to_bits(), (a << frac) - (b << frac));
            assert_eq!((af * bf).to_bits(), (a << frac) * b);
            assert_eq!((af / bf).to_bits(), (a << frac) / b);
            assert_eq!((af % bf).to_bits(), (a << frac) % (b << frac));
            assert_eq!((af & bf).to_bits(), (a << frac) & (b << frac));
            assert_eq!((af | bf).to_bits(), (a << frac) | (b << frac));
            assert_eq!((af ^ bf).to_bits(), (a << frac) ^ (b << frac));
            assert_eq!((-af).to_bits(), -(a << frac));
            assert_eq!((!af).to_bits(), !(a << frac));
            assert_eq!((af << 4u8).to_bits(), (a << frac) << 4);
            assert_eq!((af >> 4i128).to_bits(), (a << frac) >> 4);
            assert_eq!((af * b).to_bits(), (a << frac) * b);
            assert_eq!((b * af).to_bits(), (a << frac) * b);
            assert_eq!((af / b).to_bits(), (a << frac) / b);
            assert_eq!((af % b).to_bits(), (a << frac) % (b << frac));

            let af = I0F128::from_bits(a);
            let bf = I0F128::from_bits(b);
            let prod = if a.is_negative() == b.is_negative() {
                I0F128::ZERO
            } else {
                -I0F128::DELTA
            };
            assert_eq!(af * bf, prod);
            assert_eq!(af * b, I0F128::from_bits(a * b));
            assert_eq!(a * bf, I0F128::from_bits(a * b));
            assert_eq!(bf * af, prod);

            let af = I128F0::from_num(a);
            let bf = I128F0::from_num(b);
            assert_eq!(af * bf, a * b);
            assert_eq!(af * b, a * b);
            assert_eq!(a * bf, a * b);
            assert_eq!(bf * af, a * b);
            assert_eq!(af / bf, a / b);
            assert_eq!(af / b, a / b);
            assert_eq!(af % bf, a % b);
            assert_eq!(af % b, a % b);
        }
    }

    fn check_rem_int(a: i32, b: i32) {
        use crate::types::I16F16;
        assert_eq!(I16F16::from_num(a) % b, a % b);
        assert_eq!(I16F16::from_num(a).rem_euclid_int(b), a.rem_euclid(b));
        match (I16F16::from_num(a).checked_rem_int(b), a.checked_rem(b)) {
            (Some(a), Some(b)) => assert_eq!(a, b),
            (None, None) => {}
            (a, b) => panic!("mismatch {:?}, {:?}", a, b),
        }
        match (
            I16F16::from_num(a).checked_rem_euclid_int(b),
            a.checked_rem_euclid(b),
        ) {
            (Some(a), Some(b)) => assert_eq!(a, b),
            (None, None) => {}
            (a, b) => panic!("mismatch {:?}, {:?}", a, b),
        }
    }

    #[test]
    #[allow(clippy::modulo_one)]
    fn rem_int() {
        use crate::types::{I0F32, I16F16, I1F31};
        check_rem_int(-0x8000, -0x8000);
        check_rem_int(-0x8000, -0x7fff);
        check_rem_int(-0x8000, 0x7fff);
        check_rem_int(-0x8000, 0x8000);
        check_rem_int(-0x7fff, -0x8000);
        check_rem_int(-0x7fff, -0x7fff);
        check_rem_int(-0x7fff, 0x7fff);
        check_rem_int(-0x7fff, 0x8000);
        check_rem_int(0x7fff, -0x8000);
        check_rem_int(0x7fff, -0x7fff);
        check_rem_int(0x7fff, 0x7fff);
        check_rem_int(0x7fff, 0x8000);

        fn i1(f: f32) -> I1F31 {
            I1F31::from_num(f)
        }
        fn i0(f: f32) -> I0F32 {
            I0F32::from_num(f)
        }

        assert_eq!(I16F16::MIN % -1, 0);
        assert_eq!(I16F16::MIN.checked_rem_int(-1).unwrap(), 0);
        assert_eq!(I16F16::MIN.rem_euclid_int(-1), 0);
        assert_eq!(I16F16::MIN.checked_rem_euclid_int(-1).unwrap(), 0);

        assert_eq!(i1(-1.0) % 1, i1(0.0));
        assert_eq!(i1(-1.0).rem_euclid_int(1), i1(0.0));

        assert_eq!(i1(-0.75) % 1, i1(-0.75));
        assert_eq!(i1(-0.75).rem_euclid_int(1), i1(0.25));

        assert_eq!(i1(-0.5) % 1, i1(-0.5));
        assert_eq!(i1(-0.5).rem_euclid_int(1), i1(0.5));

        assert_eq!(i1(-0.5) % 3, i1(-0.5));
        assert_eq!(i1(-0.5).checked_rem_euclid_int(3), None);
        assert_eq!(i1(-0.5).wrapping_rem_euclid_int(3), i1(0.5));
        assert_eq!(i1(-0.5).overflowing_rem_euclid_int(3), (i1(0.5), true));

        assert_eq!(i1(-0.25) % 1, i1(-0.25));
        assert_eq!(i1(-0.25).rem_euclid_int(1), i1(0.75));

        assert_eq!(i1(-0.25) % 3, i1(-0.25));
        assert_eq!(i1(-0.25).checked_rem_euclid_int(3), None);
        assert_eq!(i1(-0.25).wrapping_rem_euclid_int(3), i1(0.75));
        assert_eq!(i1(-0.25).overflowing_rem_euclid_int(3), (i1(0.75), true));

        assert_eq!(i1(0.0) % 1, i1(0.0));
        assert_eq!(i1(0.0).rem_euclid_int(1), i1(0.0));

        assert_eq!(i1(0.25) % 1, i1(0.25));
        assert_eq!(i1(0.25).rem_euclid_int(1), i1(0.25));

        assert_eq!(i1(0.5) % 1, i1(0.5));
        assert_eq!(i1(0.5).rem_euclid_int(1), i1(0.5));

        assert_eq!(i1(0.75) % 1, i1(0.75));
        assert_eq!(i1(0.75).rem_euclid_int(1), i1(0.75));

        assert_eq!(i0(-0.5) % 1, i0(-0.5));
        assert_eq!(i0(-0.5).checked_rem_euclid_int(1), None);
        assert_eq!(i0(-0.5).wrapping_rem_euclid_int(1), i0(-0.5));
        assert_eq!(i0(-0.5).overflowing_rem_euclid_int(1), (i0(-0.5), true));

        assert_eq!(i0(-0.375) % 1, i0(-0.375));
        assert_eq!(i0(-0.375).checked_rem_euclid_int(1), None);
        assert_eq!(i0(-0.375).wrapping_rem_euclid_int(1), i0(-0.375));
        assert_eq!(i0(-0.375).overflowing_rem_euclid_int(1), (i0(-0.375), true));

        assert_eq!(i0(-0.25) % 1, i0(-0.25));
        assert_eq!(i0(-0.25).checked_rem_euclid_int(1), None);
        assert_eq!(i0(-0.25).wrapping_rem_euclid_int(1), i0(-0.25));
        assert_eq!(i0(-0.25).overflowing_rem_euclid_int(1), (i0(-0.25), true));

        assert_eq!(i0(0.0) % 1, i0(0.0));
        assert_eq!(i0(0.0).rem_euclid_int(1), i0(0.0));

        assert_eq!(i0(0.25) % 1, i0(0.25));
        assert_eq!(i0(0.25).rem_euclid_int(1), i0(0.25));
    }

    #[test]
    fn div_rem_nonzero() {
        use crate::types::{U0F32, U16F16, U32F0};
        use core::num::NonZeroU32;
        let half_bits = u32::from(u16::MAX);
        let vals = &[
            0,
            1,
            100,
            5555,
            half_bits - 1,
            half_bits,
            half_bits + 1,
            u32::MAX - 1,
            u32::MAX,
        ];
        for &a in vals {
            for &b in vals {
                let all_frac = U0F32::from_bits(a);
                let some_frac = U16F16::from_bits(a);
                let no_frac = U32F0::from_bits(a);
                let nz = match NonZeroU32::new(b) {
                    Some(s) => s,
                    None => continue,
                };
                assert_eq!(all_frac / b, all_frac / nz);
                assert_eq!(all_frac % b, all_frac % nz);
                assert_eq!(some_frac / b, some_frac / nz);
                assert_eq!(some_frac % b, some_frac % nz);
                assert_eq!(no_frac / b, no_frac / nz);
                assert_eq!(no_frac % b, no_frac % nz);
            }
        }
    }

    macro_rules! check_mul_add {
        ($($F:ty)*) => { $(
            let min = <$F>::MIN;
            let max = <$F>::MAX;
            let hmax = max / 2;
            let delta = <$F>::DELTA;
            let zero = <$F>::ZERO;
            let one = <$F>::ONE;
            let three = one * 3;
            let m_hmax = zero.wrapping_sub(hmax);
            let m_delta = zero.wrapping_sub(delta);
            let max_m_delta = max - delta;
            assert_eq!(max.overflowing_mul_add(one, zero), (max, false));
            assert_eq!(max.overflowing_mul_add(one, delta), (min, true));
            assert_eq!(max.overflowing_mul_add(one, m_delta), (max_m_delta, m_delta > 0));
            assert_eq!(max.overflowing_mul_add(three, max), (<$F>::from_bits(!0 << 2), true));
            assert_eq!(hmax.overflowing_mul_add(three, m_hmax), (hmax * 2, m_hmax > 0));
        )* };
    }

    macro_rules! check_mul_add_no_int {
        ($($F:ty)*) => { $(
            let min = <$F>::MIN;
            let max = <$F>::MAX;
            let hmax = max / 2;
            let delta = <$F>::DELTA;
            let zero = <$F>::ZERO;
            let quarter = delta << (<$F>::FRAC_NBITS - 2);
            assert_eq!(max.overflowing_mul_add(quarter, zero), (max >> 2, false));
            if <$F>::IS_SIGNED {
                assert_eq!(max.overflowing_mul_add(max, zero), (hmax, false));
                assert_eq!(max.overflowing_mul_add(max, max), (min + hmax - delta, true));
            } else {
                assert_eq!(max.overflowing_mul_add(max, zero), (max - delta, false));
                assert_eq!(max.overflowing_mul_add(max, max), (max - 2 * delta, true));
            }
        )* };
    }

    #[test]
    fn mul_add() {
        use crate::types::*;
        check_mul_add! { I3F5 I3F13 I3F29 I3F61 I3F125 }
        check_mul_add! { I4F4 I8F8 I16F16 I32F32 I64F64 }
        check_mul_add! { I8F0 I16F0 I32F0 I64F0 I128F0 }
        check_mul_add! { U2F6 U2F14 U2F30 U2F62 U2F126 }
        check_mul_add! { U4F4 U8F8 U16F16 U32F32 U64F64 }
        check_mul_add! { U8F0 U16F0 U32F0 U64F0 U128F0 }

        check_mul_add_no_int! { I0F8 I0F16 I0F32 I0F64 I0F128 }
        check_mul_add_no_int! { U0F8 U0F16 U0F32 U0F64 U0F128 }
    }

    #[test]
    fn mul_add_overflow_large_frac_nbits() {
        let nbits_2 = 128;

        let max = u64::MAX;

        assert_eq!(max.mul_add_overflow(max, max, nbits_2), (max, false));
        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 1), (0, true));
        assert_eq!(
            max.mul_add_overflow(max, max - 1, nbits_2 - 1),
            (max, false)
        );

        let (min, max) = (i64::MIN, i64::MAX);

        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 2), (max, false));
        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 3), (min, true));
        assert_eq!(
            max.mul_add_overflow(max, max - 1, nbits_2 - 3),
            (max, false)
        );

        assert_eq!(min.mul_add_overflow(min, max, nbits_2 - 1), (max, false));
        assert_eq!(min.mul_add_overflow(min, max, nbits_2 - 2), (min, true));
        assert_eq!(
            min.mul_add_overflow(min, max - 1, nbits_2 - 2),
            (max, false)
        );

        assert_eq!(max.mul_add_overflow(min, -max, nbits_2 - 2), (min, false));
        assert_eq!(max.mul_add_overflow(min, -max, nbits_2 - 3), (max, true));
        assert_eq!(
            max.mul_add_overflow(min, -max + 1, nbits_2 - 3),
            (min, false)
        );

        let nbits_2 = 256;

        let max = u128::MAX;

        assert_eq!(max.mul_add_overflow(max, max, nbits_2), (max, false));
        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 1), (0, true));
        assert_eq!(
            max.mul_add_overflow(max, max - 1, nbits_2 - 1),
            (max, false)
        );

        let (min, max) = (i128::MIN, i128::MAX);

        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 2), (max, false));
        assert_eq!(max.mul_add_overflow(max, max, nbits_2 - 3), (min, true));
        assert_eq!(
            max.mul_add_overflow(max, max - 1, nbits_2 - 3),
            (max, false)
        );

        assert_eq!(min.mul_add_overflow(min, max, nbits_2 - 1), (max, false));
        assert_eq!(min.mul_add_overflow(min, max, nbits_2 - 2), (min, true));
        assert_eq!(
            min.mul_add_overflow(min, max - 1, nbits_2 - 2),
            (max, false)
        );

        assert_eq!(max.mul_add_overflow(min, -max, nbits_2 - 2), (min, false));
        assert_eq!(max.mul_add_overflow(min, -max, nbits_2 - 3), (max, true));
        assert_eq!(
            max.mul_add_overflow(min, -max + 1, nbits_2 - 3),
            (min, false)
        );
    }

    #[test]
    fn mul_add_overflow_neg_frac_nbits() {
        let nbits = 64;

        let (zero, one, max) = (0u64, 1u64, u64::MAX);

        assert_eq!(zero.mul_add_overflow(zero, max, -nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, max, -nbits), (max, true));
        assert_eq!(
            one.mul_add_overflow(one, zero, 1 - nbits),
            (max - max / 2, false)
        );
        assert_eq!(one.mul_add_overflow(one, max, 1 - nbits), (max / 2, true));

        let (zero, one, min, max) = (0i64, 1i64, i64::MIN, i64::MAX);

        assert_eq!(zero.mul_add_overflow(zero, max, -nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, max, -nbits), (max, true));
        assert_eq!(one.mul_add_overflow(one, -one, 1 - nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, zero, 1 - nbits), (min, true));

        assert_eq!((-one).mul_add_overflow(-one, max, -nbits), (max, true));
        assert_eq!((-one).mul_add_overflow(-one, -one, 1 - nbits), (max, false));
        assert_eq!((-one).mul_add_overflow(-one, zero, 1 - nbits), (min, true));

        assert_eq!(one.mul_add_overflow(-one, max, -nbits), (max, true));
        assert_eq!(one.mul_add_overflow(-one, min, -nbits), (min, true));
        assert_eq!(one.mul_add_overflow(-one, zero, 1 - nbits), (min, false));
        assert_eq!(one.mul_add_overflow(-one, max, 1 - nbits), (-one, false));

        let nbits = 128;

        let (zero, one, max) = (0u128, 1u128, u128::MAX);

        assert_eq!(zero.mul_add_overflow(zero, max, -nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, max, -nbits), (max, true));
        assert_eq!(
            one.mul_add_overflow(one, zero, 1 - nbits),
            (max - max / 2, false)
        );
        assert_eq!(one.mul_add_overflow(one, max, 1 - nbits), (max / 2, true));

        let (zero, one, min, max) = (0i128, 1i128, i128::MIN, i128::MAX);

        assert_eq!(zero.mul_add_overflow(zero, max, -nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, max, -nbits), (max, true));
        assert_eq!(one.mul_add_overflow(one, -one, 1 - nbits), (max, false));
        assert_eq!(one.mul_add_overflow(one, zero, 1 - nbits), (min, true));

        assert_eq!((-one).mul_add_overflow(-one, max, -nbits), (max, true));
        assert_eq!((-one).mul_add_overflow(-one, -one, 1 - nbits), (max, false));
        assert_eq!((-one).mul_add_overflow(-one, zero, 1 - nbits), (min, true));

        assert_eq!(one.mul_add_overflow(-one, max, -nbits), (max, true));
        assert_eq!(one.mul_add_overflow(-one, min, -nbits), (min, true));
        assert_eq!(one.mul_add_overflow(-one, zero, 1 - nbits), (min, false));
        assert_eq!(one.mul_add_overflow(-one, max, 1 - nbits), (-one, false));
    }

    #[test]
    fn issue_26() {
        use crate::{
            types::extra::{U120, U121, U122, U123, U124},
            FixedI128, FixedU128,
        };

        // issue 26 is about FixedI128<U123>, the others are just some extra tests

        let x: FixedI128<U120> = "-9.079999999999999999999".parse().unwrap();
        let squared = x.checked_mul(x).unwrap();
        assert!(82.44639 < squared && squared < 82.44641);
        let msquared = (-x).checked_mul(x).unwrap();
        assert!(-82.44641 < msquared && msquared < -82.44639);
        assert_eq!(x.checked_mul(-x), Some(msquared));
        assert_eq!((-x).checked_mul(-x), Some(squared));

        // 82 requires 8 signed integer bits
        let x: FixedI128<U121> = "-9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());
        assert!((-x).checked_mul(x).is_none());
        assert!(x.checked_mul(-x).is_none());
        assert!((-x).checked_mul(-x).is_none());
        let x: FixedI128<U122> = "-9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());
        assert!((-x).checked_mul(x).is_none());
        assert!(x.checked_mul(-x).is_none());
        assert!((-x).checked_mul(-x).is_none());
        let x: FixedI128<U123> = "-9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());
        assert!((-x).checked_mul(x).is_none());
        assert!(x.checked_mul(-x).is_none());
        assert!((-x).checked_mul(-x).is_none());

        let x: Result<FixedI128<U124>, _> = "-9.079999999999999999999".parse();
        assert!(x.is_err());

        // Test unsigned

        let x: FixedU128<U120> = "9.079999999999999999999".parse().unwrap();
        let squared = x.checked_mul(x).unwrap();
        assert!(82.44639 < squared && squared < 82.44641);

        // 82 requires 8 signed integer bits
        let x: FixedU128<U122> = "9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());
        let x: FixedU128<U123> = "9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());
        let x: FixedU128<U124> = "9.079999999999999999999".parse().unwrap();
        assert!(x.checked_mul(x).is_none());

        let x: Result<FixedI128<U125>, _> = "9.079999999999999999999".parse();
        assert!(x.is_err());
    }
}
