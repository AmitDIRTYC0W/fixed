<!-- Copyright © 2018–2022 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

# Fixed-point numbers

**Alpha:** This is an alpha release of the new major version 2.0.0 that makes
use of const generics instead of the [*typenum*
crate](https://crates.io/crate/typenum). This version requires the nightly
compiler with the [`generic_const_exprs` feature] enabled. The stable version
2.0.0 itself will not be released before the [`generic_const_exprs` feature] is
stabilized.

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560

The [*fixed* crate] provides fixed-point numbers.

  * [`FixedI8`] and [`FixedU8`] are eight-bit fixed-point numbers.
  * [`FixedI16`] and [`FixedU16`] are 16-bit fixed-point numbers.
  * [`FixedI32`] and [`FixedU32`] are 32-bit fixed-point numbers.
  * [`FixedI64`] and [`FixedU64`] are 64-bit fixed-point numbers.
  * [`FixedI128`] and [`FixedU128`] are 128-bit fixed-point numbers.

An <i>n</i>-bit fixed-point number has <i>f</i>&nbsp;=&nbsp;`FRAC` fractional
bits, and <i>n</i>&nbsp;&minus;&nbsp;<i>f</i> integer bits. For example,
<code>[FixedI32]\<24></code> is a 32-bit signed fixed-point number with
<i>n</i>&nbsp;=&nbsp;32 total bits, <i>f</i>&nbsp;=&nbsp;24 fractional bits, and
<i>n</i>&nbsp;&minus;&nbsp;<i>f</i>&nbsp;=&nbsp;8 integer bits.
<code>[FixedI32]\<0></code> behaves like [`i32`], and
<code>[FixedU32]\<0></code> behaves like [`u32`].

The difference between any two successive representable numbers is constant
throughout the possible range for a fixed-point number:
<i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>. When <i>f</i>&nbsp;=&nbsp;0, like
in <code>[FixedI32]\<0></code>, <i>Δ</i>&nbsp;=&nbsp;1 because representable
numbers are integers, and the difference between two successive integers is 1.
When <i>f</i>&nbsp;=&nbsp;<i>n</i>, <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>n</i></sup>
and the value lies in the range &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;0.5
for signed numbers like <code>[FixedI32]\<32></code>, and in the range
0&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1 for unsigned numbers like
<code>[FixedU32]\<32></code>.

The main features are

  * Representation of binary fixed-point numbers up to 128 bits wide.
  * Conversions between fixed-point numbers and numeric primitives.
  * Comparisons between fixed-point numbers and numeric primitives.
  * Parsing from strings in decimal, binary, octal and hexadecimal.
  * Display as decimal, binary, octal and hexadecimal.
  * Arithmetic and logic operations.

This crate does *not* provide decimal fixed-point numbers. For example 0.001
cannot be represented exactly, as it is 1/10<sup>3</sup>. It is binary fractions
like 1/2<sup>4</sup> (0.0625) that can be represented exactly, provided there
are enough fractional bits.

This crate does *not* provide general analytic functions.

  * No algebraic functions are provided, for example no `sqrt` or `pow`.
  * No trigonometric functions are provided, for example no `sin` or `cos`.
  * No other transcendental functions are provided, for example no `log` or
    `exp`.

These functions are not provided because different implementations can have
different trade-offs, for example trading some correctness for speed.
Implementations can be provided in other crates.

  * The [*fixed-sqrt* crate] provides the square root operation.
  * The [*cordic* crate] provides various functions implemented using the
    [CORDIC] algorithm.

The conversions supported cover the following cases.

  * Infallible lossless conversions between fixed-point numbers and numeric
    primitives are provided using [`From`] and [`Into`]. These never fail
    (infallible) and do not lose any bits (lossless).
  * Infallible lossy conversions between fixed-point numbers and numeric
    primitives are provided using the [`LossyFrom`] and [`LossyInto`] traits.
    The source can have more fractional bits than the destination.
  * Checked lossless conversions between fixed-point numbers and numeric
    primitives are provided using the [`LosslessTryFrom`] and
    [`LosslessTryInto`] traits. The source cannot have more fractional bits than
    the destination.
  * Checked conversions between fixed-point numbers and numeric primitives are
    provided using the [`FromFixed`] and [`ToFixed`] traits, or using the
    [`from_num`] and [`to_num`] methods and [their checked
    versions][`checked_from_num`].
  * Additionally, [`az`] casts are implemented for conversion between
    fixed-point nubmers and numeric primitives.
  * Fixed-point numbers can be parsed from decimal strings using [`FromStr`],
    and from binary, octal and hexadecimal strings using the
    [`from_str_binary`], [`from_str_octal`] and [`from_str_hex`] methods. The
    result is rounded to the nearest, with ties rounded to even.
  * Fixed-point numbers can be converted to strings using [`Display`],
    [`Binary`], [`Octal`], [`LowerHex`] and [`UpperHex`]. The output is rounded
    to the nearest, with ties rounded to even.
  * All fixed-point numbers are plain old data, so [`bytemuck`] bit casting
    conversions can be used.

## What’s new

### Version 2.0.0-alpha.2 news (unreleased)

  * The following methods are now `const` functions:
      * [`int`][f-i-2-0a2], [`frac`][f-fr-2-0a2], [`round_to_zero`][f-rtz-2-0a2]
      * [`ceil`][f-c-2-0a2], [`checked_ceil`][f-cc-2-0a2],
        [`saturating_ceil`][f-sc-2-0a2], [`wrapping_ceil`][f-wc-2-0a2],
        [`unwrapped_ceil`][f-uc-2-0a2], [`overflowing_ceil`][f-oc-2-0a2],
      * [`floor`][f-f-2-0a2], [`checked_floor`][f-cf-2-0a2],
        [`saturating_floor`][f-sf-2-0a2], [`wrapping_floor`][f-wf-2-0a2],
        [`unwrapped_floor`][f-uf-2-0a2], [`overflowing_floor`][f-of-2-0a2],
      * [`round`][f-r-2-0a2], [`checked_round`][f-cr-2-0a2],
        [`saturating_round`][f-sr-2-0a2], [`wrapping_round`][f-wr-2-0a2],
        [`unwrapped_round`][f-ur-2-0a2], [`overflowing_round`][f-or-2-0a2],
      * [`round_ties_to_even`][f-rtte-2-0a2],
        [`checked_round_ties_to_even`][f-crtte-2-0a2],
        [`saturating_round_ties_to_even`][f-srtte-2-0a2],
        [`wrapping_round_ties_to_even`][f-wrtte-2-0a2],
        [`unwrapped_round_ties_to_even`][f-urtte-2-0a2],
        [`overflowing_round_ties_to_even`][f-ortte-2-0a2],
      * [`int_log2`][f-il2-2-0a2], [`checked_int_log2`][f-cil2-2-0a2],
        [`int_log10`][f-il10-2-0a2], [`checked_int_log2`][f-cil10-2-0a2]
      * [`wide_mul`][f-wim-2-0a2], [`wide_div`][f-wd-2-0a2],
      * [`checked_mul`][f-cm-2-0a2], [`saturating_mul`][f-sm-2-0a2],
        [`wrapping_mul`][f-wm-2-0a2], [`unwrapped_mul`][f-um-2-0a2],
        [`overflowing_mul`][f-om-2-0a2],
      * [`mul_add`][f-ma-2-0a2], [`checked_mul_add`][f-cma-2-0a2],
        [`saturating_mul_add`][f-sma-2-0a2], [`wrapping_mul_add`][f-wma-2-0a2],
        [`unwrapped_mul_add`][f-uma-2-0a2],
        [`overflowing_mul_add`][f-oma-2-0a2],
      * [`signum`][f-s-2-0a2], [`checked_signum`][f-cs-2-0a2],
        [`saturating_signum`][f-ss-2-0a2], [`wrapping_signum`][f-ws-2-0a2],
        [`unwrapped_signum`][f-us-2-0a2], [`overflowing_signum`][f-os-2-0a2],
  * `F128Bits` has been replaced by [`F128`][f128-2-0a2] which has proper
    floating-point ordering and various classification methods and associated
    constants.

[f-c-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.ceil
[f-cc-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_ceil
[f-cf-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_floor
[f-cil10-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_int_log10
[f-cil2-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_int_log2
[f-cm-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_mul
[f-cma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_mul_add
[f-cr-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_round
[f-crtte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_round_ties_to_even
[f-cs-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.checked_signum
[f-f-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.floor
[f-fr-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.frac
[f-i-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.int
[f-il10-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.int_log10
[f-il2-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.int_log2
[f-ma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.mul_add
[f-oc-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_ceil
[f-of-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_floor
[f-om-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_mul
[f-oma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_mul_add
[f-or-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_round
[f-ortte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_round_ties_to_even
[f-os-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.overflowing_signum
[f-r-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.round
[f-rtte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.round_ties_to_even
[f-rtz-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.round_to_zero
[f-s-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.signum
[f-sc-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_ceil
[f-sf-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_floor
[f-sm-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_mul
[f-sma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_mul_add
[f-sr-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_round
[f-srtte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_round_ties_to_even
[f-ss-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.saturating_signum
[f-uc-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_ceil
[f-uf-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_floor
[f-um-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_mul
[f-uma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_mul_add
[f-ur-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_round
[f-urtte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_round_ties_to_even
[f-us-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.unwrapped_signum
[f-wc-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_ceil
[f-wd-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wide_div
[f-wf-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_floor
[f-wim-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wide_mul
[f-wm-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_mul
[f-wma-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_mul_add
[f-wr-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_round
[f-wrtte-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_round_ties_to_even
[f-ws-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.FixedI32.html#method.wrapping_signum
[f128-2-0a2]: https://tspiteri.gitlab.io/fixed/dev/fixed/struct.F128.html

### Version 2.0.0-alpha.1 news (2022-02-26)

  * The crate now requires the nightly compiler with the [`generic_const_exprs`
    feature] enabled.
  * The crate now uses generic constant expressions to specify the number of
    fractional bits.
  * The deprecated optional features `az` and `f16` were removed. These features
    had no effect, as the functionality they enabled is now always enabled.
  * The `INT_NBITS` and `FRAC_NBITS` associated constants were replaced with
    [`INT_BITS`][f-ib-2-0a1] and [`FRAC_BITS`][f-fb-2-0a1] which can be negative.

[f-fb-2-0a1]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#associatedconstant.FRAC_BITS
[f-ib-2-0a1]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#associatedconstant.INT_BITS
[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560

### Version 1.13.1 news (2022-03-04)

  * The [`const_fixed_from_int`][cffi-1-13] macro now accepts a visibility
    qualifier ([merge request 10]).

### Version 1.13.0 news (2022-02-22)

  * The [`AddAssign`], [`SubAssign`], [`MulAssign`], [`DivAssign`],
    [`RemAssign`], [`BitAndAssign`], [`BitOrAssign`] and [`BitXorAssign`] traits
    for <code>[Wrapping][w-1-13]&lt;F></code> and
    <code>[Unwrapped][u-1-13]&lt;F></code> are now also implemented with `F` as
    the type of the right-hand side operand.
  * Bug fix: compilation with certain flags was hanging for the thumbv6m target
    because of a [rustc/LLVM issue][rust issue 75045]. This version should not
    trigger the rustc/LLVM issue ([issue 45]).

[`AddAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html
[`BitAndAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.BitAndAssign.html
[`BitOrAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.BitOrAssign.html
[`BitXorAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.BitXorAssign.html
[`DivAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html
[`MulAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html
[`RemAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.RemAssign.html
[`SubAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html
[cffi-1-13]: https://docs.rs/fixed/~1.13/fixed/macro.const_fixed_from_int.html
[issue 45]: https://gitlab.com/tspiteri/fixed/-/issues/45
[merge request 10]: https://gitlab.com/tspiteri/fixed/-/merge_requests/10
[rust issue 75045]: https://github.com/rust-lang/rust/issues/75045
[u-1-13]: https://docs.rs/fixed/~1.13/fixed/struct.Unwrapped.html
[w-1-13]: https://docs.rs/fixed/~1.13/fixed/struct.Wrapping.html

### Version 1.12.0 news (2022-02-04)

  * The crate now requires rustc version 1.57.0 or later.
  * The [`wide_div`][f-wd-1-12] method was added to all fixed-point numbers up to
    64 bits wide ([issue 25]).
  * The following methods are now `const` functions:
      * [`unwrapped_neg`][f-un-1-12], [`unwrapped_add`][f-ua-1-12],
        [`unwrapped_sub`][f-us-1-12]
      * [`unwrapped_mul_int`][f-umi-1-12]
      * [`unwrapped_shl`][f-ushl-1-12], [`unwrapped_shr`][f-ushr-1-12]
      * [`unwrapped_abs`][f-uabs-1-12], [`unwrapped_dist`][f-ud-1-12]
      * [`unwrapped_next_power_of_two`][f-unpot-1-12]

[f-ua-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_add
[f-uabs-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_abs
[f-ud-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_dist
[f-umi-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_mul_int
[f-un-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_neg
[f-unpot-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedU32.html#method.unwrapped_next_power_of_two
[f-us-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_sub
[f-ushl-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_shl
[f-ushr-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.unwrapped_shr
[f-wd-1-12]: https://docs.rs/fixed/~1.12/fixed/struct.FixedI32.html#method.wide_div
[issue 25]: https://gitlab.com/tspiteri/fixed/-/issues/25

### Other releases

Details on other releases can be found in [*RELEASES.md*].

[*RELEASES.md*]: https://gitlab.com/tspiteri/fixed/blob/master/RELEASES.md

## Quick examples

```rust
#![feature(generic_const_exprs)]

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

The type [`I20F12`] is a 32-bit fixed-point signed number with 20 integer bits
and 12 fractional bits. It is an alias to <code>[FixedI32]\<12></code>. The
unsigned counterpart would be [`U20F12`]. Aliases are provided for all
combinations of integer and fractional bits adding up to a total of eight, 16,
32, 64 or 128 bits.

```rust
#![feature(generic_const_exprs)]

use fixed::types::{I4F4, I4F12};

// -8 ≤ I4F4 < 8 with steps of 1/16 (~0.06)
let a = I4F4::from_num(1);
// multiplication and division by integers are possible
let ans1 = a / 5 * 17;
// 1 / 5 × 17 = 3 2/5 (3.4), but we get 3 3/16 (~3.2)
assert_eq!(ans1, I4F4::from_bits((3 << 4) + 3));
assert_eq!(ans1.to_string(), "3.2");

// -8 ≤ I4F12 < 8 with steps of 1/4096 (~0.0002)
let wider_a = I4F12::from(a);
let wider_ans = wider_a / 5 * 17;
let ans2 = I4F4::from_num(wider_ans);
// now the answer is the much closer 3 6/16 (~3.4)
assert_eq!(ans2, I4F4::from_bits((3 << 4) + 6));
assert_eq!(ans2.to_string(), "3.4");
```

The second example shows some precision and conversion issues. The low precision
of `a` means that `a / 5` is 3⁄16 instead of 1⁄5, leading to an inaccurate
result `ans1` = 3 3⁄16 (~3.2). With a higher precision, we get `wider_a / 5`
equal to 819⁄4096, leading to a more accurate intermediate result `wider_ans` =
3 1635⁄4096. When we convert back to four fractional bits, we get `ans2` = 3
6⁄16 (~3.4).

Note that we can convert from [`I4F4`] to [`I4F12`] using [`From`], as the
target type has the same number of integer bits and a larger number of
fractional bits. Converting from [`I4F12`] to [`I4F4`] cannot use [`From`] as we
have less fractional bits, so we use [`from_num`] instead.

## Writing fixed-point constants and values literally

The [*fixed-macro* crate] provides a convenient macro to write down fixed-point
constants literally in the code.

```rust
#![feature(generic_const_exprs)]

use fixed::types::I16F16;
use fixed_macro::fixed;

const NUM1: I16F16 = fixed!(12.75: I16F16);
let num2 = NUM1 + fixed!(13.125: I16F16);
assert_eq!(num2, 25.875);
```

## Using the *fixed* crate

The *fixed* crate is available on [crates.io][*fixed* crate]. To use it in your
crate, add it as a dependency inside [*Cargo.toml*]:

```toml
[dependencies]
fixed = "2.0.0-alpha.1"
```

This alpha version of the *fixed* crate requires the nightly compiler with the
[`generic_const_exprs` feature] enabled.

[`generic_const_exprs` feature]: https://github.com/rust-lang/rust/issues/76560

## Optional features

The *fixed* crate has these optional feature:

 1. `arbitrary`, disabled by default. This provides the generation of arbitrary
    fixed-point numbers from raw, unstructured data. This feature requires the
    [*arbitrary* crate].
 2. `serde`, disabled by default. This provides serialization support for the
    fixed-point types. This feature requires the [*serde* crate].
 3. `std`, disabled by default. This is for features that are not possible under
    `no_std`: currently the implementation of the [`Error`] trait for
    [`ParseFixedError`].
 4. `serde-str`, disabled by default. Fixed-point numbers are serialized as
    strings showing the value when using human-readable formats. This feature
    requires the `serde` and the `std` optional features. **Warning:** numbers
    serialized when this feature is enabled cannot be deserialized when this
    feature is disabled, and vice versa.

To enable features, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.fixed]
version = "2.0.0-alpha.1"
features = ["serde"]
```

## Experimental optional features

It is not considered a breaking change if the following experimental features
are removed. The removal of experimental features would however require a minor
version bump. Similarly, on a minor version bump, optional dependencies can be
updated to an incompatible newer version.

 1. `borsh`, disabled by default. This implements serialization and
    deserialization using the [*borsh* crate]. (The plan is to promote this to
    an optional feature once the [*borsh* crate] reaches version 1.0.0.)
 2. `num-traits`, disabled by default. This implements some traits from the
    [*num-traits* crate]. (The plan is to promote this to an optional feature
    once the [*num-traits* crate] reaches version 1.0.0.)

## License

This crate is free software: you can redistribute it and/or modify it under the
terms of either

  * the [Apache License, Version 2.0][LICENSE-APACHE] or
  * the [MIT License][LICENSE-MIT]

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache License, Version 2.0,
shall be dual licensed as above, without any additional terms or conditions.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[*arbitrary* crate]: https://crates.io/crates/arbitrary
[*borsh* crate]: https://crates.io/crates/borsh
[*cordic* crate]: https://crates.io/crates/cordic
[*fixed* crate]: https://crates.io/crates/fixed
[*fixed-macro* crate]: https://crates.io/crates/fixed-macro
[*fixed-sqrt* crate]: https://crates.io/crates/fixed-sqrt
[*half* crate]: https://crates.io/crates/half
[*num-traits* crate]: https://crates.io/crates/num-traits
[*serde* crate]: https://crates.io/crates/serde
[CORDIC]: https://en.wikipedia.org/wiki/CORDIC
[FixedI32]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html
[FixedU32]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU32.html
[LICENSE-APACHE]: https://www.apache.org/licenses/LICENSE-2.0
[LICENSE-MIT]: https://opensource.org/licenses/MIT
[`Binary`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html
[`Display`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html
[`Error`]: https://doc.rust-lang.org/nightly/std/error/trait.Error.html
[`FixedI128`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI128.html
[`FixedI16`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI16.html
[`FixedI32`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html
[`FixedI64`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI64.html
[`FixedI8`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI8.html
[`FixedU128`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU128.html
[`FixedU16`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU16.html
[`FixedU32`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU32.html
[`FixedU64`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU64.html
[`FixedU8`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedU8.html
[`FromFixed`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.FromFixed.html
[`FromStr`]: https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html
[`From`]: https://doc.rust-lang.org/nightly/core/convert/trait.From.html
[`I20F12`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/types/type.I20F12.html
[`I4F12`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/types/type.I4F12.html
[`I4F4`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/types/type.I4F4.html
[`Into`]: https://doc.rust-lang.org/nightly/core/convert/trait.Into.html
[`LosslessTryFrom`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.LosslessTryFrom.html
[`LosslessTryInto`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.LosslessTryInto.html
[`LossyFrom`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.LossyFrom.html
[`LossyInto`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.LossyInto.html
[`LowerHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html
[`Octal`]: https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html
[`ParseFixedError`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.ParseFixedError.html
[`ToFixed`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/traits/trait.ToFixed.html
[`U20F12`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/types/type.U20F12.html
[`UpperHex`]: https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html
[`az`]: https://docs.rs/az/^1/az/index.html
[`bf16`]: https://docs.rs/half/^1/half/struct.bf16.html
[`bytemuck`]: https://docs.rs/bytemuck/^1/bytemuck/index.html
[`checked_from_num`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.checked_from_num
[`f16`]: https://docs.rs/half/^1/half/struct.f16.html
[`from_num`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.from_num
[`from_str_binary`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.from_str_binary
[`from_str_hex`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.from_str_hex
[`from_str_octal`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.from_str_octal
[`i32`]: https://doc.rust-lang.org/nightly/core/primitive.i32.html
[`to_num`]: https://docs.rs/fixed/2.0.0-alpha.1/fixed/struct.FixedI32.html#method.to_num
[`u32`]: https://doc.rust-lang.org/nightly/core/primitive.u32.html
