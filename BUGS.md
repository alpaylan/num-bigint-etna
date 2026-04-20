# num-bigint — Injected Bugs

Total mutations: 5

All variants are patch-based; apply the listed patch to a clean HEAD to reproduce the buggy build. Each `etna/<variant>` branch is a pre-applied snapshot.

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | `is_multiple_of(&0)` falls through to division | `is_multiple_of_zero_016c04a_1` | `patches/is_multiple_of_zero_016c04a_1.patch` | patch | `016c04ac4d6757d455578e1433b6001e35e73f71` |
| 2 | `mac3` unwraps `None` on all-zero sub-slice | `mac3_all_zero_0940e50_1` | `patches/mac3_all_zero_0940e50_1.patch` | patch | `0940e509dca55d19197adbe7cb5c1d5423a390cf` |
| 3 | Undersized product buffer in multiplication | `mul_undersized_buffer_8008707_1` | `patches/mul_undersized_buffer_8008707_1.patch` | patch | `8008707fea97b9215f3b949a0eb6044aec709a31` |
| 4 | Negative `isize` addend routed through `UsizePromotion` | `neg_isize_promotion_3b13d9a_1` | `patches/neg_isize_promotion_3b13d9a_1.patch` | patch | `3b13d9a20e649c928282f88531af8a07f03ef092` |
| 5 | `BigUint` scalar division hides divide-by-zero for zero dividend | `scalar_div_zero_5dcf2a1_1` | `patches/scalar_div_zero_5dcf2a1_1.patch` | patch | `5dcf2a1deb1b8e2e225521cb103ee90a8c70b666` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `is_multiple_of_zero_016c04a_1` | `property_is_multiple_of_zero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` |
| `mac3_all_zero_0940e50_1` | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` |
| `mul_undersized_buffer_8008707_1` | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` |
| `neg_isize_promotion_3b13d9a_1` | `property_neg_isize_addassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` |
| `scalar_div_zero_5dcf2a1_1` | `property_scalar_div_by_zero_panics` | `witness_scalar_div_by_zero_panics_case_zero` |

## Framework Coverage

| Property | etna | proptest | quickcheck | crabcheck | hegel |
|----------|:----:|:--------:|:----------:|:---------:|:-----:|
| `property_is_multiple_of_zero` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_scalar_div_by_zero_panics` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_neg_isize_addassign` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_mul_square_all_ones` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_mul_does_not_panic` | ✓ | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. `is_multiple_of(&0)` falls through to division

- **Variant**: `is_multiple_of_zero_016c04a_1`
- **Location**: `patches/is_multiple_of_zero_016c04a_1.patch` (applies to `src/biguint.rs`)
- **Property**: `property_is_multiple_of_zero`
- **Witness(es)**: `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large`
- **Fix commit**: `016c04ac4d6757d455578e1433b6001e35e73f71` — `Fix is_multiple_of with a 0 arg`
- **Invariant violated**: `BigUint::is_multiple_of(&0)` returns `true` iff the receiver is zero, and never panics.
- **How the mutation triggers**: the explicit `if other.is_zero() { return self.is_zero(); }` guard is removed, so `self.is_multiple_of(&zero)` falls through to `(self % other).is_zero()`, which divides by zero and panics.

### 2. `mac3` unwraps `None` on all-zero sub-slice

- **Variant**: `mac3_all_zero_0940e50_1`
- **Location**: `patches/mac3_all_zero_0940e50_1.patch` (applies to `src/biguint/multiplication.rs`)
- **Property**: `property_mul_square_all_ones`
- **Witness(es)**: `witness_mul_square_all_ones_case_32k`
- **Fix commit**: `0940e509dca55d19197adbe7cb5c1d5423a390cf` — `Fix a mac3 panic when an operand is all-zero`
- **Invariant violated**: `BigUint` multiplication never panics for finite, well-formed inputs.
- **How the mutation triggers**: `mac3` replaces the `if let Some(nz) = iter.position(|&d| d != 0) { … } else { return }` fallback with `.unwrap()`. During Karatsuba/Toom-3 recursion on very large operands (e.g. `(2^32768 − 1)^2`) a sub-slice ends up entirely zero, so `position` returns `None` and the `unwrap` panics.

### 3. Undersized product buffer in multiplication

- **Variant**: `mul_undersized_buffer_8008707_1`
- **Location**: `patches/mul_undersized_buffer_8008707_1.patch` (applies to `src/biguint/multiplication.rs`)
- **Property**: `property_mul_square_all_ones`
- **Witness(es)**: `witness_mul_square_all_ones_case_32k`
- **Fix commit**: `8008707fea97b9215f3b949a0eb6044aec709a31` — `Fix an undersized buffer panic in multiplication`
- **Invariant violated**: `(2^n − 1)^2 == 2^(2n) − 2·(2^n − 1) − 1`, computed without panicking, for any `n` up to the tested 32768 bits.
- **How the mutation triggers**: the product buffer size drops from `x1.len() + y1.len() + 1` to `x1.len() + y1.len()` in both `mul3` code paths, so at Karatsuba recursion the intermediate subtraction underflows or the buffer overruns.

### 4. Negative `isize` addend routed through `UsizePromotion`

- **Variant**: `neg_isize_promotion_3b13d9a_1`
- **Location**: `patches/neg_isize_promotion_3b13d9a_1.patch` (applies to `src/macros.rs`)
- **Property**: `property_neg_isize_addassign`
- **Witness(es)**: `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative`
- **Fix commit**: `3b13d9a20e649c928282f88531af8a07f03ef092` — `Fix promotion of negative isize`
- **Invariant violated**: `BigInt::from(a) += (b as isize)` equals `BigInt::from(a) + BigInt::from(b as i64)` for all `a: i64, b: i16`.
- **How the mutation triggers**: `promote_signed_scalars_assign!` is changed so the `isize` `AddAssign` impl promotes through `UsizePromotion` instead of `IsizePromotion`, casting a negative `isize` to a huge positive `usize`.

### 5. `BigUint` scalar division hides divide-by-zero for zero dividend

- **Variant**: `scalar_div_zero_5dcf2a1_1`
- **Location**: `patches/scalar_div_zero_5dcf2a1_1.patch` (applies to `src/biguint/division.rs`)
- **Property**: `property_scalar_div_by_zero_panics`
- **Witness(es)**: `witness_scalar_div_by_zero_panics_case_zero`
- **Fix commit**: `5dcf2a1deb1b8e2e225521cb103ee90a8c70b666` — `Fix scalar divide-by-zero panics`
- **Invariant violated**: `BigUint / 0u32` must panic (mirroring `u32 / 0` semantics) regardless of the dividend.
- **How the mutation triggers**: the explicit divide-by-zero guards in the `BigUint / u32` paths are removed, so when the dividend has no digits (i.e. is zero) the code silently returns `BigUint::zero()` instead of panicking.
