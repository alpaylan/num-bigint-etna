//! Deterministic witness tests for every variant in the ETNA workload.
//!
//! Each `witness_<prop>_case_<tag>` test calls `property_<prop>` with frozen
//! inputs and asserts `PropertyResult::Pass`. On the base commit every
//! witness passes. When the corresponding `etna/<variant>` branch is checked
//! out, at least one witness for that variant fails.

use num_bigint::etna::{
    fuzzed_mul_1_inputs, property_is_multiple_of_zero, property_mul_does_not_panic,
    property_mul_square_all_ones, property_neg_isize_addassign,
    property_scalar_div_by_zero_panics, PropertyResult,
};

fn assert_pass(r: PropertyResult, label: &str) {
    match r {
        PropertyResult::Pass => {}
        PropertyResult::Discard => panic!("{label}: unexpected Discard"),
        PropertyResult::Fail(m) => panic!("{label}: {m}"),
    }
}

#[test]
fn witness_is_multiple_of_zero_case_zero() {
    assert_pass(property_is_multiple_of_zero(0), "is_multiple_of_zero/zero");
}

#[test]
fn witness_is_multiple_of_zero_case_nonzero_small() {
    assert_pass(property_is_multiple_of_zero(7), "is_multiple_of_zero/7");
}

#[test]
fn witness_is_multiple_of_zero_case_nonzero_large() {
    assert_pass(
        property_is_multiple_of_zero(u64::MAX),
        "is_multiple_of_zero/u64::MAX",
    );
}

#[test]
fn witness_scalar_div_by_zero_panics_case_zero() {
    assert_pass(
        property_scalar_div_by_zero_panics(0),
        "scalar_div_by_zero/zero",
    );
}

#[test]
fn witness_scalar_div_by_zero_panics_case_nonzero() {
    assert_pass(
        property_scalar_div_by_zero_panics(42),
        "scalar_div_by_zero/42",
    );
}

#[test]
fn witness_neg_isize_addassign_case_small_negative() {
    assert_pass(
        property_neg_isize_addassign(100, -5),
        "neg_isize_addassign/100 + -5",
    );
}

#[test]
fn witness_neg_isize_addassign_case_zero_negative() {
    assert_pass(
        property_neg_isize_addassign(0, -1),
        "neg_isize_addassign/0 + -1",
    );
}

#[test]
fn witness_neg_isize_addassign_case_positive() {
    assert_pass(
        property_neg_isize_addassign(7, 3),
        "neg_isize_addassign/7 + 3",
    );
}

#[test]
fn witness_mul_square_all_ones_case_4k() {
    // shift = 8 + (4 % 8) = 12, n = 4096 bits
    assert_pass(
        property_mul_square_all_ones(4),
        "mul_square_all_ones/n=4096",
    );
}

#[test]
fn witness_mul_square_all_ones_case_16k() {
    // shift = 8 + (6 % 8) = 14, n = 16384 bits
    assert_pass(
        property_mul_square_all_ones(6),
        "mul_square_all_ones/n=16384",
    );
}

#[test]
fn witness_mul_square_all_ones_case_32k() {
    // shift = 8 + (7 % 8) = 15, n = 32768 bits
    assert_pass(
        property_mul_square_all_ones(7),
        "mul_square_all_ones/n=32768",
    );
}

#[test]
fn witness_mul_does_not_panic_case_fuzzed_mul_1() {
    let (a, b) = fuzzed_mul_1_inputs();
    assert_pass(
        property_mul_does_not_panic(a.to_bytes_be(), b.to_bytes_be()),
        "mul_does_not_panic/fuzzed_mul_1",
    );
}

#[test]
fn witness_mul_does_not_panic_case_small() {
    assert_pass(
        property_mul_does_not_panic(vec![1, 2, 3], vec![4, 5, 6]),
        "mul_does_not_panic/small",
    );
}

#[test]
fn witness_mul_does_not_panic_case_empty() {
    assert_pass(
        property_mul_does_not_panic(vec![], vec![1]),
        "mul_does_not_panic/empty",
    );
}
