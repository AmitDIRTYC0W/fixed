// Copyright © 2018–2020 Trevor Spiteri

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

/*!
# Fixed-point numbers

The [*fixed* crate] provides fixed-point numbers.

  * [`FixedI8`] and [`FixedU8`] are eight-bit fixed-point numbers.
  * [`FixedI16`] and [`FixedU16`] are 16-bit fixed-point numbers.
  * [`FixedI32`] and [`FixedU32`] are 32-bit fixed-point numbers.
  * [`FixedI64`] and [`FixedU64`] are 64-bit fixed-point numbers.
  * [`FixedI128`] and [`FixedU128`] are 128-bit fixed-point numbers.

These types can have `Frac` fractional bits, where
0 ≤ `Frac` ≤ <i>n</i> and <i>n</i> is the total number of bits. When
`Frac` = 0, the fixed-point number behaves like an <i>n</i>-bit
integer. When `Frac` = <i>n</i>, the value <i>x</i> lies in the range
−0.5 ≤ <i>x</i> < 0.5 for signed numbers, and in the range
0 ≤ <i>x</i> < 1 for unsigned numbers.

In version 1 the [*typenum* crate] is used for the fractional bit
count `Frac`; the plan is to to have a major version 2 with [const
generics] when they are supported by the Rust compiler.

The main features are

  * Representation of fixed-point numbers up to 128 bits wide.
  * Conversions between fixed-point numbers and numeric primitives.
  * Comparisons between fixed-point numbers and numeric primitives.
  * Parsing from strings in decimal, binary, octal and hexadecimal.
  * Display as decimal, binary, octal and hexadecimal.
  * Arithmetic and logic operations.

This crate does *not* provide general analytic functions.

  * No algebraic functions are provided, for example no `sqrt` or
    `pow`.
  * No trigonometric functions are provided, for example no `sin` or
    `cos`.
  * No other transcendental functions are provided, for example no
    `log` or `exp`.

These functions are not provided because different implementations can
have different trade-offs, for example trading some correctness for
speed. Implementations can be provided in other crates.

  * The [*fixed-sqrt* crate] provides the square root operation.

The conversions supported cover the following cases.

  * Infallible lossless conversions between fixed-point numbers and
    numeric primitives are provided using [`From`] and [`Into`]. These
    never fail (infallible) and do not lose any bits (lossless).
  * Infallible lossy conversions between fixed-point numbers and
    numeric primitives are provided using the [`LossyFrom`] and
    [`LossyInto`] traits. The source can have more fractional bits
    than the destination.
  * Checked lossless conversions between fixed-point numbers and
    numeric primitives are provided using the [`LosslessTryFrom`] and
    [`LosslessTryInto`] traits. The source cannot have more fractional
    bits than the destination.
  * Checked conversions between fixed-point numbers and numeric
    primitives are provided using the [`FromFixed`] and [`ToFixed`]
    traits, or using the [`from_num`] and [`to_num`] methods and
    [their checked versions][`checked_from_num`].
  * Fixed-point numbers can be parsed from decimal strings using
    [`FromStr`], and from binary, octal and hexadecimal strings using
    the [`from_str_binary`], [`from_str_octal`] and [`from_str_hex`]
    methods. The result is rounded to the nearest, with ties rounded
    to even.
  * Fixed-point numbers can be converted to strings using [`Display`],
    [`Binary`], [`Octal`], [`LowerHex`] and [`UpperHex`]. The output
    is rounded to the nearest, with ties rounded to even.

## Quick examples

```rust
use fixed::types::I20F12;

// 19/3 = 6 1/3
let six_and_third = I20F12::from_num(19) / 3;
// four decimal digits for 12 binary digits
assert_eq!(six_and_third.to_string(), "6.3333");
// find the ceil and convert to i32
assert_eq!(six_and_third.ceil().to_num::<i32>(), 7);
// we can also compare directly to integers
assert_eq!(six_and_third.ceil(), 7);
```

The type [`I20F12`] is a 32-bit fixed-point signed number with 20
integer bits and 12 fractional bits. It is an alias to
<code>[FixedI32][`FixedI32`]&lt;[U12][`U12`]&gt;</code>. The unsigned
counterpart would be [`U20F12`]. Aliases are provided for all
combinations of integer and fractional bits adding up to a total of
eight, 16, 32, 64 or 128 bits.

```rust
use fixed::types::{I4F4, I4F12};

// −8 ≤ I4F4 < 8 with steps of 1/16 (~0.06)
let a = I4F4::from_num(1);
// multiplication and division by integers are possible
let ans1 = a / 5 * 17;
// 1 / 5 × 17 = 3 2/5 (3.4), but we get 3 3/16 (~3.2)
assert_eq!(ans1, I4F4::from_bits((3 << 4) + 3));
assert_eq!(ans1.to_string(), "3.2");

// −8 ≤ I4F12 < 8 with steps of 1/4096 (~0.0002)
let wider_a = I4F12::from(a);
let wider_ans = wider_a / 5 * 17;
let ans2 = I4F4::from_num(wider_ans);
// now the answer is the much closer 3 6/16 (~3.4)
assert_eq!(ans2, I4F4::from_bits((3 << 4) + 6));
assert_eq!(ans2.to_string(), "3.4");
```

The second example shows some precision and conversion issues. The low
precision of `a` means that `a / 5` is 3⁄16 instead of 1⁄5, leading to
an inaccurate result `ans1` = 3 3⁄16 (~3.2). With a higher precision,
we get `wider_a / 5` equal to 819⁄4096, leading to a more accurate
intermediate result `wider_ans` = 3 1635⁄4096. When we convert back to
four fractional bits, we get `ans2` = 3 6⁄16 (~3.4).

Note that we can convert from [`I4F4`] to [`I4F12`] using [`From`], as
the target type has the same number of integer bits and a larger
number of fractional bits. Converting from [`I4F12`] to [`I4F4`]
cannot use [`From`] as we have less fractional bits, so we use
[`from_num`] instead.

## Writing fixed-point constants and values literally

The [*fixed-macro* crate] provides a convenient macro to write down
fixed-point constants literally in the code.

```rust
# #[cfg(feature = "add-fixed-macro-dev-dependency-on-rustc-version-bump")] {
use fixed::types::I16F16;
use fixed_macro::fixed;

const NUM1: I16F16 = fixed!(12.75: I16F16);
let num2 = NUM1 + fixed!(13.125: I16F16);
assert_eq!(num2, 25.875);
# }
```

## Using the *fixed* crate

The *fixed* crate is available on [crates.io][*fixed* crate]. To use
it in your crate, add it as a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
fixed = "1.3"
```

The *fixed* crate requires rustc version 1.44.0 or later.

## Optional features

The *fixed* crate has four optional feature:

 1. `az`, disabled by default. This implements the cast traits
    provided by the [*az* crate].
 2. `f16`, disabled by default. This provides conversion to/from
    [`f16`] and [`bf16`]. This features requires the [*half* crate].
 3. `serde`, disabled by default. This provides serialization support
    for the fixed-point types. This feature requires the
    [*serde* crate].
 4. `std`, disabled by default. This is for features that are not
    possible under `no_std`: currently the implementation of the
    [`Error`] trait for [`ParseFixedError`].

To enable features, you can add the dependency like this to
[*Cargo.toml*]:

```toml
[dependencies.fixed]
version = "1.3"
features = ["f16", "serde"]
```

## Experimental optional features

It is not considered a breaking change if experimental features are
removed. The removal of experimental features would however require a
minor version bump. Similarly, on a minor version bump, optional
dependencies can be updated to an incompatible newer version.

There are three experimental feature:

 1. `num-traits`, disabled by default. This implements some traits
    from the [*num-traits* crate]. (The plan is to upgrade this to an
    optional feature once *num-traits* reaches version 1.0.0.)
 2. `unwrapped`, disabled by default. This adds methods for arithmetic
    that panic on overflow even when debug assertions are disabled,
    similar to how wrapping methods will wrap even when debug
    assertions are enabled. (The plan is to always enable this
    functionality but remove the experimental feature in version
    1.5.0.)
 3. `serde-str`, disabled by default. Fixed-point numbers are
    serialized as strings showing the value when using human-readable
    formats. This feature requires the `serde` and the `std` optional
    features. (The plan is to upgrade this to an optional feature in
    version 1.5.0.) **Warning:** numbers serialized when this feature
    is enabled cannot be deserialized when this feature is disabled.

## License

This crate is free software: you can redistribute it and/or modify it
under the terms of either

  * the [Apache License, Version 2.0][LICENSE-APACHE] or
  * the [MIT License][LICENSE-MIT]

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache
License, Version 2.0, shall be dual licensed as above, without any
additional terms or conditions.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*az* crate]: https://crates.io/crates/az
[*fixed* crate]: https://crates.io/crates/fixed
[*fixed-macro* crate]: https://crates.io/crates/fixed-macro
[*fixed-sqrt* crate]: https://crates.io/crates/fixed-sqrt
[*half* crate]: https://crates.io/crates/half
[*num-traits* crate]: https://crates.io/crates/num-traits
[*serde* crate]: https://crates.io/crates/serde
[*typenum* crate]: https://crates.io/crates/typenum
[LICENSE-APACHE]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: https://opensource.org/licenses/MIT
[`Binary`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html
[`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`FixedI128`]: struct.FixedI128.html
[`FixedI16`]: struct.FixedI16.html
[`FixedI32`]: struct.FixedI32.html
[`FixedI64`]: struct.FixedI64.html
[`FixedI8`]: struct.FixedI8.html
[`FixedU128`]: struct.FixedU128.html
[`FixedU16`]: struct.FixedU16.html
[`FixedU32`]: struct.FixedU32.html
[`FixedU64`]: struct.FixedU64.html
[`FixedU8`]: struct.FixedU8.html
[`FromFixed`]: traits/trait.FromFixed.html
[`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`I20F12`]: types/type.I20F12.html
[`I4F12`]: types/type.I4F12.html
[`I4F4`]: types/type.I4F4.html
[`Into`]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
[`LosslessTryFrom`]: traits/trait.LosslessTryFrom.html
[`LosslessTryInto`]: traits/trait.LosslessTryInto.html
[`LossyFrom`]: traits/trait.LossyFrom.html
[`LossyInto`]: traits/trait.LossyInto.html
[`LowerHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html
[`Octal`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html
[`ParseFixedError`]: struct.ParseFixedError.html
[`ToFixed`]: traits/trait.ToFixed.html
[`U12`]: types/extra/type.U12.html
[`U20F12`]: types/type.U20F12.html
[`UpperHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html
[`bf16`]: https://docs.rs/half/^1/half/struct.bf16.html
[`checked_from_num`]: struct.FixedI32.html#method.checked_from_num
[`f16`]: https://docs.rs/half/^1/half/struct.f16.html
[`from_num`]: struct.FixedI32.html#method.from_num
[`from_str_binary`]: struct.FixedI32.html#method.from_str_binary
[`from_str_hex`]: struct.FixedI32.html#method.from_str_hex
[`from_str_octal`]: struct.FixedI32.html#method.from_str_octal
[`to_num`]: struct.FixedI32.html#method.to_num
[const generics]: https://github.com/rust-lang/rust/issues/44580
*/
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/fixed/~1.3")]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

#[cfg(all(not(feature = "std"), test))]
extern crate std;

#[macro_use]
mod macros;

mod arith;
#[cfg(feature = "az")]
mod cast;
mod cmp;
pub mod consts;
mod convert;
mod display;
mod float_helper;
mod from_str;
mod helpers;
#[cfg(feature = "num-traits")]
mod impl_num_traits;
mod int_helper;
mod log10;
#[cfg(feature = "serde")]
mod serdeize;
pub mod traits;
pub mod types;
#[cfg(feature = "unwrapped")]
mod unwrapped;
mod wide_div;
mod wrapping;

#[cfg(feature = "unwrapped")]
pub use crate::unwrapped::Unwrapped;
use crate::{
    arith::MulDivOverflow,
    from_str::FromStrRadix,
    log10::IntFracLog10,
    traits::{FromFixed, ToFixed},
    types::extra::{
        IsLessOrEqual, LeEqU128, LeEqU16, LeEqU32, LeEqU64, LeEqU8, True, Unsigned, U12, U124,
        U125, U126, U127, U128, U13, U14, U15, U16, U28, U29, U30, U31, U32, U4, U5, U6, U60, U61,
        U62, U63, U64, U7, U8,
    },
};
pub use crate::{from_str::ParseFixedError, wrapping::Wrapping};
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
};

/// A prelude to import useful traits.
///
/// This prelude is similar to the
/// [standard library’s prelude][prelude] in that you’ll almost always
/// want to import its entire contents, but unlike the standard
/// library’s prelude you’ll have to do so manually:
///
/// ```
/// # #[allow(unused_imports)]
/// use fixed::prelude::*;
/// ```
///
/// The prelude may grow over time as additional items see ubiquitous use.
///
/// [prelude]: https://doc.rust-lang.org/nightly/std/prelude/index.html
pub mod prelude {
    pub use crate::traits::{
        FromFixed, LosslessTryFrom, LosslessTryInto, LossyFrom, LossyInto, ToFixed,
    };
}

#[macro_use]
mod macros_from_to;
#[macro_use]
mod macros_round;
#[macro_use]
mod macros_no_frac;
#[macro_use]
mod macros_frac;
#[macro_use]
mod macros_const;

macro_rules! fixed {
    (
        $description:expr,
        $Fixed:ident(
            $Inner:ty, $LeEqU:tt, $s_nbits:expr,
            $s_nbits_m1:expr, $s_nbits_m2:expr, $s_nbits_m3:expr, $s_nbits_m4:expr
        ),
        $nbytes:expr, $bytes_val:expr, $be_bytes:expr, $le_bytes:expr,
        $UInner:ty, $Signedness:tt,
        $LeEqU_C0:tt, $LeEqU_C1:tt, $LeEqU_C2:tt, $LeEqU_C3:tt
    ) => {
        fixed! {
            $description,
            $Fixed[stringify!($Fixed)](
                $Inner[stringify!($Inner)], $LeEqU, $s_nbits,
                $s_nbits_m1, $s_nbits_m2, $s_nbits_m3, $s_nbits_m4
            ),
            $nbytes, $bytes_val, $be_bytes, $le_bytes,
            $UInner, $Signedness,
            $LeEqU_C0, $LeEqU_C1, $LeEqU_C2, $LeEqU_C3
        }
    };
    (
        $description:expr,
        $Fixed:ident[$s_fixed:expr](
            $Inner:ty[$s_inner:expr], $LeEqU:tt, $s_nbits:expr,
            $s_nbits_m1:expr, $s_nbits_m2:expr, $s_nbits_m3:expr, $s_nbits_m4:expr
        ),
        $nbytes:expr, $bytes_val:expr, $be_bytes:expr, $le_bytes:expr,
        $UInner:ty, $Signedness:tt,
        $LeEqU_C0:tt, $LeEqU_C1:tt, $LeEqU_C2:tt, $LeEqU_C3:tt
    ) => {
        comment! {
            $description,
            " number with `Frac` fractional bits.

`Frac` is an [`Unsigned`] as provided by the [*typenum* crate]; the
plan is to move to [const generics] in version 2 when they are
supported by the Rust compiler.

# Examples

```rust
use fixed::{types::extra::U3, ", $s_fixed, "};
let eleven = ", $s_fixed, "::<U3>::from_num(11);
assert_eq!(eleven, ", $s_fixed, "::<U3>::from_bits(11 << 3));
assert_eq!(eleven, 11);
assert_eq!(eleven.to_string(), \"11\");
let two_point_75 = eleven / 4;
assert_eq!(two_point_75, ", $s_fixed, "::<U3>::from_bits(11 << 1));
assert_eq!(two_point_75, 2.75);
assert_eq!(two_point_75.to_string(), \"2.8\");
```

[*typenum* crate]: https://crates.io/crates/typenum
[`Unsigned`]: https://docs.rs/typenum/^1.3/typenum/marker_traits/trait.Unsigned.html
[const generics]: https://github.com/rust-lang/rust/issues/44580
";
            #[repr(transparent)]
            pub struct $Fixed<Frac> {
                bits: $Inner,
                phantom: PhantomData<Frac>,
            }
        }

        impl<Frac> Clone for $Fixed<Frac> {
            #[inline]
            fn clone(&self) -> $Fixed<Frac> {
                $Fixed {
                    bits: self.bits,
                    phantom: PhantomData,
                }
            }
        }

        impl<Frac> Copy for $Fixed<Frac> {}

        impl<Frac> Default for $Fixed<Frac> {
            #[inline]
            fn default() -> Self {
                $Fixed {
                    bits: Default::default(),
                    phantom: PhantomData,
                }
            }
        }

        impl<Frac> Hash for $Fixed<Frac> {
            #[inline]
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.bits.hash(state);
            }
        }

        // inherent methods that do not require Frac bounds, some of which can thus be const
        fixed_no_frac! {
            $Fixed[$s_fixed]($Inner[$s_inner], $LeEqU, $s_nbits),
            $nbytes, $bytes_val, $be_bytes, $le_bytes,
            $UInner, $Signedness
        }
        // inherent methods that require Frac bounds, and cannot be const
        fixed_frac! {
            $Fixed[$s_fixed]($Inner[$s_inner], $LeEqU, $s_nbits, $s_nbits_m1, $s_nbits_m4),
            $UInner, $Signedness
        }
        fixed_const! {
            $Fixed[$s_fixed]($LeEqU, $s_nbits, $s_nbits_m1, $s_nbits_m2, $s_nbits_m3, $s_nbits_m4),
            $LeEqU_C0, $LeEqU_C1, $LeEqU_C2, $LeEqU_C3,
            $Signedness
        }
    };
}

fixed! {
    "An eight-bit fixed-point unsigned",
    FixedU8(u8, LeEqU8, "8", "7", "6", "5", "4"),
    1, "0x12", "[0x12]", "[0x12]",
    u8, Unsigned,
    U8, U7, U6, U5
}
fixed! {
    "A 16-bit fixed-point unsigned",
    FixedU16(u16, LeEqU16, "16", "15", "14", "13", "12"),
    2, "0x1234", "[0x12, 0x34]", "[0x34, 0x12]",
    u16, Unsigned,
    U16, U15, U14, U13
}
fixed! {
    "A 32-bit fixed-point unsigned",
    FixedU32(u32, LeEqU32, "32", "31", "30", "29", "28"),
    4, "0x1234_5678", "[0x12, 0x34, 0x56, 0x78]", "[0x78, 0x56, 0x34, 0x12]",
    u32, Unsigned,
    U32, U31, U30, U29
}
fixed! {
    "A 64-bit fixed-point unsigned",
    FixedU64(u64, LeEqU64, "64", "63", "62", "61", "60"),
    8, "0x1234_5678_9ABC_DEF0",
    "[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]",
    "[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]",
    u64, Unsigned,
    U64, U63, U62, U61
}
fixed! {
    "A 128-bit fixed-point unsigned",
    FixedU128(u128, LeEqU128, "128", "127", "126", "125", "124"),
    16, "0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0",
    "[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, \
     0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]",
    "[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, \
     0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]",
    u128, Unsigned,
    U128, U127, U126, U125
}
fixed! {
    "An eight-bit fixed-point signed",
    FixedI8(i8, LeEqU8, "8", "7", "6", "5", "4"),
    1, "0x12", "[0x12]", "[0x12]",
    u8, Signed,
    U7, U6, U5, U4
}
fixed! {
    "A 16-bit fixed-point signed",
    FixedI16(i16, LeEqU16, "16", "15", "14", "13", "12"),
    2, "0x1234", "[0x12, 0x34]", "[0x34, 0x12]",
    u16, Signed,
    U15, U14, U13, U12
}
fixed! {
    "A 32-bit fixed-point signed",
    FixedI32(i32, LeEqU32, "32", "31", "30", "29", "28"),
    4, "0x1234_5678", "[0x12, 0x34, 0x56, 0x78]", "[0x78, 0x56, 0x34, 0x12]",
    u32, Signed,
    U31, U30, U29, U28
}
fixed! {
    "A 64-bit fixed-point signed",
    FixedI64(i64, LeEqU64, "64", "63", "62", "61", "60"),
    8, "0x1234_5678_9ABC_DEF0",
    "[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]",
    "[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]",
    u64, Signed,
    U63, U62, U61, U60
}
fixed! {
    "A 128-bit fixed-point signed",
    FixedI128(i128, LeEqU128, "128", "127", "126", "125", "124"),
    16, "0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0",
    "[0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0, \
     0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]",
    "[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12, \
     0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]",
    u128, Signed,
    U127, U126, U125, U124
}

/// Defines constant fixed-point numbers from integer expressions.
///
/// This macro is useful because [`from_num`] cannot be used in
/// constant expressions.
///
/// # Alternative
///
/// The [*fixed-macro* crate] provides a convenient macro to write
/// down fixed-point constants literally in code which has two
/// advantages over this macro:
///
///  1. It can handle fixed-point numbers with fractions, not just
///     integers.
///  2. It can be used anywhere an expression or constant expression
///     can be used, not just to define a constant.
///
/// # Examples
///
/// ```rust
/// use fixed::{const_fixed_from_int, types::I16F16};
/// const_fixed_from_int! {
///     // define a constant using an integer
///     const FIVE: I16F16 = 5;
///     // define a constant using an integer expression
///     const SUM: I16F16 = 3 + 2;
/// }
/// assert_eq!(FIVE, 5);
/// assert_eq!(SUM, 5);
/// ```
///
/// The following would fail to compile because
/// <code>[i32][`i32`]::[MAX][`i32::MAX`]</code> is not representable
/// by [`I16F16`].
///
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::I16F16};
/// const_fixed_from_int! {
///     // fails because i32::MAX > I16F16::MAX
///     const _OVERFLOW: I16F16 = i32::MAX;
/// }
/// ```
///
/// The following would fail to compile because [`I16F16`] is an alias
/// for <code>[FixedI32][`FixedI32`]&lt;[U32][`U32`]&gt;</code>, and
/// this macro can define [`FixedI32`] constants using [`i32`]
/// expressions, not [`i16`] expressions.
///
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::I16F16};
/// const_fixed_from_int! {
///     // fails because 0i16 is not of type i32
///     const _MISMATCH: I16F16 = 0i16;
/// }
/// ```
///
/// [*fixed-macro* crate]: https://crates.io/crates/fixed-macro
/// [`FixedI32`]: struct.FixedI32.html
/// [`I16F16`]: types/type.I16F16.html
/// [`U32`]: types/extra/type.U32.html
/// [`from_num`]: struct.FixedI32.html#method.from_num
/// [`i16`]: https://doc.rust-lang.org/nightly/core/i16/index.html
/// [`i32::MAX`]: https://doc.rust-lang.org/nightly/core/i32/constant.MAX.html
/// [`i32`]: https://doc.rust-lang.org/nightly/core/i32/index.html
#[macro_export]
macro_rules! const_fixed_from_int {
    ($(const $NAME:ident: $Fixed:ty = $int:expr;)*) => { $(
        const $NAME: $Fixed = {
            const FRAC: u32 = <$Fixed>::FRAC_NBITS;
            // Use $Fixed as type because there isn't a const way to use the inner type.
            const INT: $Fixed = <$Fixed>::from_bits($int);
            // LSB.to_bits() has a specific type, unlike the literal `1`.
            const LSB: $Fixed = <$Fixed>::from_bits(1);
            // Divide shift into two parts for cases where $Fixed cannot represent 1.
            const ONE_A: $Fixed = <$Fixed>::from_bits(LSB.to_bits() << FRAC / 2);
            const ONE_B: $Fixed = <$Fixed>::from_bits(LSB.to_bits() << FRAC - FRAC / 2);
            <$Fixed>::from_bits(INT.to_bits() * ONE_A.to_bits() * ONE_B.to_bits())
        };
    )* };
}

/// These are doc tests that should not appear in the docs, but are
/// useful as doc tests can check to ensure compilation failure.
///
/// The first snippet succeeds, and acts as a control.
///
/// ```rust
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const ZERO_I0: I0F32 = 0;
///     const ZERO_I1: I32F0 = 0;
///     const ZERO_U0: U0F32 = 0;
///     const ZERO_U1: U32F0 = 0;
///
///     const ONE_I0: I2F30 = 1;
///     const ONE_I1: I32F0 = 1;
///     const ONE_U0: U1F31 = 1;
///     const ONE_U1: U32F0 = 1;
///
///     const MINUS_ONE_I0: I1F31 = -1;
///     const MINUS_ONE_I1: I32F0 = -1;
///
///     const MINUS_TWO_I0: I2F30 = -2;
///     const MINUS_TWO_I1: I32F0 = -2;
/// }
/// assert_eq!(ZERO_I0, 0);
/// assert_eq!(ZERO_I1, 0);
/// assert_eq!(ZERO_U0, 0);
/// assert_eq!(ZERO_U1, 0);
///
/// assert_eq!(ONE_I0, 1);
/// assert_eq!(ONE_I1, 1);
/// assert_eq!(ONE_U0, 1);
/// assert_eq!(ONE_U1, 1);
///
/// assert_eq!(MINUS_ONE_I0, -1);
/// assert_eq!(MINUS_ONE_I1, -1);
///
/// assert_eq!(MINUS_TWO_I0, -2);
/// assert_eq!(MINUS_TWO_I1, -2);
/// ```
///
/// The rest of the tests should all fail compilation.
///
/// Not enough integer bits for 1.
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const _ONE: I0F32 = 1;
/// }
/// ```
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const _ONE: I1F31 = 1;
/// }
/// ```
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const _ONE: U0F32 = 1;
/// }
/// ```
///
/// Not enough integer bits for -1.
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const _MINUS_ONE: I0F32 = -1;
/// }
/// ```
///
/// Not enough integer bits for -2.
/// ```compile_fail
/// use fixed::{const_fixed_from_int, types::*};
/// const_fixed_from_int! {
///     const _MINUS_TWO: I1F31 = -2;
/// }
/// ```
fn _compile_fail_tests() {}

#[cfg(test)]
#[allow(clippy::cognitive_complexity)]
mod tests {
    use crate::types::{I0F32, I16F16, I1F31, U0F32, U16F16};

    #[test]
    fn rounding_signed() {
        // -0.5
        let f = I0F32::from_bits(-1 << 31);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I0F32::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I0F32::from_num(0), true));
        assert_eq!(f.overflowing_round(), (I0F32::from_num(0), true));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I0F32::from_num(0), false)
        );

        // -0.5 + Δ
        let f = I0F32::from_bits((-1 << 31) + 1);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I0F32::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I0F32::from_num(0), true));
        assert_eq!(f.overflowing_round(), (I0F32::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I0F32::from_num(0), false)
        );

        // 0
        let f = I0F32::from_bits(0);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I0F32::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I0F32::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I0F32::from_num(0), false)
        );

        // 0.5 - Δ
        let f = I0F32::from_bits((1 << 30) - 1 + (1 << 30));
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I0F32::from_num(0), true));
        assert_eq!(f.overflowing_floor(), (I0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I0F32::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I0F32::from_num(0), false)
        );

        // -1
        let f = I1F31::from_bits((-1) << 31);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), -1);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(-1), false));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(-1), false)
        );

        // -0.5 - Δ
        let f = I1F31::from_bits(((-1) << 30) - 1);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(-1), false)
        );

        // -0.5
        let f = I1F31::from_bits((-1) << 30);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(0), false)
        );

        // -0.5 + Δ
        let f = I1F31::from_bits(((-1) << 30) + 1);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(0), false)
        );

        // 0.5 - Δ
        let f = I1F31::from_bits((1 << 30) - 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(-1), true));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(0), false)
        );

        // 0.5
        let f = I1F31::from_bits(1 << 30);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(-1), true));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(-1), true));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(0), false)
        );

        // 0
        let f = I1F31::from_bits(0);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(0), false)
        );

        // 0.5 + Δ
        let f = I1F31::from_bits((1 << 30) + 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I1F31::from_num(-1), true));
        assert_eq!(f.overflowing_floor(), (I1F31::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I1F31::from_num(-1), true));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I1F31::from_num(-1), true)
        );

        // -3.5 - Δ
        let f = I16F16::from_bits(((-7) << 15) - 1);
        assert_eq!(f.to_num::<i32>(), -4);
        assert_eq!(f.round_to_zero(), -3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-4), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-4), false)
        );

        // -3.5
        let f = I16F16::from_bits((-7) << 15);
        assert_eq!(f.to_num::<i32>(), -4);
        assert_eq!(f.round_to_zero(), -3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-4), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-4), false)
        );

        // -3.5 + Δ
        let f = I16F16::from_bits(((-7) << 15) + 1);
        assert_eq!(f.to_num::<i32>(), -4);
        assert_eq!(f.round_to_zero(), -3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-4), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-3), false)
        );

        // -2.5 - Δ
        let f = I16F16::from_bits(((-5) << 15) - 1);
        assert_eq!(f.to_num::<i32>(), -3);
        assert_eq!(f.round_to_zero(), -2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-2), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-3), false)
        );

        // -2.5
        let f = I16F16::from_bits((-5) << 15);
        assert_eq!(f.to_num::<i32>(), -3);
        assert_eq!(f.round_to_zero(), -2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-2), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-2), false)
        );

        // -2.5 + Δ
        let f = I16F16::from_bits(((-5) << 15) + 1);
        assert_eq!(f.to_num::<i32>(), -3);
        assert_eq!(f.round_to_zero(), -2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-2), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-2), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-2), false)
        );

        // -1
        let f = I16F16::from_bits((-1) << 16);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), -1);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(-1), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-1), false)
        );

        // -0.5 - Δ
        let f = I16F16::from_bits(((-1) << 15) - 1);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(-1), false)
        );

        // -0.5
        let f = I16F16::from_bits((-1) << 15);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(-1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(0), false)
        );

        // -0.5 + Δ
        let f = I16F16::from_bits(((-1) << 15) + 1);
        assert_eq!(f.to_num::<i32>(), -1);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(-1), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(0), false)
        );

        // 0
        let f = I16F16::from_bits(0);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(0), false)
        );

        // 0.5 - Δ
        let f = I16F16::from_bits((1 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(0), false)
        );

        // 0.5
        let f = I16F16::from_bits(1 << 15);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(0), false)
        );

        // 0.5 + Δ
        let f = I16F16::from_bits((1 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(1), false)
        );

        // 1
        let f = I16F16::from_bits(1 << 16);
        assert_eq!(f.to_num::<i32>(), 1);
        assert_eq!(f.round_to_zero(), 1);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(1), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(1), false)
        );

        // 2.5 - Δ
        let f = I16F16::from_bits((5 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(2), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(2), false)
        );

        // 2.5
        let f = I16F16::from_bits(5 << 15);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(2), false)
        );

        // 2.5 + Δ
        let f = I16F16::from_bits((5 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(3), false)
        );

        // 3.5 - Δ
        let f = I16F16::from_bits((7 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(3), false)
        );

        // 3.5
        let f = I16F16::from_bits(7 << 15);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(4), false)
        );

        // 3.5 + Δ
        let f = I16F16::from_bits((7 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (I16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (I16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (I16F16::from_num(4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (I16F16::from_num(4), false)
        );
    }

    #[test]
    fn rounding_unsigned() {
        // 0
        let f = U0F32::from_bits(0);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U0F32::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (U0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U0F32::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U0F32::from_num(0), false)
        );

        // 0.5 - Δ
        let f = U0F32::from_bits((1 << 31) - 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U0F32::from_num(0), true));
        assert_eq!(f.overflowing_floor(), (U0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U0F32::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U0F32::from_num(0), false)
        );

        // 0.5
        let f = U0F32::from_bits(1 << 31);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U0F32::from_num(0), true));
        assert_eq!(f.overflowing_floor(), (U0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U0F32::from_num(0), true));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U0F32::from_num(0), false)
        );

        // 0.5 + Δ
        let f = U0F32::from_bits((1 << 31) + 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U0F32::from_num(0), true));
        assert_eq!(f.overflowing_floor(), (U0F32::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U0F32::from_num(0), true));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U0F32::from_num(0), true)
        );

        // 0
        let f = U16F16::from_bits(0);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(0), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(0), false)
        );

        // 0.5 - Δ
        let f = U16F16::from_bits((1 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(0), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(0), false)
        );

        // 0.5
        let f = U16F16::from_bits(1 << 15);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(0), false)
        );

        // 0.5 + Δ
        let f = U16F16::from_bits((1 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 0);
        assert_eq!(f.round_to_zero(), 0);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(0), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(1), false)
        );

        // 1
        let f = U16F16::from_bits(1 << 16);
        assert_eq!(f.to_num::<i32>(), 1);
        assert_eq!(f.round_to_zero(), 1);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(1), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(1), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(1), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(1), false)
        );

        // 2.5 - Δ
        let f = U16F16::from_bits((5 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(2), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(2), false)
        );

        // 2.5
        let f = U16F16::from_bits(5 << 15);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(2), false)
        );

        // 2.5 + Δ
        let f = U16F16::from_bits((5 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 2);
        assert_eq!(f.round_to_zero(), 2);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(2), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(3), false)
        );

        // 3.5 - Δ
        let f = U16F16::from_bits((7 << 15) - 1);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(3), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(3), false)
        );

        // 3.5
        let f = U16F16::from_bits(7 << 15);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(4), false)
        );

        // 3.5 + Δ
        let f = U16F16::from_bits((7 << 15) + 1);
        assert_eq!(f.to_num::<i32>(), 3);
        assert_eq!(f.round_to_zero(), 3);
        assert_eq!(f.overflowing_ceil(), (U16F16::from_num(4), false));
        assert_eq!(f.overflowing_floor(), (U16F16::from_num(3), false));
        assert_eq!(f.overflowing_round(), (U16F16::from_num(4), false));
        assert_eq!(
            f.overflowing_round_ties_to_even(),
            (U16F16::from_num(4), false)
        );
    }
}
