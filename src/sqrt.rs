// Copyright © 2018–2024 Trevor Spiteri

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

use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

// q_i = sqrt(y) truncated to i bits after point.
// q_0 = 1
// y_i = 2^i (y - q_i^2)
// y_0 = y - 1
//
// If (q_i + 1>>(i+1))^2 <= y:
//     q_(i+1) = q_i + 1>>(i+1)
// Else:
//     q_(i+1) = q_i
//
// Equivalently:
//
// If q_i + 1>>(i+2) <= y_i:
//     q_(i+1) = q_i + 1>>(i+1)
//     y_(i+1) = 2 (y_i - q_i - 1>>(i+2))
// Else:
//     q_i+1 = q_i
//     y_i+1 = 2 y_i
//
//   * Iterations do not include q_0, y_0 as they are initialization.
//   * i goes from 1 to iter.
//   * Both q and y are stored with 2 integer bits. q is in range [1, 2); y is
//     in range [1, 4).
//   * 1>>(i+2) needs special code when i + 2 > nbits - 2. Since maximum iter is
//     nbits - 1, i + 2 can be nbits + 1 which is > nbits - 2 by 2.
//
// Some examples for u8.
//
// frac_nbits == 0:
//     sip = 4 - leading / 2
//     4 significant int pairs: 0100 0000. -> 0000 1000. (y << 0, 3 iter, q >> 3)
//     3 significant int pairs: 0001 0000. -> 0000 0100. (y << 2, 2 iter, q >> 4)
//     2 significant int pairs: 0000 0100. -> 0000 0010. (y << 4, 1 iter, q >> 5)
//     1 significant int pairs: 0000 0001. -> 0000 0001. (y << 6, 0 iter, q >> 6)
//     General: y << 8 - 2sip, -1 + sip iter, q >> 7 - sip
//
// frac_nbits == 1:
//     sip = 4 - (leading + 1) / 2
//     4 significant int pairs: 100 0000.0 -> 000 1000.0 (y >> 1, 4 iter, q >> 2)
//     3 significant int pairs: 001 0000.0 -> 000 0100.0 (y << 1, 3 iter, q >> 3)
//     2 significant int pairs: 000 0100.0 -> 000 0010.0 (y << 3, 2 iter, q >> 4)
//     1 significant int pairs: 000 0001.0 -> 000 0001.0 (y << 5, 1 iter, q >> 5)
//     0 significant int pairs: 000 0000.1 -> 000 0000.1 (y << 7, 0 iter, q >> 6)
//     General: y << 7 - 2sip, sip iter, q >> 6 - sip
//
// frac_nbits == 2:
//     sip = 3 - leading / 2
//     3 significant int pairs: 01 0000.00 -> 00 0100.00 (y << 0, 4 iter, q >> 2)
//     2 significant int pairs: 00 0100.00 -> 00 0010.00 (y << 2, 3 iter, q >> 3)
//     1 significant int pairs: 00 0001.00 -> 00 0001.00 (y << 4, 2 iter, q >> 4)
//     0 significant int pairs: 00 0000.01 -> 00 0000.10 (y << 6, 1 iter, q >> 5)
//     General: y << 6 - 2sip, 1 + sip iter, q >> 5 - sip
//
// frac_nbits = 3:
//     sip = 3 - (leading + 1) / 2
//     3 significant int pairs: 1 0000.000 -> 0 0100.000 (y >> 1, 5 iter, q >> 1)
//     2 significant int pairs: 0 0100.000 -> 0 0010.000 (y << 1, 4 iter, q >> 2)
//     1 significant int pairs: 0 0001.000 -> 0 0001.000 (y << 3, 3 iter, q >> 3)
//     0 significant int pairs: 0 0000.010 -> 0 0000.100 (y << 5, 2 iter, q >> 4)
//    -1 significant int pairs: 0 0000.001 -> 0 0000.010 (y << 7, 1 iter, q >> 5)
//     General: y << 5 - 2sip, 2 + sip iter, q >> 4 - sip
//
// frac_nbits == 4:
//     sip = 2 - leading / 2
//     2 significant int pairs: 0100.0000 -> 0010.0000 (y << 0, 5 iter, q >> 1)
//     1 significant int pairs: 0001.0000 -> 0001.0000 (y << 2, 4 iter, q >> 2)
//     0 significant int pairs: 0000.0100 -> 0000.1000 (y << 4, 3 iter, q >> 3)
//    -1 significant int pairs: 0000.0001 -> 0000.0100 (y << 6, 2 iter, q >> 4)
//     General: y << 4 - 2sip, 3 + sip iter, q >> 3 - sip
//
// frac_nbits = 5:
//     sip = 2 - (leading + 1) / 2
//     2 significant int pairs: 100.0000 0 -> 010.0000 0 (y >> 1, 6 iter, q >> 0)
//     1 significant int pairs: 001.0000 0 -> 001.0000 0 (y << 1, 5 iter, q >> 1)
//     0 significant int pairs: 000.0100 0 -> 000.1000 0 (y << 3, 4 iter, q >> 2)
//    -1 significant int pairs: 000.0001 0 -> 000.0100 0 (y << 5, 3 iter, q >> 3)
//    -2 significant int pairs: 000.0000 1 -> 000.0010 1 (y << 7, 2 iter, q >> 4)
//     General: y << 3 - 2sip, 4 + sip iter, q >> 2 - sip
//
// frac_nbits == 6:
//     sip = 1 - leading / 2
//     1 significant int pairs: 01.0000 00 -> 01.0000 00 (y << 0, 6 iter, q >> 0)
//     0 significant int pairs: 00.0100 00 -> 00.1000 00 (y << 2, 5 iter, q >> 1)
//    -1 significant int pairs: 00.0001 00 -> 00.0100 00 (y << 4, 4 iter, q >> 2)
//    -2 significant int pairs: 00.0000 01 -> 00.0010 00 (y << 6, 3 iter, q >> 3)
//     General: y << 2 - 2sip, 5 + sip iter, q >> 1 - sip
//
// frac_nbits == 7:
//     sip = 1 - (leading + 1) / 2
//     1 significant int pairs: 1.0000 000 -> 1.0000 000 (y >> 1, 7 iter, q << 1)
//     0 significant int pairs: 0.0100 000 -> 0.1000 000 (y << 1, 6 iter, q >> 0)
//    -1 significant int pairs: 0.0001 000 -> 0.0100 000 (y << 3, 5 iter, q >> 1)
//    -2 significant int pairs: 0.0000 010 -> 0.0010 000 (y << 5, 4 iter, q >> 2)
//    -3 significant int pairs: 0.0000 001 -> 0.0001 011 (y << 7, 3 iter, q >> 3)
//     General: y << 1 - 2sip, 6 + sip iter, q >> -sip
//
// frac_nbits == 8:
//     sip = 0 - leading / 2
//     0 significant int pairs: .0100 0000 -> .1000 0000 (y << 0, 7 iter, q << 1)
//    -1 significant int pairs: .0001 0000 -> .0100 0000 (y << 2, 6 iter, q >> 0)
//    -2 significant int pairs: .0000 0100 -> .0010 0000 (y << 4, 5 iter, q >> 1)
//    -3 significant int pairs: .0000 0001 -> .0001 0000 (y << 6, 4 iter, q >> 2)
//     General: y << -2sip, 7 + sip iter, q >> -1 - sip
//
// General:
//     Even frac_nbits:
//         sip = int_nbits / 2 - leading / 2
//     Odd frac_nbits:
//         sip = (int_nbits + 1) / 2 - (leading + 1) / 2
//     Then:
//         y << int_nbits - 2sip, frac_nbits - 1 + sip iter, q >> int_nbits - 1 - sip

macro_rules! impl_sqrt {
    ($u:ident, $NZ:ident) => {
        pub const fn $u(val: $NZ, frac_nbits: u32) -> $u {
            let int_nbits = $u::BITS - frac_nbits;
            let odd_frac_nbits = frac_nbits % 2 != 0;
            let leading = val.leading_zeros();
            let sig_int_pairs = if odd_frac_nbits {
                ((int_nbits + 1) / 2) as i32 - ((leading + 1) / 2) as i32
            } else {
                (int_nbits / 2) as i32 - (leading / 2) as i32
            };

            let mut i = 1;
            let mut q_i = 1 << ($u::BITS - 2);
            let mut next_bit = q_i >> 1;
            let mut y_i = val.get();
            let input_shl = int_nbits as i32 - sig_int_pairs * 2;
            if input_shl < 0 {
                // This can only happen when we have odd frac_nbits and the most
                // significant bit is set. We would need to shift right by 1.
                debug_assert!(input_shl == -1);

                // Do one iteration here as this is a special case.

                // In this special case, y is in the range [1, 2) instead of [1, 4),
                // and q is in the range [1, √2) instead of [1, 2).
                // Therefore, q_1 is always 0b1.0, and never 0b1.1.
                // Since q_0 = q_1 = 1, y_1 = 2 × (y - q_1^2) = 2 × y - 2 × q_i.
                // Since input_shl is -1, its effect is cancelled out by 2 × y,
                // and we only need to subtract 2 × q_i from y_i.
                y_i -= 2 * q_i;
                next_bit >>= 1;
                i += 1;
            } else {
                y_i <<= input_shl;
                y_i -= q_i;
            };

            let iters = (frac_nbits as i32 - 1 + sig_int_pairs) as u32;
            while i <= iters {
                let d = next_bit >> 1;
                if d == 0 {
                    if i == iters {
                        // Here result_shr must be 0, otherwise we wouldn't have
                        // room to potentially insert one extra bit.
                        debug_assert!(int_nbits as i32 - 1 - sig_int_pairs == 0);

                        // d == 0.5, so we really need q_i + 0.5 <= y_i,
                        // which can be obtained with q_i < y_i
                        if q_i < y_i {
                            q_i += 1;
                        }

                        return q_i;
                    }

                    debug_assert!(i == iters - 1);
                    // Here result_shr must be -1, otherwise we wouldn't have
                    // room to potentially insert two extra bits.
                    debug_assert!(int_nbits as i32 - 1 - sig_int_pairs == -1);

                    // d == 0.5, so we really need q_i + 0.5 <= y_i,
                    // which can be obtained with q_i < y_i
                    if q_i < y_i {
                        // We cannot subtract d == 0.5 from y_i immediately, so
                        // we subtract 1 from y_i before the multiplication by 2
                        // and then add 1 back. (There may be a potential overflow
                        // if we multiply y_i by 2 and then subtract 1.)
                        y_i -= q_i + 1;
                        y_i *= 2;
                        y_i += 1;
                        q_i += 1;
                    } else {
                        y_i *= 2;
                    }

                    // d == 0.25, so we really need q_i + 0.25 <= y_i,
                    // which can be obtained with q_i < y_i
                    if q_i < y_i {
                        // We cannot add next_bit == 0.5 to q_i immediately, so
                        // we add 1 to q_i after the left shift.
                        q_i = (q_i << 1) + 1;
                    } else {
                        q_i <<= 1;
                    }

                    return q_i;
                }

                if q_i + d <= y_i {
                    y_i -= q_i + d;
                    q_i += next_bit;
                }
                y_i *= 2;

                next_bit = d;
                i += 1;
            }
            let result_shr = int_nbits as i32 - 1 - sig_int_pairs;
            q_i >> result_shr
        }
    };
}

impl_sqrt! { u8, NonZeroU8 }
impl_sqrt! { u16, NonZeroU16 }
impl_sqrt! { u32, NonZeroU32 }
impl_sqrt! { u64, NonZeroU64 }
impl_sqrt! { u128, NonZeroU128 }

#[cfg(test)]
mod tests {
    use crate::{
        FixedI128, FixedI16, FixedI32, FixedI64, FixedI8, FixedU128, FixedU16, FixedU32, FixedU64,
        FixedU8,
    };

    macro_rules! check_sqrt {
        ($val:expr) => {{
            let sqrt = $val.sqrt();
            assert!(sqrt * sqrt <= $val);
            let delta = $val.wrapping_neg().wrapping_sub(!$val);
            if let Some(sqrt_delta) = sqrt.checked_add(delta) {
                if let Some(prod) = sqrt_delta.checked_mul(sqrt_delta) {
                    assert!(prod >= $val);
                }
            }
        }};
    }

    #[test]
    fn check_max_8() {
        check_sqrt!(FixedU8::<0>::MAX);
        check_sqrt!(FixedU8::<1>::MAX);
        check_sqrt!(FixedU8::<3>::MAX);
        check_sqrt!(FixedU8::<4>::MAX);
        check_sqrt!(FixedU8::<5>::MAX);
        check_sqrt!(FixedU8::<7>::MAX);
        check_sqrt!(FixedU8::<8>::MAX);
        assert_eq!(FixedU8::<8>::MAX.sqrt(), FixedU8::<8>::MAX);

        check_sqrt!(FixedI8::<0>::MAX);
        check_sqrt!(FixedI8::<1>::MAX);
        check_sqrt!(FixedI8::<3>::MAX);
        check_sqrt!(FixedI8::<4>::MAX);
        check_sqrt!(FixedI8::<5>::MAX);
        check_sqrt!(FixedI8::<7>::MAX);
        assert!(FixedI8::<8>::MAX.checked_sqrt().is_none());
    }

    #[test]
    fn check_max_16() {
        check_sqrt!(FixedU16::<0>::MAX);
        check_sqrt!(FixedU16::<1>::MAX);
        check_sqrt!(FixedU16::<7>::MAX);
        check_sqrt!(FixedU16::<8>::MAX);
        check_sqrt!(FixedU16::<9>::MAX);
        check_sqrt!(FixedU16::<15>::MAX);
        check_sqrt!(FixedU16::<16>::MAX);
        assert_eq!(FixedU16::<16>::MAX.sqrt(), FixedU16::<16>::MAX);

        check_sqrt!(FixedI16::<0>::MAX);
        check_sqrt!(FixedI16::<1>::MAX);
        check_sqrt!(FixedI16::<7>::MAX);
        check_sqrt!(FixedI16::<8>::MAX);
        check_sqrt!(FixedI16::<9>::MAX);
        check_sqrt!(FixedI16::<15>::MAX);
        assert!(FixedI16::<16>::MAX.checked_sqrt().is_none());
    }

    #[test]
    fn check_max_32() {
        check_sqrt!(FixedU32::<0>::MAX);
        check_sqrt!(FixedU32::<1>::MAX);
        check_sqrt!(FixedU32::<15>::MAX);
        check_sqrt!(FixedU32::<16>::MAX);
        check_sqrt!(FixedU32::<17>::MAX);
        check_sqrt!(FixedU32::<31>::MAX);
        check_sqrt!(FixedU32::<32>::MAX);
        assert_eq!(FixedU32::<32>::MAX.sqrt(), FixedU32::<32>::MAX);

        check_sqrt!(FixedI32::<0>::MAX);
        check_sqrt!(FixedI32::<1>::MAX);
        check_sqrt!(FixedI32::<15>::MAX);
        check_sqrt!(FixedI32::<16>::MAX);
        check_sqrt!(FixedI32::<17>::MAX);
        check_sqrt!(FixedI32::<31>::MAX);
        assert!(FixedI32::<32>::MAX.checked_sqrt().is_none());
    }

    #[test]
    fn check_max_64() {
        check_sqrt!(FixedU64::<0>::MAX);
        check_sqrt!(FixedU64::<1>::MAX);
        check_sqrt!(FixedU64::<31>::MAX);
        check_sqrt!(FixedU64::<32>::MAX);
        check_sqrt!(FixedU64::<33>::MAX);
        check_sqrt!(FixedU64::<63>::MAX);
        check_sqrt!(FixedU64::<64>::MAX);
        assert_eq!(FixedU64::<64>::MAX.sqrt(), FixedU64::<64>::MAX);

        check_sqrt!(FixedI64::<0>::MAX);
        check_sqrt!(FixedI64::<1>::MAX);
        check_sqrt!(FixedI64::<31>::MAX);
        check_sqrt!(FixedI64::<32>::MAX);
        check_sqrt!(FixedI64::<33>::MAX);
        check_sqrt!(FixedI64::<63>::MAX);
        assert!(FixedI64::<64>::MAX.checked_sqrt().is_none());
    }

    #[test]
    fn check_max_128() {
        check_sqrt!(FixedU128::<0>::MAX);
        check_sqrt!(FixedU128::<1>::MAX);
        check_sqrt!(FixedU128::<63>::MAX);
        check_sqrt!(FixedU128::<64>::MAX);
        check_sqrt!(FixedU128::<65>::MAX);
        check_sqrt!(FixedU128::<127>::MAX);
        check_sqrt!(FixedU128::<128>::MAX);
        assert_eq!(FixedU128::<128>::MAX.sqrt(), FixedU128::<128>::MAX);

        check_sqrt!(FixedI128::<0>::MAX);
        check_sqrt!(FixedI128::<1>::MAX);
        check_sqrt!(FixedI128::<63>::MAX);
        check_sqrt!(FixedI128::<64>::MAX);
        check_sqrt!(FixedI128::<65>::MAX);
        check_sqrt!(FixedI128::<127>::MAX);
        assert!(FixedI128::<128>::MAX.checked_sqrt().is_none());
    }

    #[test]
    fn check_two_8() {
        assert_eq!(FixedU8::<0>::from_num(2).sqrt(), FixedU8::<0>::SQRT_2);
        assert_eq!(FixedU8::<1>::from_num(2).sqrt(), FixedU8::<1>::SQRT_2);
        assert_eq!(FixedU8::<3>::from_num(2).sqrt(), FixedU8::<3>::SQRT_2);
        assert_eq!(FixedU8::<4>::from_num(2).sqrt(), FixedU8::<4>::SQRT_2);
        assert_eq!(FixedU8::<5>::from_num(2).sqrt(), FixedU8::<5>::SQRT_2);
        assert_eq!(FixedU8::<6>::from_num(2).sqrt(), FixedU8::<6>::SQRT_2);
        assert!(
            FixedU8::<7>::MAX.sqrt() == FixedU8::<7>::SQRT_2 - FixedU8::<7>::DELTA
                || FixedU8::<7>::MAX.sqrt() == FixedU8::<7>::SQRT_2
        );

        assert_eq!(FixedI8::<0>::from_num(2).sqrt(), FixedI8::<0>::SQRT_2);
        assert_eq!(FixedI8::<1>::from_num(2).sqrt(), FixedI8::<1>::SQRT_2);
        assert_eq!(FixedI8::<3>::from_num(2).sqrt(), FixedI8::<3>::SQRT_2);
        assert_eq!(FixedI8::<4>::from_num(2).sqrt(), FixedI8::<4>::SQRT_2);
        assert_eq!(FixedI8::<5>::from_num(2).sqrt(), FixedI8::<5>::SQRT_2);
        assert!(
            FixedI8::<6>::MAX.sqrt() == FixedI8::<6>::SQRT_2 - FixedI8::<6>::DELTA
                || FixedI8::<6>::MAX.sqrt() == FixedI8::<6>::SQRT_2
        );
    }

    #[test]
    fn check_two_16() {
        assert_eq!(FixedU16::<0>::from_num(2).sqrt(), FixedU16::<0>::SQRT_2);
        assert_eq!(FixedU16::<1>::from_num(2).sqrt(), FixedU16::<1>::SQRT_2);
        assert_eq!(FixedU16::<7>::from_num(2).sqrt(), FixedU16::<7>::SQRT_2);
        assert_eq!(FixedU16::<8>::from_num(2).sqrt(), FixedU16::<8>::SQRT_2);
        assert_eq!(FixedU16::<9>::from_num(2).sqrt(), FixedU16::<9>::SQRT_2);
        assert_eq!(FixedU16::<13>::from_num(2).sqrt(), FixedU16::<13>::SQRT_2);
        assert_eq!(FixedU16::<14>::from_num(2).sqrt(), FixedU16::<14>::SQRT_2);
        assert!(
            FixedU16::<15>::MAX.sqrt() == FixedU16::<15>::SQRT_2 - FixedU16::<15>::DELTA
                || FixedU16::<15>::MAX.sqrt() == FixedU16::<15>::SQRT_2
        );

        assert_eq!(FixedI16::<0>::from_num(2).sqrt(), FixedI16::<0>::SQRT_2);
        assert_eq!(FixedI16::<1>::from_num(2).sqrt(), FixedI16::<1>::SQRT_2);
        assert_eq!(FixedI16::<7>::from_num(2).sqrt(), FixedI16::<7>::SQRT_2);
        assert_eq!(FixedI16::<8>::from_num(2).sqrt(), FixedI16::<8>::SQRT_2);
        assert_eq!(FixedI16::<9>::from_num(2).sqrt(), FixedI16::<9>::SQRT_2);
        assert_eq!(FixedI16::<13>::from_num(2).sqrt(), FixedI16::<13>::SQRT_2);
        assert!(
            FixedI16::<14>::MAX.sqrt() == FixedI16::<14>::SQRT_2 - FixedI16::<14>::DELTA
                || FixedI16::<14>::MAX.sqrt() == FixedI16::<14>::SQRT_2
        );
    }

    #[test]
    fn check_two_32() {
        assert_eq!(FixedU32::<0>::from_num(2).sqrt(), FixedU32::<0>::SQRT_2);
        assert_eq!(FixedU32::<1>::from_num(2).sqrt(), FixedU32::<1>::SQRT_2);
        assert_eq!(FixedU32::<15>::from_num(2).sqrt(), FixedU32::<15>::SQRT_2);
        assert_eq!(FixedU32::<16>::from_num(2).sqrt(), FixedU32::<16>::SQRT_2);
        assert_eq!(FixedU32::<17>::from_num(2).sqrt(), FixedU32::<17>::SQRT_2);
        assert_eq!(FixedU32::<29>::from_num(2).sqrt(), FixedU32::<29>::SQRT_2);
        assert_eq!(FixedU32::<30>::from_num(2).sqrt(), FixedU32::<30>::SQRT_2);
        assert!(
            FixedU32::<31>::MAX.sqrt() == FixedU32::<31>::SQRT_2 - FixedU32::<31>::DELTA
                || FixedU32::<31>::MAX.sqrt() == FixedU32::<31>::SQRT_2
        );

        assert_eq!(FixedI32::<0>::from_num(2).sqrt(), FixedI32::<0>::SQRT_2);
        assert_eq!(FixedI32::<1>::from_num(2).sqrt(), FixedI32::<1>::SQRT_2);
        assert_eq!(FixedI32::<15>::from_num(2).sqrt(), FixedI32::<15>::SQRT_2);
        assert_eq!(FixedI32::<16>::from_num(2).sqrt(), FixedI32::<16>::SQRT_2);
        assert_eq!(FixedI32::<17>::from_num(2).sqrt(), FixedI32::<17>::SQRT_2);
        assert_eq!(FixedI32::<29>::from_num(2).sqrt(), FixedI32::<29>::SQRT_2);
        assert!(
            FixedI32::<30>::MAX.sqrt() == FixedI32::<30>::SQRT_2 - FixedI32::<30>::DELTA
                || FixedI32::<30>::MAX.sqrt() == FixedI32::<30>::SQRT_2
        );
    }

    #[test]
    fn check_two_64() {
        assert_eq!(FixedU64::<0>::from_num(2).sqrt(), FixedU64::<0>::SQRT_2);
        assert_eq!(FixedU64::<1>::from_num(2).sqrt(), FixedU64::<1>::SQRT_2);
        assert_eq!(FixedU64::<31>::from_num(2).sqrt(), FixedU64::<31>::SQRT_2);
        assert_eq!(FixedU64::<32>::from_num(2).sqrt(), FixedU64::<32>::SQRT_2);
        assert_eq!(FixedU64::<33>::from_num(2).sqrt(), FixedU64::<33>::SQRT_2);
        assert_eq!(FixedU64::<61>::from_num(2).sqrt(), FixedU64::<61>::SQRT_2);
        assert_eq!(FixedU64::<62>::from_num(2).sqrt(), FixedU64::<62>::SQRT_2);
        assert!(
            FixedU64::<63>::MAX.sqrt() == FixedU64::<63>::SQRT_2 - FixedU64::<63>::DELTA
                || FixedU64::<63>::MAX.sqrt() == FixedU64::<63>::SQRT_2
        );

        assert_eq!(FixedI64::<0>::from_num(2).sqrt(), FixedI64::<0>::SQRT_2);
        assert_eq!(FixedI64::<1>::from_num(2).sqrt(), FixedI64::<1>::SQRT_2);
        assert_eq!(FixedI64::<31>::from_num(2).sqrt(), FixedI64::<31>::SQRT_2);
        assert_eq!(FixedI64::<32>::from_num(2).sqrt(), FixedI64::<32>::SQRT_2);
        assert_eq!(FixedI64::<33>::from_num(2).sqrt(), FixedI64::<33>::SQRT_2);
        assert_eq!(FixedI64::<61>::from_num(2).sqrt(), FixedI64::<61>::SQRT_2);
        assert!(
            FixedI64::<62>::MAX.sqrt() == FixedI64::<62>::SQRT_2 - FixedI64::<62>::DELTA
                || FixedI64::<62>::MAX.sqrt() == FixedI64::<62>::SQRT_2
        );
    }

    #[test]
    fn check_two_128() {
        assert_eq!(FixedU128::<0>::from_num(2).sqrt(), FixedU128::<0>::SQRT_2);
        assert_eq!(FixedU128::<1>::from_num(2).sqrt(), FixedU128::<1>::SQRT_2);
        assert_eq!(FixedU128::<63>::from_num(2).sqrt(), FixedU128::<63>::SQRT_2);
        assert_eq!(FixedU128::<64>::from_num(2).sqrt(), FixedU128::<64>::SQRT_2);
        assert_eq!(FixedU128::<65>::from_num(2).sqrt(), FixedU128::<65>::SQRT_2);
        assert_eq!(
            FixedU128::<125>::from_num(2).sqrt(),
            FixedU128::<125>::SQRT_2
        );
        assert_eq!(
            FixedU128::<126>::from_num(2).sqrt(),
            FixedU128::<126>::SQRT_2
        );
        assert!(
            FixedU128::<127>::MAX.sqrt() == FixedU128::<127>::SQRT_2 - FixedU128::<127>::DELTA
                || FixedU128::<127>::MAX.sqrt() == FixedU128::<127>::SQRT_2
        );

        assert_eq!(FixedI128::<0>::from_num(2).sqrt(), FixedI128::<0>::SQRT_2);
        assert_eq!(FixedI128::<1>::from_num(2).sqrt(), FixedI128::<1>::SQRT_2);
        assert_eq!(FixedI128::<63>::from_num(2).sqrt(), FixedI128::<63>::SQRT_2);
        assert_eq!(FixedI128::<64>::from_num(2).sqrt(), FixedI128::<64>::SQRT_2);
        assert_eq!(FixedI128::<65>::from_num(2).sqrt(), FixedI128::<65>::SQRT_2);
        assert_eq!(
            FixedI128::<125>::from_num(2).sqrt(),
            FixedI128::<125>::SQRT_2
        );
        assert!(
            FixedI128::<126>::MAX.sqrt() == FixedI128::<126>::SQRT_2 - FixedI128::<126>::DELTA
                || FixedI128::<126>::MAX.sqrt() == FixedI128::<126>::SQRT_2
        );
    }
}
