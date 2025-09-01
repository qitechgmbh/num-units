# Num Units - Type-Safe Dimensional Analysis for Rust 🧮⚖️

A comprehensive, zero-cost dimensional analysis library that prevents unit conversion errors at compile time. Perfect for physics simulations, engineering calculations, and any domain where dimensional correctness is critical.


## Cargo Features

- `std` (default): Enable standard library support
- `libm`: Enable libm support for no_std floating-point operations


## `Quantity` Implementation Status

### Core Operations
- [X] `core::ops::Add`
  - `Quantity + Quantity`
- [X] `core::ops::Sub`
  - `Quantity - Quantity`
- [X] `core::ops::Mul`
  - `Quantity * Quantity`
  - `Quantity * Num`
- [X] `core::ops::Div`
  - `Quantity / Quantity`
  - `Quantity / Num`
- [X] `core::ops::Rem`
  - `Quantity % Quantity`
  - `Quantity % Num`
- [X] `num_traits::Pow`
  - `Quantity.pow(Num)`
  - `Quantity.pow(Quantity)`

### Checked Operations
- [ ] `num_traits::CheckedAdd`
  - `Quantity.checked_add(Quantity)`
- [ ] `num_traits::CheckedSub`
  - `Quantity.checked_sub(Quantity)`
- [ ] `num_traits::CheckedMul`
  - `Quantity.checked_mul(Quantity)`
- [ ] `num_traits::CheckedNeg`
  - `Quantity.checked_neg()`
- [ ] `num_traits::CheckedRem`
  - `Quantity.checked_rem(&Quantity)`
- [ ] Custom Functions
  - `Quantity.checked_mul_quantity(Quantity)`
  - `Quantity.checked_mul_scalar(Num)`
  - `Quantity.checked_div(Quantity)`
  - `Quantity.checked_div_scalar(Num)`
  - `Quantity.checked_rem_quantity(Quantity)`
  - `Quantity.checked_rem_scalar(Num)`


### Saturating Operations
- [ ] `num_traits::SaturatingAdd`
  - `Quantity.saturating_add(Quantity)`
- [ ] `num_traits::SaturatingSub`
  - `Quantity.saturating_sub(Quantity)`
- [ ] `num_traits::SaturatingMul`
  - `Quantity.saturating_mul(Quantity)`
- Custom Functions
  - `Quantity.saturating_mul_quantity(Quantity)`
  - `Quantity.saturating_mul_scalar(Num)`


### Wrapping Operations
- [ ] `num_traits::WrappingAdd`
  - `Quantity.wrapping_add(Quantity)`
- [ ] `num_traits::WrappingSub`
  - `Quantity.wrapping_sub(Quantity)`
- [ ] `num_traits::WrappingMul`
  - `Quantity.wrapping_mul(Quantity)`
- [ ] `num_traits::WrappingNeg`
  - `Quantity.wrapping_neg()`
- [ ] Custom Functions
  - `Quantity.wrapping_mul_quantity(Quantity)`
  - `Quantity.wrapping_mul_scalar(Num)`


### Identity
- [X] `num_traits::Zero`
  - `Quantity::zero()`
  - `Quantity.is_zero()`
- [X] `num_traits::One`
  - `Quantity::one()`
  - `Quantity.is_one()`
- [X] `num_traits::ConstZero`
  - `Quantity::ZERO`
- [ ] `num_traits::ConstOne`
  - `Quantity::from_one_raw()`

## Signed
- [X] `num_traits::Signed`
  - `Quantity.abs()`
  - `Quantity.abs_sub(&other)`
  - `Quantity.signum()`
  - `Quantity.is_positive()`
  - `Quantity.is_negative()`

### Float
- [X] `num_traits::Float`
  - `Quantity::nan()`, `Quantity::infinity()`, `Quantity::neg_infinity()`, `Quantity::neg_zero()`, `Quantity::min_value()`, `Quantity::min_positive_value()`, `Quantity::max_value()`, `Quantity::is_nan()`, `Quantity::is_infinite()`, `Quantity::is_finite()`, `Quantity::is_normal()`, `Quantity::classify()`, `Quantity::floor()`, `Quantity::ceil()`, `Quantity::round()`, `Quantity::trunc()`, `Quantity::fract()`, `Quantity::is_sign_positive()`, `Quantity::is_sign_negative()`, `Quantity::recip()`, `Quantity::powi(n)`, `Quantity::powf(n)`, `Quantity::sqrt()`, `Quantity::cbrt()`, `Quantity::exp()`, `Quantity::exp2()`, `Quantity::ln()`, `Quantity::log(base)`, `Quantity::log2()`, `Quantity::log10()`, `Quantity::exp_m1()`, `Quantity::ln_1p()`, `Quantity::sin()`, `Quantity::cos()`, `Quantity::tan()`, `Quantity::asin()`, `Quantity::acos()`, `Quantity::atan()`, `Quantity::atan2(other)`, `Quantity::sin_cos()`, `Quantity::sinh()`, `Quantity::cosh()`, `Quantity::tanh()`, `Quantity::asinh()`, `Quantity::acosh()`, `Quantity::atanh()`, `Quantity::max(other)`, `Quantity::min(other)`, `Quantity::hypot(other)`, `Quantity::integer_decode()`
- [X] `num_traits::FloatConst`
  - `Quantity::PI()`, `Quantity::E()`, `Quantity::FRAC_PI_2()`, `Quantity::FRAC_PI_3()`, `Quantity::FRAC_PI_4()`, `Quantity::FRAC_PI_6()`, `Quantity::FRAC_PI_8()`, `Quantity::FRAC_1_PI()`, `Quantity::FRAC_2_PI()`, `Quantity::FRAC_2_SQRT_PI()`, `Quantity::SQRT_2()`, `Quantity::FRAC_1_SQRT_2()`, `Quantity::SQRT_3()`, `Quantity::LN_2()`, `Quantity::LN_10()`, `Quantity::LOG2_E()`, `Quantity::LOG10_E()` 
- [X]  Custom Functions
  - `Quantity.e()`, `Quantity.pi()`, `Quantity.frac_pi_2()`, `Quantity.frac_pi_3()`, `Quantity.frac_pi_4()`, `Quantity.frac_pi_6()`, `Quantity.frac_pi_8()`, `Quantity.frac_1_pi()`, `Quantity.frac_2_pi()`, `Quantity.frac_2_sqrt_pi()`, `Quantity.sqrt_2()`, `Quantity.frac_1_sqrt_2()`, `Quantity.sqrt_3()`, `Quantity.ln_2()`, `Quantity.ln_10()`, `Quantity.log2_e()`, `Quantity.log10_e()`


### Conversion
- [ ] `num_traits::AsPrimitive`
  - `Quantity.as_::<T>()`
- [ ] `num_traits::FromPrimitive`
  - `Quantity::from_i8(n)`, `Quantity::from_i16(n)`, `Quantity::from_i32(n)`, `Quantity::from_i64(n)`, `Quantity::from_i128(n)`
  - `Quantity::from_u8(n)`, `Quantity::from_u16(n)`, `Quantity::from_u32(n)`, `Quantity::from_u64(n)`, `Quantity::from_u128(n)`
  - `Quantity::from_f32(n)`, `Quantity::from_f64(n)`, `Quantity::from_isize(n)`, `Quantity::from_usize(n)`
- [ ] `num_traits::ToPrimitive`
  - `Quantity.to_i8()`, `Quantity.to_i16()`, `Quantity.to_i32()`, `Quantity.to_i64()`, `Quantity.to_i128()`
  - `Quantity.to_u8()`, `Quantity.to_u16()`, `Quantity.to_u32()`, `Quantity.to_u64()`, `Quantity.to_u128()`
  - `Quantity.to_f32()`, `Quantity.to_f64()`, `Quantity.to_isize()`, `Quantity.to_usize()`
- [ ] `num_traits::NumCast`
  - `Quantity::from::<T>(n)`

### Mul-Add Operations
- [X] `num_traits::MulAdd`
  - `Quantity.mul_add(a, b)`
- [X] `num_traits::MulAddAssign`
  - `Quantity.mul_add_assign(a, b)`
- [X] Custom Functions
  - `Quantity.mul_add_scalar(a, b)`
  - `Quantity.mul_add_mixed(a, b)`
  - `Quantity.mul_add_assign_scalar(a, b)`
  - `Quantity.mul_add_assign_mixed(a, b)`

