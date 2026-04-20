# num-bigint — ETNA Tasks

Total tasks: 20

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task: the command executes the framework-specific adapter against the buggy variant branch and should report a counterexample.

Run against a variant by first checking out its branch (`git checkout etna/<variant>`) or applying its patch on a clean tree (`git apply patches/<variant>.patch`).

## Task Index

| Task | Variant | Framework | Property | Witness(es) | Command |
|------|---------|-----------|----------|-------------|---------|
| 001 | `is_multiple_of_zero_016c04a_1` | proptest | `property_is_multiple_of_zero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` | `cargo run --release --bin etna -- proptest IsMultipleOfZero` |
| 002 | `is_multiple_of_zero_016c04a_1` | quickcheck | `property_is_multiple_of_zero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` | `cargo run --release --bin etna -- quickcheck IsMultipleOfZero` |
| 003 | `is_multiple_of_zero_016c04a_1` | crabcheck | `property_is_multiple_of_zero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` | `cargo run --release --bin etna -- crabcheck IsMultipleOfZero` |
| 004 | `is_multiple_of_zero_016c04a_1` | hegel | `property_is_multiple_of_zero` | `witness_is_multiple_of_zero_case_zero`, `witness_is_multiple_of_zero_case_nonzero_small`, `witness_is_multiple_of_zero_case_nonzero_large` | `cargo run --release --bin etna -- hegel IsMultipleOfZero` |
| 005 | `mac3_all_zero_0940e50_1` | proptest | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- proptest MulSquareAllOnes` |
| 006 | `mac3_all_zero_0940e50_1` | quickcheck | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- quickcheck MulSquareAllOnes` |
| 007 | `mac3_all_zero_0940e50_1` | crabcheck | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- crabcheck MulSquareAllOnes` |
| 008 | `mac3_all_zero_0940e50_1` | hegel | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- hegel MulSquareAllOnes` |
| 009 | `mul_undersized_buffer_8008707_1` | proptest | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- proptest MulSquareAllOnes` |
| 010 | `mul_undersized_buffer_8008707_1` | quickcheck | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- quickcheck MulSquareAllOnes` |
| 011 | `mul_undersized_buffer_8008707_1` | crabcheck | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- crabcheck MulSquareAllOnes` |
| 012 | `mul_undersized_buffer_8008707_1` | hegel | `property_mul_square_all_ones` | `witness_mul_square_all_ones_case_32k` | `cargo run --release --bin etna -- hegel MulSquareAllOnes` |
| 013 | `neg_isize_promotion_3b13d9a_1` | proptest | `property_neg_isize_addassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` | `cargo run --release --bin etna -- proptest NegIsizeAddAssign` |
| 014 | `neg_isize_promotion_3b13d9a_1` | quickcheck | `property_neg_isize_addassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` | `cargo run --release --bin etna -- quickcheck NegIsizeAddAssign` |
| 015 | `neg_isize_promotion_3b13d9a_1` | crabcheck | `property_neg_isize_addassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` | `cargo run --release --bin etna -- crabcheck NegIsizeAddAssign` |
| 016 | `neg_isize_promotion_3b13d9a_1` | hegel | `property_neg_isize_addassign` | `witness_neg_isize_addassign_case_small_negative`, `witness_neg_isize_addassign_case_zero_negative` | `cargo run --release --bin etna -- hegel NegIsizeAddAssign` |
| 017 | `scalar_div_zero_5dcf2a1_1` | proptest | `property_scalar_div_by_zero_panics` | `witness_scalar_div_by_zero_panics_case_zero` | `cargo run --release --bin etna -- proptest ScalarDivByZeroPanics` |
| 018 | `scalar_div_zero_5dcf2a1_1` | quickcheck | `property_scalar_div_by_zero_panics` | `witness_scalar_div_by_zero_panics_case_zero` | `cargo run --release --bin etna -- quickcheck ScalarDivByZeroPanics` |
| 019 | `scalar_div_zero_5dcf2a1_1` | crabcheck | `property_scalar_div_by_zero_panics` | `witness_scalar_div_by_zero_panics_case_zero` | `cargo run --release --bin etna -- crabcheck ScalarDivByZeroPanics` |
| 020 | `scalar_div_zero_5dcf2a1_1` | hegel | `property_scalar_div_by_zero_panics` | `witness_scalar_div_by_zero_panics_case_zero` | `cargo run --release --bin etna -- hegel ScalarDivByZeroPanics` |

## Witness catalog

Each witness is a deterministic concrete test in `tests/etna_witnesses.rs`. Base build: passes. Variant-active build: fails.

- `witness_is_multiple_of_zero_case_zero` — `property_is_multiple_of_zero(0)` → `Pass`. Under `is_multiple_of_zero_016c04a_1` the call panics from `0 % 0`.
- `witness_is_multiple_of_zero_case_nonzero_small` — `property_is_multiple_of_zero(7)` → `Pass`. Same bug: small nonzero dividend panics when `other == 0`.
- `witness_is_multiple_of_zero_case_nonzero_large` — `property_is_multiple_of_zero(u64::MAX)` → `Pass`. Same bug at the largest single-u64 input.
- `witness_mul_square_all_ones_case_4k` — `property_mul_square_all_ones(tag)` for `n = 4096` → `Pass`. Base: identity holds; under `mul_undersized_buffer_8008707_1` the Karatsuba path trips the undersized buffer.
- `witness_mul_square_all_ones_case_16k` — `property_mul_square_all_ones(tag)` for `n = 16384` → `Pass`. Larger all-ones square; undersized buffer still fires.
- `witness_mul_square_all_ones_case_32k` — `property_mul_square_all_ones(tag)` for `n = 32768` → `Pass`. Reliably triggers both the undersized-buffer bug and the `mac3` all-zero `unwrap`.
- `witness_mul_does_not_panic_case_fuzzed_mul_1` — `property_mul_does_not_panic(a_bytes, b_bytes)` → `Pass` on base; exercises the regression input from upstream `tests/fuzzed.rs::fuzzed_mul_1`.
- `witness_mul_does_not_panic_case_small` — `property_mul_does_not_panic(small, small)` → `Pass`. Guards the non-Karatsuba code path from regressions.
- `witness_mul_does_not_panic_case_empty` — `property_mul_does_not_panic([], [])` → `Pass`. Guards zero-length BigUint multiplication.
- `witness_neg_isize_addassign_case_small_negative` — `property_neg_isize_addassign(0, -1)` → `Pass`. Under `neg_isize_promotion_3b13d9a_1` the result is off by `2^64` instead of `-1`.
- `witness_neg_isize_addassign_case_zero_negative` — `property_neg_isize_addassign(0, i16::MIN)` → `Pass`. Same `UsizePromotion` misrouting with the most negative `i16`.
- `witness_neg_isize_addassign_case_positive` — `property_neg_isize_addassign(10, 5)` → `Pass`. Sanity case (still succeeds under the variant because the bug only affects negative inputs).
- `witness_scalar_div_by_zero_panics_case_zero` — `property_scalar_div_by_zero_panics(0)` → `Pass` on base (divide-by-zero panic observed). Under `scalar_div_zero_5dcf2a1_1` the `BigUint::zero() / 0u32` returns `0` silently, failing the property.
- `witness_scalar_div_by_zero_panics_case_nonzero` — `property_scalar_div_by_zero_panics(42)` → `Pass`. Nonzero dividend already panics in the variant, so this case continues to pass; the zero case is the detector.
