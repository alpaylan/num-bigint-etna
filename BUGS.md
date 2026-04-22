# num-bigint — Injected Bugs

Total mutations: 5

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `is_multiple_of_zero_016c04a_1` | `is_multiple_of_zero` | `src/biguint.rs` | `patch` | `016c04ac4d6757d455578e1433b6001e35e73f71` |
| 2 | `mac3_all_zero_0940e50_1` | `mac3_all_zero` | `src/biguint/multiplication.rs` | `patch` | `0940e509dca55d19197adbe7cb5c1d5423a390cf` |
| 3 | `mul_undersized_buffer_8008707_1` | `mul_undersized_buffer` | `src/biguint/multiplication.rs` | `patch` | `8008707fea97b9215f3b949a0eb6044aec709a31` |
| 4 | `neg_isize_promotion_3b13d9a_1` | `neg_isize_promotion` | `src/macros.rs` | `patch` | `3b13d9a20e649c928282f88531af8a07f03ef092` |
| 5 | `scalar_div_zero_5dcf2a1_1` | `scalar_div_zero` | `src/biguint/division.rs` | `patch` | `5dcf2a1deb1b8e2e225521cb103ee90a8c70b666` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `is_multiple_of_zero_016c04a_1` | `IsMultipleOfZero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` |
| `mac3_all_zero_0940e50_1` | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| `mul_undersized_buffer_8008707_1` | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| `neg_isize_promotion_3b13d9a_1` | `NegIsizeAddassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` |
| `scalar_div_zero_5dcf2a1_1` | `ScalarDivByZeroPanics` | `witness_scalar_div_by_zero_panics_case_zero` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `IsMultipleOfZero` | ✓ | ✓ | ✓ | ✓ |
| `MulSquareAllOnes` | ✓ | ✓ | ✓ | ✓ |
| `NegIsizeAddassign` | ✓ | ✓ | ✓ | ✓ |
| `ScalarDivByZeroPanics` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. is_multiple_of_zero

- **Variant**: `is_multiple_of_zero_016c04a_1`
- **Location**: `src/biguint.rs`
- **Property**: `IsMultipleOfZero`
- **Witness(es)**:
  - `witness_is_multiple_of_zero_case_zero`
  - `witness_is_multiple_of_zero_case_nonzero_small`
  - `witness_is_multiple_of_zero_case_nonzero_large`
- **Source**: Fix is_multiple_of with a 0 arg
  > `BigUint::is_multiple_of(&0)` fell through to `(self % other).is_zero()`, triggering a divide-by-zero panic. The fix adds an explicit `if other.is_zero() { return self.is_zero(); }` early-return so the zero-argument case is answered without division.
- **Fix commit**: `016c04ac4d6757d455578e1433b6001e35e73f71` — Fix is_multiple_of with a 0 arg
- **Invariant violated**: `BigUint::is_multiple_of(&0)` returns `true` iff the receiver is zero, and never panics.
- **How the mutation triggers**: the explicit `if other.is_zero() { return self.is_zero(); }` guard is removed, so `self.is_multiple_of(&zero)` falls through to `(self % other).is_zero()`, which divides by zero and panics.

### 2. mac3_all_zero

- **Variant**: `mac3_all_zero_0940e50_1`
- **Location**: `src/biguint/multiplication.rs`
- **Property**: `MulSquareAllOnes`
- **Witness(es)**:
  - `witness_mul_square_all_ones_case_32k`
- **Source**: Fix a mac3 panic when an operand is all-zero
  > `mac3` called `.unwrap()` on `iter.position(|&d| d != 0)`, assuming at least one non-zero digit. During Karatsuba/Toom-3 recursion on very large operands a sub-slice can become entirely zero, so `position` returns `None` and the unwrap panics. The fix replaces the unwrap with an `if let Some(nz) … else { return }` branch.
- **Fix commit**: `0940e509dca55d19197adbe7cb5c1d5423a390cf` — Fix a mac3 panic when an operand is all-zero
- **Invariant violated**: `BigUint` multiplication never panics for finite, well-formed inputs.
- **How the mutation triggers**: `mac3` replaces the `if let Some(nz) = iter.position(|&d| d != 0) { … } else { return }` fallback with `.unwrap()`. During Karatsuba/Toom-3 recursion on very large operands (e.g. `(2^32768 − 1)^2`) a sub-slice ends up entirely zero, so `position` returns `None` and the `unwrap` panics.

### 3. mul_undersized_buffer

- **Variant**: `mul_undersized_buffer_8008707_1`
- **Location**: `src/biguint/multiplication.rs`
- **Property**: `MulSquareAllOnes`
- **Witness(es)**:
  - `witness_mul_square_all_ones_case_32k`
- **Source**: Fix an undersized buffer panic in multiplication
  > `mul3` allocated a product buffer of `x1.len() + y1.len()` digits, one digit short of what the Karatsuba recursion needs for an intermediate subtraction. The fix bumps the allocation to `x1.len() + y1.len() + 1` on both code paths.
- **Fix commit**: `8008707fea97b9215f3b949a0eb6044aec709a31` — Fix an undersized buffer panic in multiplication
- **Invariant violated**: `(2^n − 1)^2 == 2^(2n) − 2·(2^n − 1) − 1`, computed without panicking, for any `n` up to the tested 32768 bits.
- **How the mutation triggers**: the product buffer size drops from `x1.len() + y1.len() + 1` to `x1.len() + y1.len()` in both `mul3` code paths, so at Karatsuba recursion the intermediate subtraction underflows or the buffer overruns.

### 4. neg_isize_promotion

- **Variant**: `neg_isize_promotion_3b13d9a_1`
- **Location**: `src/macros.rs`
- **Property**: `NegIsizeAddassign`
- **Witness(es)**:
  - `witness_neg_isize_addassign_case_small_negative`
  - `witness_neg_isize_addassign_case_zero_negative`
- **Source**: Fix promotion of negative isize
  > `promote_signed_scalars_assign!` routed the `isize` `AddAssign` impl through `UsizePromotion`, so a negative `isize` was reinterpreted as a huge positive `usize` before addition. The fix routes `isize` through `IsizePromotion`, preserving the sign.
- **Fix commit**: `3b13d9a20e649c928282f88531af8a07f03ef092` — Fix promotion of negative isize
- **Invariant violated**: `BigInt::from(a) += (b as isize)` equals `BigInt::from(a) + BigInt::from(b as i64)` for all `a: i64, b: i16`.
- **How the mutation triggers**: `promote_signed_scalars_assign!` is changed so the `isize` `AddAssign` impl promotes through `UsizePromotion` instead of `IsizePromotion`, casting a negative `isize` to a huge positive `usize`.

### 5. scalar_div_zero

- **Variant**: `scalar_div_zero_5dcf2a1_1`
- **Location**: `src/biguint/division.rs`
- **Property**: `ScalarDivByZeroPanics`
- **Witness(es)**:
  - `witness_scalar_div_by_zero_panics_case_zero`
- **Source**: Fix scalar divide-by-zero panics
  > `BigUint / u32` code paths lacked an explicit divide-by-zero check; when the dividend had no digits (i.e. was zero) the code silently returned `BigUint::zero()` instead of panicking, diverging from primitive `u32 / 0` semantics. The fix re-adds the explicit guards.
- **Fix commit**: `5dcf2a1deb1b8e2e225521cb103ee90a8c70b666` — Fix scalar divide-by-zero panics
- **Invariant violated**: `BigUint / 0u32` must panic (mirroring `u32 / 0` semantics) regardless of the dividend.
- **How the mutation triggers**: the explicit divide-by-zero guards in the `BigUint / u32` paths are removed, so when the dividend has no digits (i.e. is zero) the code silently returns `BigUint::zero()` instead of panicking.
