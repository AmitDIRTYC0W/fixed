<!-- Copyright © 2018–2020 Trevor Spiteri -->

<!-- Copying and distribution of this file, with or without
modification, are permitted in any medium without royalty provided the
copyright notice and this notice are preserved. This file is offered
as-is, without any warranty. -->

Version 1.4.0 (unreleased)
==========================

  * For the experimental feature [`serde-str`][feat-exp-1-4],
    serialization in human-readable formats was made more convenient
    to write manually. (This makes it incompatible with the version in
    1.3.0.)

[feat-exp-1-4]: https://tspiteri.gitlab.io/fixed/dev/fixed/#experimental-optional-features

Version 1.3.0 (2020-10-15)
==========================

  * The [`MulAssign`] implementation on fixed-point numbers now
    accepts an rhs fixed-point number with a different number of
    fractional bits from `self`.
  * The following methods were added to all fixed-point types, to the
    [`Fixed`][tf-1-3] trait, and to the [`Wrapping`][w-1-3] wrapper:
      * [`mul_add`][f-ma-1-3], [`checked_mul_add`][f-cma-1-3],
        [`saturating_mul_add`][f-sma-1-3],
        [`wrapping_mul_add`][f-wma-1-3],
        [`overflowing_mul_add`][f-oma-1-3]
  * The new experimental feature [`unwrapped`][feat-exp-1-3] was
    added, providing arithmetic methods that panic on overflow even
    when debug assertions are disabled.
  * The new experimental feature [`serde-str`][feat-exp-1-3] was
    added, which makes serialization use the number’s value in
    human-readable formats.

[f-cma-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.FixedI32.html#method.checked_mul_add
[f-ma-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.FixedI32.html#method.mul_add
[f-oma-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.FixedI32.html#method.overflowing_mul_add
[f-sma-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.FixedI32.html#method.saturating_mul_add
[f-wma-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.FixedI32.html#method.wrapping_mul_add
[feat-exp-1-3]: https://docs.rs/fixed/~1.3/fixed/#experimental-optional-features
[tf-1-3]: https://docs.rs/fixed/~1.3/fixed/traits/trait.Fixed.html
[w-1-3]: https://docs.rs/fixed/~1.3/fixed/struct.Wrapping.html

Version 1.2.0 (2020-09-02)
==========================

  * The [`const_fixed_from_int!`][cffi-1-2] macro was added to make it
    easy to define constant fixed-point numbers using integer
    expressions ([issue 20]).

[cffi-1-2]: https://docs.rs/fixed/~1.2/fixed/macro.const_fixed_from_int.html
[issue 20]: https://gitlab.com/tspiteri/fixed/-/issues/20

Version 1.1.0 (2020-07-21)
==========================

  * The new experimental feature [`num-traits`][feat-nt-1-1] was added
    to implement some traits, and to also add the relevant traits as
    supertraits to the [`FixedOptionalFeatures`][fof-1-1] trait
    ([issue 18]).

[issue 18]: https://gitlab.com/tspiteri/fixed/-/issues/18
[feat-nt-1-1]: https://docs.rs/fixed/~1.1/fixed/#experimental-optional-features
[fof-1-1]: https://docs.rs/fixed/~1.1/fixed/traits/trait.FixedOptionalFeatures.html

Version 1.0.0 (2020-06-04)
==========================

  * The crate now requires rustc version 1.44.0 or later.
  * The following methods are now `const` functions:
      * [`from_be_bytes`][f-fbb-1-0], [`from_le_bytes`][f-flb-1-0],
        [`from_ne_bytes`][f-fnb-1-0]
      * [`to_be_bytes`][f-tbb-1-0], [`to_le_bytes`][f-tlb-1-0],
        [`to_ne_bytes`][f-tnb-1-0]
  * All deprecated items were removed.

[f-fbb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.from_be_bytes
[f-flb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.from_le_bytes
[f-fnb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.from_ne_bytes
[f-tbb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.to_be_bytes
[f-tlb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.to_le_bytes
[f-tnb-1-0]: https://docs.rs/fixed/~1.0/fixed/struct.FixedI32.html#method.to_ne_bytes

Version 0.5.7 (2020-05-11)
==========================

  * The `LosslessTryFrom` and `LosslessTryInto` traits were added.
  * The following methods were added to all fixed-point types, to the
    `Fixed` trait, and to the `Wrapping` wrapper:
      * `leading_ones`, `trailing_ones`
  * The following method was added to unsigned fixed-point types and
    to the `FixedUnsigned` trait:
      * `wrapping_next_power_of_two`
  * The `PHI` and `FRAC_1_PHI` constants were added to the `consts`
    module and as associated constants for fixed-point types.

Version 0.5.6 (2020-05-01)
==========================

  * The following methods were added to signed fixed-point types and to
    the `FixedSigned` trait:
      * `checked_signum`, `saturating_signum`, `wrapping_signum`,
        `overflowing_signum`
  * The `LossyFrom` and `LossyInto` traits were added to the prelude.
  * Casts deprecated in version 0.3.1 of the *az* crate were marked as
    deprecated.

Version 0.5.5 (2020-04-16)
==========================

  * Bug fix: an incorrect result could be given when comparing a
    signed fixed-point number of type `FixedI` to a number that would
    overflow by exactly one bit when converting to `FixedI`.
  * The following associated constants were added to all fixed-point
    types, to the `Fixed` trait, and to the `Wrapping` wrapper:
      * `MIN`, `MAX`
  * The following associated constants were added to the `Fixed` trait
    and to the `Wrapping` wrapper:
      * `INT_NBITS`, `FRAC_NBITS`
  * The following methods were added to all fixed-point types and to
    the `Fixed` trait:
      * `int_log2`, `int_log10`
      * `checked_int_log2`, `checked_int_log10`
  * The following methods were added to the `Wrapping` wrapper:
      *  `int_log2`, `int_log10`
  * The constants in the `consts` module were also added as associated
    constants to fixed-point types that can represent them.
  * The following methods were deprecated:
      * `min_value`, `max_value`
      * `int_nbits`, `frac_nbits`

Version 0.5.4 (2020-02-21)
==========================

  * Bug fix: `rem_euclid_int` and its checked versions were handling
    overflow incorrectly.

Version 0.5.3 (2020-02-13)
==========================

  * Bug fix: `round_to_zero` was returning incorrect results for
    negative whole number operands.
  * Bug fix: all remainder operations with a fixed-point LHS and an
    integer RHS were giving an incorrect answer
    (https://gitlab.com/tspiteri/fixed/issues/13).
  * Bug fix: Euclidean division operations by integers were giving an
    incorrect answer.
  * `Rem` and `RemAssign` were implemented for fixed-point numbers.
  * The following methods were added to all fixed-point types and to
    the `Fixed` trait:
      * `checked_rem`
      * `div_euclid`, `rem_euclid`
      * `checked_div_euclid`, `checked_rem_euclid`
      * `saturating_div_euclid`
      * `wrapping_div_euclid`
      * `overflowing_div_euclid`
  * The following methods were added to the `Wrapping` wrapper:
      * `div_euclid`, `rem_euclid`
      * `div_euclid_int`, `rem_euclid_int`
  * The following methods were deprecated:
      * `wrapping_rem_int`, `overflowing_rem_int`

Version 0.5.2 (2020-02-02)
==========================

  * `Wrapping` now supports serialization. (Thanks: Shane Pearman)

Version 0.5.1 (2019-12-22)
==========================

  * `ParseFixedError` implements `Error` when the new `std` feature is
    enabled.

Version 0.5.0 (2019-12-06)
==========================

  * The crate now requires rustc version 1.39.0 or later.
  * The following methods were added to all fixed-point types and to
    the `Fixed` trait:
      * `from_be_bytes`, `from_le_bytes`, `from_ne_bytes`
      * `to_be_bytes`, `to_le_bytes`, `to_ne_bytes`
      * `div_euclid_int`, `rem_euclid_int`
      * `checked_div_euclid_int`, `checked_rem_euclid_int`
      * `wrapping_div_euclid_int`, `wrapping_rem_euclid_int`
      * `overflowing_div_euclid_int`, `overflowing_rem_euclid_int`
  * Casts deprecated in version 0.3.1 of the *az* crate were marked as
    deprecated.

Incompatible changes
--------------------

  * Deprecated methods and modules were removed.

Version 0.4.6 (2019-10-16)
==========================

  * Conversions to/from `bf16` are now provided when the `f16` option
    is enabled.
  * The following methods are now `const` functions: `saturating_neg`,
    `saturating_add`, `saturating_sub`, `saturating_mul_int`,
    `saturating_abs`
  * Support for casts using the *az* crate was added.

Version 0.4.5 (2019-08-30)
==========================

  * Bug fix: display of many decimal numbers was panicking in debug
    mode or including a leading zero in release mode.
  * Many methods were added to `Wrapping` for convenience, even if
    they do not involve wrapping.

Version 0.4.4 (2019-08-24)
==========================

  * Bug fix: rounding could produce bad output for `Binary`, `Octal`,
    `LowerHex` and `UpperHex`.
  * The following methods are now `const` functions:
    `is_power_of_two`, `abs`, `wrapping_abs`, `overflowing_abs`
  * The method `round_to_zero` was added.
  * The method `round_ties_to_even` and its checked versions were
    added.

Version 0.4.3 (2019-08-20)
==========================

  * The crate now requires rustc version 1.34.0 or later.
  * The precision argument is no longer ignored when formatting
    fixed-point numbers; precision should now be handled the same as
    for primitive floating-point numbers in the standard library.
  * Parsing strings now rounds to the nearest with ties rounded to
    even.
  * Checked versions of string parsing methods are now available as
    inherent methods to all fixed-point numbers, and as methods in the
    `Fixed` trait.
  * `Wrapping` now has methods for parsing with wrapping, including
    an implementation of `FromStr`.
  * The following methods are now `const` functions:
      * `min_value`, `max_value`, `from_bits`, `to_bits`
      * `count_ones`, `count_zeros`, `leading_zeros`, `trailing_zeros`
        `rotate_left`, `rotate_right`
      * `wrapping_neg`, `wrapping_add`, `wrapping_sub`,
        `wrapping_mul_int`, `wrapping_shl`, `wrapping_shr`
      * `overflowing_neg`, `overflowing_add`, `overflowing_sub`,
        `overflowing_mul_int`, `overflowing_shl`, `overflowing_shr`
      * `is_positive`, `is_negative`
  * The associated constants `INT_NBITS` and `FRAC_NBITS` were added.
  * The reexports in the `frac` module and the `LeEqU*` traits were
    moved into the new `types::extra` module.

Version 0.4.2 (2019-08-16)
==========================

  * The new methods `from_num` and `to_num` together with their
    checked versions were added to all fixed-point numbers.
  * The methods `from_fixed`, `to_fixed`, `from_int`, `to_int`,
    `from_float`, and `to_float`, and their checked versions, were
    deprecated.
  * The new method `from_num` was added to the `Wrapping` wrapper.
  * Bug fix: parsing of decimal fractions was fixed to give correctly
    rounded results for long decimal fraction strings, for example
    with four fractional bits, 0.96874999… (just below 31⁄32) and
    0.96875 (31⁄32) are now parsed correctly as 0.9375 (15⁄16) and 1.0.

Version 0.4.1 (2019-08-12)
==========================

  * All fixed-point types now implement `FromStr`.
  * The methods `from_str_binary`, `from_str_octal` and `from_str_hex`
    were added.

Version 0.4.0 (2019-08-08)
==========================

  * The crate now requires rustc version 1.31.0 or later.
  * The `traits` module was added, with its traits `Fixed`,
    `FixedSigned`, `FixedUnsigned`, `FromFixed`, `ToFixed`,
    `LossyFrom` and `LossyInto`.
  * The `saturating_neg` method was added to all fixed-point numbers,
    and the `saturating_abs` method was added to signed fixed-point
    numbers.
  * The `consts` module was added.
  * The `signum` method now wraps instead of panics in release mode.

Incompatible changes
--------------------

  * The sealed traits `Int` and `Float` now have no provided methods;
    the methods in the old implementation are now provided by
    `FromFixed` and `ToFixed`.
  * Deprecated methods were removed.

Contributors
------------

  * @jean-airoldie
  * @tspiteri

Version 0.3.3 (2019-06-27)
==========================

  * Conversions to/from `isize` and `usize` were added.

Version 0.3.2 (2019-02-27)
==========================

  * The `Wrapping` wrapper was added.

Version 0.3.1 (2019-02-07)
==========================

  * Reimplement `From<bool>` for all fixed-point types which can
    represent the integer 1. This was inadvertently removed in 0.3.0.

Version 0.3.0 (2019-02-03)
==========================

  * Incompatible change: the return type of `to_int` is now generic.
  * Incompatible change: the `Int` trait implementation for `bool` was
    removed.
  * The new method `to_fixed` was added.
  * The new methods `checked_to_fixed`, `checked_to_int`,
    `saturating_to_fixed`, `saturating_to_int`, `wrapping_to_fixed`,
    `wrapping_to_int`, `overflowing_to_fixed` and `overflowing_to_int`
    were added.
  * The methods `from_fixed`, `to_fixed`, `checked_from_fixed`,
    `checked_to_fixed`, `saturating_from_fixed`,
    `saturating_to_fixed`, `wrapping_from_fixed`, `wrapping_to_fixed`,
    `overflowing_from_fixed` and `overflowing_to_fixed` were added to
    the `Int` trait.
  * The methods `from_fixed`, `to_fixed`, `checked_to_fixed`,
    `saturating_to_fixed`, `wrapping_to_fixed` and
    `overflowing_to_fixed` were added to the `Float` trait.
  * `PartialEq` and `PartialCmp` are now implemented for all
    combinations of fixed-point numbers and primitive integers.
  * The methods `int_bits` and `frac_bits` were deprecated and
    replaced by the methods `int_nbits` and `frac_nbits`.

Version 0.2.1 (2019-01-29)
==========================

  * Bug fix: the `from_fixed` and `from_int` methods (and their
    checked counterparts) could return wrong values for negative
    values.
  * Bug fix: display was using one fractional digit less than
    required, thus yielding the same output for diffent fixed-point
    numbers.

Version 0.2.0 (2019-01-29)
==========================

  * Incompatible change: The method `from_int` was change to accept a
    generic parameter.
  * The new methods `from_fixed`, `checked_from_fixed`,
    `saturating_from_fixed`, `wrapping_from_fixed` and
    `overflowing_from_fixed` were added.
  * The new methods `checked_from_int`, `saturating_from_int`,
    `wrapping_from_int` and `overflowing_from_int` were added.
  * The new methods `from_float`, `checked_from_float`,
    `saturating_from_float`, `wrapping_from_float` and
    `overflowing_from_float` were added.
  * The new method `to_float` was added.
  * The methods `from_f16`, `from_f32`, `from_f64`, `to_f16`, `to_f32`
    and `to_f64` were deprecated.
  * The `to_int` method was fixed to truncate fractional bits as
    documented for negative values.
  * The new methods `ceil`, `floor`, `round`, `checked_ceil`,
    `checked_floor`, `checked_round`, `saturating_ceil`,
    `saturating_floor`, `saturating_round`, `wrapping_ceil`,
    `wrapping_floor`, `wrapping_round`, `overflowing_ceil`,
    `overflowing_floor` and `overflowing_round` were added.
  * The methods `to_int_ceil`, `to_int_floor` and `to_int_round` were
    deprecated.

Version 0.1.6 (2019-01-27)
==========================

  * Optional serde support was added.

Version 0.1.5 (2019-01-26)
==========================

  * Lossless infallible conversions between fixed-point numbers and
    numeric primitives are now supported using `From` and `Into`.
  * A new module `types` is available with aliases for all supported
    fixed-point numbers.

Version 0.1.4 (2018-11-29)
==========================

  * Division is now implemented for `FixedI128` and `FixedU128`.

Version 0.1.3 (2018-08-23)
==========================

  * The `f16` feature was added, and new methods `from_f16` and
    `to_f16` were added.

Version 0.1.2 (2018-08-15)
==========================

  * The crate can now be used without the standard library `std`.
  * New methods `from_f32` and `from_f64` were added.
  * New methods `is_positive` and `is_negative` were added to signed
    fixed-point numbers.

Version 0.1.1 (2018-08-11)
==========================

  * Comparisons are now supported between all fixed-point numbers with
    the same underlying integer type.
  * New static methods `int_bits` and `frac_bits` were added.
  * New methods `from_int`, `to_int`, `to_int_ceil`, `to_int_floor`
    and `to_int_round` were added.
  * New methods `int` and `frac` were added.
  * Support for multiplication and division by integers was added.

Version 0.1.0 (2018-08-10)
==========================

  * `Unsigned` constants provided by the *typenum* crate are now used
    for the number of fractional bits.
  * Many methods and trait implementations available for primitive
    integers are now also supported by the fixed-point numbers.

[`MulAssign`]: https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html
