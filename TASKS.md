# num-bigint — ETNA Tasks

Total tasks: 20

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `is_multiple_of_zero_016c04a_1` | proptest | `IsMultipleOfZero` | `witness_is_multiple_of_zero_case_zero` |
| 002 | `is_multiple_of_zero_016c04a_1` | quickcheck | `IsMultipleOfZero` | `witness_is_multiple_of_zero_case_zero` |
| 003 | `is_multiple_of_zero_016c04a_1` | crabcheck | `IsMultipleOfZero` | `witness_is_multiple_of_zero_case_zero` |
| 004 | `is_multiple_of_zero_016c04a_1` | hegel | `IsMultipleOfZero` | `witness_is_multiple_of_zero_case_zero` |
| 005 | `mac3_all_zero_0940e50_1` | proptest | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 006 | `mac3_all_zero_0940e50_1` | quickcheck | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 007 | `mac3_all_zero_0940e50_1` | crabcheck | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 008 | `mac3_all_zero_0940e50_1` | hegel | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 009 | `mul_undersized_buffer_8008707_1` | proptest | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 010 | `mul_undersized_buffer_8008707_1` | quickcheck | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 011 | `mul_undersized_buffer_8008707_1` | crabcheck | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 012 | `mul_undersized_buffer_8008707_1` | hegel | `MulSquareAllOnes` | `witness_mul_square_all_ones_case_32k` |
| 013 | `neg_isize_promotion_3b13d9a_1` | proptest | `NegIsizeAddassign` | `witness_neg_isize_addassign_case_small_negative` |
| 014 | `neg_isize_promotion_3b13d9a_1` | quickcheck | `NegIsizeAddassign` | `witness_neg_isize_addassign_case_small_negative` |
| 015 | `neg_isize_promotion_3b13d9a_1` | crabcheck | `NegIsizeAddassign` | `witness_neg_isize_addassign_case_small_negative` |
| 016 | `neg_isize_promotion_3b13d9a_1` | hegel | `NegIsizeAddassign` | `witness_neg_isize_addassign_case_small_negative` |
| 017 | `scalar_div_zero_5dcf2a1_1` | proptest | `ScalarDivByZeroPanics` | `witness_scalar_div_by_zero_panics_case_zero` |
| 018 | `scalar_div_zero_5dcf2a1_1` | quickcheck | `ScalarDivByZeroPanics` | `witness_scalar_div_by_zero_panics_case_zero` |
| 019 | `scalar_div_zero_5dcf2a1_1` | crabcheck | `ScalarDivByZeroPanics` | `witness_scalar_div_by_zero_panics_case_zero` |
| 020 | `scalar_div_zero_5dcf2a1_1` | hegel | `ScalarDivByZeroPanics` | `witness_scalar_div_by_zero_panics_case_zero` |

## Witness Catalog

- `witness_is_multiple_of_zero_case_zero` — base passes, variant fails
- `witness_is_multiple_of_zero_case_nonzero_small` — base passes, variant fails
- `witness_is_multiple_of_zero_case_nonzero_large` — base passes, variant fails
- `witness_mul_square_all_ones_case_32k` — base passes, variant fails
- `witness_neg_isize_addassign_case_small_negative` — base passes, variant fails
- `witness_neg_isize_addassign_case_zero_negative` — base passes, variant fails
- `witness_scalar_div_by_zero_panics_case_zero` — base passes, variant fails
