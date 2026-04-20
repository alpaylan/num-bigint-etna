//! Programmatic dispatcher for the num-bigint ETNA workload.
//!
//! Each framework is driven directly — no subprocess, no shelling out. Each
//! adapter wraps the framework-neutral `property_<name>` functions in
//! `src/etna.rs`.

use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use num_bigint::etna::{
    fuzzed_mul_1_inputs, property_is_multiple_of_zero, property_mul_does_not_panic,
    property_mul_square_all_ones, property_neg_isize_addassign,
    property_scalar_div_by_zero_panics, PropertyResult, ALL_PROPERTIES,
};

fn pr_into_result(p: PropertyResult) -> Result<(), String> {
    match p {
        PropertyResult::Pass | PropertyResult::Discard => Ok(()),
        PropertyResult::Fail(m) => Err(m),
    }
}

#[derive(Default, Clone, Copy)]
struct Metrics {
    inputs: u64,
    elapsed_us: u128,
}

type Outcome = (Result<(), String>, Metrics);

fn run_all<F: FnMut(&str) -> Outcome>(mut f: F) -> Outcome {
    let mut total = Metrics::default();
    let mut final_status = Ok(());
    for prop in ALL_PROPERTIES {
        let (status, m) = f(prop);
        total.inputs += m.inputs;
        total.elapsed_us += m.elapsed_us;
        if status.is_err() && final_status.is_ok() {
            final_status = status;
        }
    }
    (final_status, total)
}

// ---------------------------------------------------------------------------
// etna (canonical witness-replay tool)

fn check_is_multiple_of_zero() -> Result<(), String> {
    pr_into_result(property_is_multiple_of_zero(0))
        .and_then(|_| pr_into_result(property_is_multiple_of_zero(7)))
        .and_then(|_| pr_into_result(property_is_multiple_of_zero(u64::MAX)))
}

fn check_scalar_div_by_zero_panics() -> Result<(), String> {
    pr_into_result(property_scalar_div_by_zero_panics(0))
        .and_then(|_| pr_into_result(property_scalar_div_by_zero_panics(42)))
}

fn check_neg_isize_addassign() -> Result<(), String> {
    pr_into_result(property_neg_isize_addassign(100, -5))
        .and_then(|_| pr_into_result(property_neg_isize_addassign(0, -1)))
        .and_then(|_| pr_into_result(property_neg_isize_addassign(7, 3)))
}

fn check_mul_square_all_ones() -> Result<(), String> {
    pr_into_result(property_mul_square_all_ones(4))
        .and_then(|_| pr_into_result(property_mul_square_all_ones(6)))
        .and_then(|_| pr_into_result(property_mul_square_all_ones(7)))
}

fn check_mul_does_not_panic() -> Result<(), String> {
    let (a, b) = fuzzed_mul_1_inputs();
    pr_into_result(property_mul_does_not_panic(a.to_bytes_be(), b.to_bytes_be()))
        .and_then(|_| pr_into_result(property_mul_does_not_panic(vec![1, 2, 3], vec![4, 5, 6])))
        .and_then(|_| pr_into_result(property_mul_does_not_panic(vec![], vec![1])))
}

fn run_etna_property(property: &str) -> Outcome {
    if property == "All" {
        return run_all(run_etna_property);
    }
    let t0 = Instant::now();
    let status = match property {
        "IsMultipleOfZero" => check_is_multiple_of_zero(),
        "ScalarDivByZeroPanics" => check_scalar_div_by_zero_panics(),
        "NegIsizeAddAssign" => check_neg_isize_addassign(),
        "MulSquareAllOnes" => check_mul_square_all_ones(),
        "MulDoesNotPanic" => check_mul_does_not_panic(),
        _ => {
            return (
                Err(format!("Unknown property for etna: {property}")),
                Metrics::default(),
            );
        }
    };
    (
        status,
        Metrics {
            inputs: 1,
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------------------------------------------------------------------------
// proptest

fn run_proptest_property(property: &str) -> Outcome {
    use proptest::collection::vec as pvec;
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestCaseError, TestError, TestRunner};
    if property == "All" {
        return run_all(run_proptest_property);
    }
    let counter = Arc::new(AtomicU64::new(0));
    let t0 = Instant::now();
    let cfg = Config {
        cases: 200,
        ..Config::default()
    };
    let mut runner = TestRunner::new(cfg);
    let status: Result<(), String> = match property {
        "IsMultipleOfZero" => {
            let c = counter.clone();
            runner
                .run(&any::<u64>(), move |a| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_is_multiple_of_zero(a)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({a})")))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "ScalarDivByZeroPanics" => {
            let c = counter.clone();
            runner
                .run(&any::<u64>(), move |a| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_scalar_div_by_zero_panics(a)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({a})")))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "NegIsizeAddAssign" => {
            let c = counter.clone();
            runner
                .run(&(any::<i64>(), any::<i16>()), move |(a, b)| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_neg_isize_addassign(a, b)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({a} {b})")))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "MulSquareAllOnes" => {
            let c = counter.clone();
            runner
                .run(&any::<u8>(), move |bits_tag| {
                    c.fetch_add(1, Ordering::Relaxed);
                    let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                        property_mul_square_all_ones(bits_tag)
                    }));
                    match res {
                        Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                        Ok(PropertyResult::Fail(_)) | Err(_) => {
                            Err(TestCaseError::fail(format!("({bits_tag})")))
                        }
                    }
                })
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        "MulDoesNotPanic" => {
            let c = counter.clone();
            runner
                .run(
                    &(pvec(any::<u8>(), 0..256), pvec(any::<u8>(), 0..256)),
                    move |(a_bytes, b_bytes)| {
                        c.fetch_add(1, Ordering::Relaxed);
                        let a = a_bytes.clone();
                        let b = b_bytes.clone();
                        let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                            property_mul_does_not_panic(a, b)
                        }));
                        match res {
                            Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => Ok(()),
                            Ok(PropertyResult::Fail(_)) | Err(_) => Err(TestCaseError::fail(
                                format!("({:?} {:?})", a_bytes, b_bytes),
                            )),
                        }
                    },
                )
                .map_err(|e| match e {
                    TestError::Fail(reason, _) => reason.to_string(),
                    other => other.to_string(),
                })
        }
        _ => {
            return (
                Err(format!("Unknown property for proptest: {property}")),
                Metrics::default(),
            );
        }
    };
    (
        status,
        Metrics {
            inputs: counter.load(Ordering::Relaxed),
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------------------------------------------------------------------------
// quickcheck (fork with `etna` feature)

static QC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn qc_is_multiple_of_zero(a: u64) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_is_multiple_of_zero(a)));
    match res {
        Ok(PropertyResult::Pass) => quickcheck::TestResult::passed(),
        Ok(PropertyResult::Discard) => quickcheck::TestResult::discard(),
        Ok(PropertyResult::Fail(_)) | Err(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_scalar_div_by_zero_panics(a: u64) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_scalar_div_by_zero_panics(a)));
    match res {
        Ok(PropertyResult::Pass) => quickcheck::TestResult::passed(),
        Ok(PropertyResult::Discard) => quickcheck::TestResult::discard(),
        Ok(PropertyResult::Fail(_)) | Err(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_neg_isize_addassign(a: i64, b: i16) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_neg_isize_addassign(a, b)));
    match res {
        Ok(PropertyResult::Pass) => quickcheck::TestResult::passed(),
        Ok(PropertyResult::Discard) => quickcheck::TestResult::discard(),
        Ok(PropertyResult::Fail(_)) | Err(_) => quickcheck::TestResult::failed(),
    }
}

fn qc_mul_square_all_ones(bits_tag: u8) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_mul_square_all_ones(bits_tag)));
    match res {
        Ok(PropertyResult::Pass) => quickcheck::TestResult::passed(),
        Ok(PropertyResult::Discard) => quickcheck::TestResult::discard(),
        Ok(PropertyResult::Fail(_)) | Err(_) => quickcheck::TestResult::failed(),
    }
}

// quickcheck's fn-pointer Testable bound requires Display on every arg, which
// `Vec<u8>` doesn't implement. Wrap the bytes in a tiny newtype that has a
// hex Display for readable counterexamples.
#[derive(Clone)]
struct ByteVec(Vec<u8>);

impl std::fmt::Debug for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Vec<u8> as std::fmt::Debug>::fmt(&self.0, f)
    }
}

impl std::fmt::Display for ByteVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        for (i, b) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            write!(f, "{b:02x}")?;
        }
        f.write_str("]")
    }
}

impl quickcheck::Arbitrary for ByteVec {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        // The forked QuickCheck::quicktest loop seeds Gen::set_size via
        // `log2(iter)`, so the first two iterations hand in size=0, and
        // Vec::arbitrary's `random_range(0..size)` panics. Cap at >= 1.
        let size = g.size().max(1);
        let len = g.random_range(0..size);
        let mut v: Vec<u8> = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(u8::arbitrary(g));
        }
        ByteVec(v)
    }
    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(self.0.shrink().map(ByteVec))
    }
}

fn qc_mul_does_not_panic(a_bytes: ByteVec, b_bytes: ByteVec) -> quickcheck::TestResult {
    QC_COUNTER.fetch_add(1, Ordering::Relaxed);
    // Cap at 256 bytes per operand — default Arbitrary<Vec<u8>> already scales
    // with size, but extreme inputs blow the budget with no extra signal.
    let a: Vec<u8> = a_bytes.0.into_iter().take(256).collect();
    let b: Vec<u8> = b_bytes.0.into_iter().take(256).collect();
    let res = std::panic::catch_unwind(AssertUnwindSafe(|| property_mul_does_not_panic(a, b)));
    match res {
        Ok(PropertyResult::Pass) => quickcheck::TestResult::passed(),
        Ok(PropertyResult::Discard) => quickcheck::TestResult::discard(),
        Ok(PropertyResult::Fail(_)) | Err(_) => quickcheck::TestResult::failed(),
    }
}

fn run_quickcheck_property(property: &str) -> Outcome {
    use quickcheck::{QuickCheck, ResultStatus};
    if property == "All" {
        return run_all(run_quickcheck_property);
    }
    QC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let qc = || {
        QuickCheck::new()
            .tests(200)
            .max_tests(1000)
            .max_time(Duration::from_secs(86_400))
    };
    let result = match property {
        "IsMultipleOfZero" => qc().quicktest(qc_is_multiple_of_zero as fn(u64) -> _),
        "ScalarDivByZeroPanics" => qc().quicktest(qc_scalar_div_by_zero_panics as fn(u64) -> _),
        "NegIsizeAddAssign" => qc().quicktest(qc_neg_isize_addassign as fn(i64, i16) -> _),
        "MulSquareAllOnes" => qc().quicktest(qc_mul_square_all_ones as fn(u8) -> _),
        "MulDoesNotPanic" => {
            qc().quicktest(qc_mul_does_not_panic as fn(ByteVec, ByteVec) -> _)
        }
        _ => {
            return (
                Err(format!("Unknown property for quickcheck: {property}")),
                Metrics::default(),
            );
        }
    };
    let status = match result.status {
        ResultStatus::Finished => Ok(()),
        ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        ResultStatus::Aborted { err } => Err(format!("aborted: {err:?}")),
        ResultStatus::TimedOut => Err("timed out".into()),
        ResultStatus::GaveUp => Err(format!("gave up after {} tests", result.n_tests_passed)),
    };
    (
        status,
        Metrics {
            inputs: QC_COUNTER.load(Ordering::Relaxed),
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------------------------------------------------------------------------
// crabcheck

static CC_COUNTER: AtomicU64 = AtomicU64::new(0);

fn cc_is_multiple_of_zero(a: u64) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_is_multiple_of_zero(a) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_scalar_div_by_zero_panics(a: u64) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_scalar_div_by_zero_panics(a) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_neg_isize_addassign(args: (i64, i16)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_neg_isize_addassign(args.0, args.1) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_mul_square_all_ones(bits_tag: u8) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    match property_mul_square_all_ones(bits_tag) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn cc_mul_does_not_panic(args: (Vec<u8>, Vec<u8>)) -> Option<bool> {
    CC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let (a, b) = args;
    // Clamp to 256 bytes per operand.
    let a: Vec<u8> = a.into_iter().take(256).collect();
    let b: Vec<u8> = b.into_iter().take(256).collect();
    match property_mul_does_not_panic(a, b) {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

fn run_crabcheck_property(property: &str) -> Outcome {
    use crabcheck::quickcheck as cc;
    if property == "All" {
        return run_all(run_crabcheck_property);
    }
    CC_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let cfg = cc::Config { tests: 200 };
    let result = match property {
        "IsMultipleOfZero" => {
            cc::quickcheck_with_config(cfg, cc_is_multiple_of_zero as fn(u64) -> Option<bool>)
        }
        "ScalarDivByZeroPanics" => cc::quickcheck_with_config(
            cfg,
            cc_scalar_div_by_zero_panics as fn(u64) -> Option<bool>,
        ),
        "NegIsizeAddAssign" => cc::quickcheck_with_config(
            cfg,
            cc_neg_isize_addassign as fn((i64, i16)) -> Option<bool>,
        ),
        "MulSquareAllOnes" => {
            cc::quickcheck_with_config(cfg, cc_mul_square_all_ones as fn(u8) -> Option<bool>)
        }
        "MulDoesNotPanic" => cc::quickcheck_with_config(
            cfg,
            cc_mul_does_not_panic as fn((Vec<u8>, Vec<u8>)) -> Option<bool>,
        ),
        _ => {
            return (
                Err(format!("Unknown property for crabcheck: {property}")),
                Metrics::default(),
            );
        }
    };
    let status = match result.status {
        cc::ResultStatus::Finished => Ok(()),
        cc::ResultStatus::Failed { arguments } => Err(format!("({})", arguments.join(" "))),
        cc::ResultStatus::TimedOut => Err("timed out".into()),
        cc::ResultStatus::GaveUp => Err(format!(
            "gave up: passed={}, discarded={}",
            result.passed, result.discarded
        )),
        cc::ResultStatus::Aborted { error } => Err(format!("aborted: {error}")),
    };
    (
        status,
        Metrics {
            inputs: CC_COUNTER.load(Ordering::Relaxed),
            elapsed_us: t0.elapsed().as_micros(),
        },
    )
}

// ---------------------------------------------------------------------------
// hegel (hegeltest 0.3.7)

static HG_COUNTER: AtomicU64 = AtomicU64::new(0);

fn hegel_settings() -> hegel::Settings {
    use hegel::HealthCheck;
    hegel::Settings::new()
        .test_cases(200)
        .suppress_health_check(HealthCheck::all())
}

fn run_hegel_property(property: &str) -> Outcome {
    use hegel::{generators as hgen, Hegel, TestCase};
    if property == "All" {
        return run_all(run_hegel_property);
    }
    HG_COUNTER.store(0, Ordering::Relaxed);
    let t0 = Instant::now();
    let settings = hegel_settings();
    let run_result = std::panic::catch_unwind(AssertUnwindSafe(|| match property {
        "IsMultipleOfZero" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a: u64 = tc.draw(hgen::integers::<u64>());
                let cex = format!("({a})");
                let res =
                    std::panic::catch_unwind(AssertUnwindSafe(|| property_is_multiple_of_zero(a)));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "ScalarDivByZeroPanics" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a: u64 = tc.draw(hgen::integers::<u64>());
                let cex = format!("({a})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_scalar_div_by_zero_panics(a)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "NegIsizeAddAssign" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a: i64 = tc.draw(hgen::integers::<i64>());
                let b: i16 = tc.draw(hgen::integers::<i16>());
                let cex = format!("({a} {b})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_neg_isize_addassign(a, b)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "MulSquareAllOnes" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let bits_tag: u8 = tc.draw(hgen::integers::<u8>());
                let cex = format!("({bits_tag})");
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_mul_square_all_ones(bits_tag)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        "MulDoesNotPanic" => {
            Hegel::new(|tc: TestCase| {
                HG_COUNTER.fetch_add(1, Ordering::Relaxed);
                let a_bytes: Vec<u8> = tc.draw(hgen::vecs(hgen::integers::<u8>()).max_size(256));
                let b_bytes: Vec<u8> = tc.draw(hgen::vecs(hgen::integers::<u8>()).max_size(256));
                let cex = format!("({:?} {:?})", a_bytes, b_bytes);
                let res = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    property_mul_does_not_panic(a_bytes, b_bytes)
                }));
                match res {
                    Ok(PropertyResult::Pass) | Ok(PropertyResult::Discard) => {}
                    Ok(PropertyResult::Fail(_)) | Err(_) => panic!("{cex}"),
                }
            })
            .settings(settings.clone())
            .run();
        }
        _ => panic!("__unknown_property:{property}"),
    }));
    let elapsed_us = t0.elapsed().as_micros();
    let inputs = HG_COUNTER.load(Ordering::Relaxed);
    let status = match run_result {
        Ok(()) => Ok(()),
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "hegel panicked with non-string payload".to_string()
            };
            if let Some(rest) = msg.strip_prefix("__unknown_property:") {
                return (
                    Err(format!("Unknown property for hegel: {rest}")),
                    Metrics::default(),
                );
            }
            Err(msg
                .strip_prefix("Property test failed: ")
                .unwrap_or(&msg)
                .to_string())
        }
    };
    (status, Metrics { inputs, elapsed_us })
}

// ---------------------------------------------------------------------------
// Dispatch + JSON output

fn run(tool: &str, property: &str) -> Outcome {
    match tool {
        "etna" => run_etna_property(property),
        "proptest" => run_proptest_property(property),
        "quickcheck" => run_quickcheck_property(property),
        "crabcheck" => run_crabcheck_property(property),
        "hegel" => run_hegel_property(property),
        _ => (
            Err(format!("Unknown tool: {tool}")),
            Metrics::default(),
        ),
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn emit_json(
    tool: &str,
    property: &str,
    status: &str,
    m: Metrics,
    cex: Option<&str>,
    err: Option<&str>,
) {
    let cex = cex.map_or("null".into(), json_str);
    let err = err.map_or("null".into(), json_str);
    println!(
        "{{\"status\":{},\"tests\":{},\"discards\":0,\"time\":{},\"counterexample\":{},\"error\":{},\"tool\":{},\"property\":{}}}",
        json_str(status),
        m.inputs,
        json_str(&format!("{}us", m.elapsed_us)),
        cex,
        err,
        json_str(tool),
        json_str(property),
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property>", args[0]);
        eprintln!("Tools: etna | proptest | quickcheck | crabcheck | hegel");
        eprintln!(
            "Properties: IsMultipleOfZero | ScalarDivByZeroPanics | NegIsizeAddAssign | MulSquareAllOnes | MulDoesNotPanic | All"
        );
        std::process::exit(2);
    }
    let (tool, property) = (args[1].as_str(), args[2].as_str());

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(AssertUnwindSafe(|| run(tool, property)));
    std::panic::set_hook(prev);

    let (status, m) = match caught {
        Ok(outcome) => outcome,
        Err(p) => {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "adapter panic (non-string payload)".into());
            emit_json(tool, property, "aborted", Metrics::default(), None, Some(&msg));
            return;
        }
    };
    match status {
        Ok(()) => emit_json(tool, property, "passed", m, None, None),
        Err(e) => emit_json(tool, property, "failed", m, Some(&e), None),
    }
}
