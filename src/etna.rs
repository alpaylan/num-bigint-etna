//! Framework-neutral property functions for the ETNA workload.
//!
//! Each `property_<name>` takes concrete owned inputs and returns a
//! `PropertyResult`. Adapters for proptest / quickcheck / crabcheck / hegel
//! call these directly in `src/bin/etna.rs`, and the witness tests in
//! `tests/etna_witnesses.rs` assert `PropertyResult::Pass` with frozen inputs.

#![cfg(feature = "std")]

extern crate std;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::AddAssign;
use num_integer::Integer;
use num_traits::{Num, One};
use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::{BigInt, BigUint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

/// Invariant: `BigUint::from(a).is_multiple_of(&BigUint::zero())` returns
/// `a == 0` and does not panic. The buggy implementation falls through to
/// `self % other`, which divides by zero and panics.
pub fn property_is_multiple_of_zero(a: u64) -> PropertyResult {
    let ba = BigUint::from(a);
    let zero = BigUint::from(0u32);
    let got = catch_unwind(AssertUnwindSafe(|| ba.is_multiple_of(&zero)));
    match got {
        Ok(b) => {
            let expected = a == 0;
            if b == expected {
                PropertyResult::Pass
            } else {
                PropertyResult::Fail(format!(
                    "BigUint::from({a}).is_multiple_of(&0) = {b}, expected {expected}"
                ))
            }
        }
        Err(_) => PropertyResult::Fail(format!(
            "BigUint::from({a}).is_multiple_of(&0) panicked"
        )),
    }
}

/// Invariant: dividing a `BigUint` by the scalar `0u32` always panics,
/// matching `std`'s integer division semantics. The buggy implementation
/// silently returns `BigUint::zero()` when the dividend has no digits.
pub fn property_scalar_div_by_zero_panics(a: u64) -> PropertyResult {
    let ba = BigUint::from(a);
    let got = catch_unwind(AssertUnwindSafe(|| ba / 0u32));
    match got {
        Err(_) => PropertyResult::Pass,
        Ok(q) => PropertyResult::Fail(format!(
            "BigUint::from({a}) / 0u32 did not panic (returned {q})"
        )),
    }
}

/// Invariant: `BigInt::from(a) += b as isize` equals
/// `BigInt::from(a + b as i64)`. The buggy implementation routes `isize`
/// through `UsizePromotion`, casting negatives to a huge positive value.
pub fn property_neg_isize_addassign(a: i64, b: i16) -> PropertyResult {
    let mut x = BigInt::from(a);
    let bi = b as isize;
    AddAssign::<isize>::add_assign(&mut x, bi);
    let expected = BigInt::from(a.wrapping_add(b as i64));
    if x == expected {
        PropertyResult::Pass
    } else {
        PropertyResult::Fail(format!(
            "{a} += ({bi} as isize) = {x}, expected {expected}"
        ))
    }
}

/// Invariant: letting `n = 1 << (8 + (bits_tag % 8))` so `n` ranges over
/// `{256, 512, ..., 32768}` bits, for `x = 2^n - 1` the identity
/// `x * x == 2^(2n) - 2*x - 1` holds. The buggy implementation allocates an
/// undersized product buffer (`x.len() + y.len()` instead of
/// `x.len() + y.len() + 1`), so at `n >= 2048` Karatsuba kicks in and the
/// square either overruns the buffer or underflows an intermediate subtraction.
pub fn property_mul_square_all_ones(bits_tag: u8) -> PropertyResult {
    let shift = 8u32 + (bits_tag as u32 % 8);
    let n: u32 = 1u32 << shift;
    let x = (BigUint::one() << n) - 1u32;
    let res = catch_unwind(AssertUnwindSafe(|| {
        let x2 = &x * &x;
        let expected = (BigUint::one() << (2 * n)) - &x - &x - 1u32;
        x2 == expected
    }));
    match res {
        Ok(true) => PropertyResult::Pass,
        Ok(false) => PropertyResult::Fail(format!(
            "(2^{n}-1)^2 mismatches closed form"
        )),
        Err(_) => PropertyResult::Fail(format!("(2^{n}-1)^2 panicked")),
    }
}

/// Invariant: multiplying any two `BigUint` values never panics, even when
/// internal Karatsuba recursion produces an all-zero sub-operand. The buggy
/// implementation unwraps a `None` from `iter.position(|&d| d != 0)` when
/// the sub-slice is entirely zero.
pub fn property_mul_does_not_panic(a_bytes: Vec<u8>, b_bytes: Vec<u8>) -> PropertyResult {
    let a = BigUint::from_bytes_be(&a_bytes);
    let b = BigUint::from_bytes_be(&b_bytes);
    let res = catch_unwind(AssertUnwindSafe(|| &a * &b));
    match res {
        Ok(_) => PropertyResult::Pass,
        Err(_) => PropertyResult::Fail(format!(
            "mul panicked: a.len={}, b.len={}",
            a_bytes.len(),
            b_bytes.len()
        )),
    }
}

/// Convenience constant: the fuzzed all-zero-mac3 input `a` from upstream
/// `tests/fuzzed.rs::fuzzed_mul_1`. Exposed so the witness can use the exact
/// byte sequence without re-parsing inside `#[test]` bodies.
pub fn fuzzed_mul_1_inputs() -> (BigUint, BigUint) {
    let hex1 = "\
        cd6839ee857cf791a40494c2e522846eefbca9eca9912fdc1feed4561dbde75c75f1ddca2325ebb1\
        b9cd6eae07308578e58e57f4ddd7dc239b4fd347b883e37d87232a8e5d5a8690c8dba69c97fe8ac4\
        58add18be7e460e03c9d1ae8223db53d20681a4027ffc17d1e43b764791c4db5ff7add849da7e378\
        ac8d9be0e8b517c490da3c0f944b6a52a0c5dc5217c71da8eec35d2c3110d8b041d2b52f3e2a8904\
        abcaaca517a8f2ef6cd26ceadd39a1cf9f770bc08f55f5a230cd81961348bb18534245430699de77\
        d93b805153cffd05dfd0f2cfc2332888cec9c5abf3ece9b4d7886ad94c784bf74fce12853b2a9a75\
        b62a845151a703446cc20300eafe7332330e992ae88817cd6ccef8877b66a7252300a4664d7074da\
        181cd9fd502ea1cd71c0b02db3c009fe970a7d226382cdba5b5576c5c0341694681c7adc4ca2d059\
        d9a6b300957a2235a4eb6689b71d34dcc4037b520eabd2c8b66604bb662fe2bcf533ba8d242dbc91\
        f04c1795b9f0fee800d197d8c6e998248b15855a9602b76cb3f94b148d8f71f7d6225b79d63a8e20\
        8ec8f0fa56a1c381b6c09bad9886056aec17fc92b9bb0f8625fd3444e40cccc2ede768ddb23c66ad\
        59a680a26a26d519d02e4d46ce93cce9e9dd86702bdd376abae0959a0e8e418aa507a63fafb8f422\
        83b03dc26f371c5e261a8f90f3ac9e2a6bcc7f0a39c3f73043b5aa5a950d4e945e9f68b2c2e593e3\
        b995be174714c1967b71f579043f89bfce37437af9388828a3ba0465c88954110cae6d38b638e094\
        13c15c9faddd6fb63623fd50e06d00c4d5954e787158b3e4eea7e9fae8b189fa8a204b23ac2f7bbc\
        b601189c0df2075977c2424336024ba3594172bea87f0f92beb20276ce8510c8ef2a4cd5ede87e7e\
        38b3fa49d66fbcd322be686a349c24919f4000000000000000000000000000000000000000000000\
        000000000000000000000000000000000";
    let hex2 = "\
        40000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000";
    (
        BigUint::from_str_radix(hex1, 16).expect("fuzzed_mul_1 hex1 parse"),
        BigUint::from_str_radix(hex2, 16).expect("fuzzed_mul_1 hex2 parse"),
    )
}

pub const ALL_PROPERTIES: &[&str] = &[
    "IsMultipleOfZero",
    "ScalarDivByZeroPanics",
    "NegIsizeAddAssign",
    "MulSquareAllOnes",
    "MulDoesNotPanic",
];
