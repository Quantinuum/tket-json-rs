//! Pass roundtrip tests.
use assert_json_diff::assert_json_eq;
use serde_json::Value;
use tket_json_rs::pass::standard::{CliffordSimp, StandardPass, TargetTwoQubitGate};
use tket_json_rs::pass::{BasePass, SequencePass};

const STANDARD_CLIFFORD_SIMP: &str = include_str!("data/pass/standard_clifford_simp.json");
const SEQUENCE: &str = include_str!("data/pass/sequence_clifford_remove.json");
const REPEAT: &str = include_str!("data/pass/repeat_clifford.json");
const REPEAT_WITH_METRIC: &str = include_str!("data/pass/repeat_with_metric_clifford.json");
const REPEAT_UNTIL: &str = include_str!("data/pass/repeat_until_remove_no_mid_measure.json");

#[test]
fn standard_clifford_simp_roundtrip() {
    let initial_json: Value = serde_json::from_str(STANDARD_CLIFFORD_SIMP).unwrap();
    let pass: BasePass = serde_json::from_value(initial_json.clone()).unwrap();

    assert!(matches!(
        &pass,
        BasePass::StandardPass {
            pass: StandardPass::CliffordSimp(CliffordSimp {
                allow_swaps: true,
                target_2qb_gate: TargetTwoQubitGate::CX,
            }),
        }
    ));

    let reencoded_json = serde_json::to_value(&pass).unwrap();
    assert_json_eq!(reencoded_json, initial_json);

    let roundtrip: BasePass = serde_json::from_value(reencoded_json).unwrap();
    assert_eq!(pass, roundtrip);
}

#[test]
fn sequence_clifford_remove_roundtrip() {
    let initial_json: Value = serde_json::from_str(SEQUENCE).unwrap();
    let pass: BasePass = serde_json::from_value(initial_json.clone()).unwrap();

    assert!(matches!(
        &pass,
        BasePass::SequencePass {
            pass: SequencePass { sequence }
        } if sequence.len() == 2
    ));

    let reencoded_json = serde_json::to_value(&pass).unwrap();
    assert_json_eq!(reencoded_json, initial_json);

    let roundtrip: BasePass = serde_json::from_value(reencoded_json).unwrap();
    assert_eq!(pass, roundtrip);
}

#[test]
fn repeat_clifford_roundtrip() {
    let initial_json: Value = serde_json::from_str(REPEAT).unwrap();
    let pass: BasePass = serde_json::from_value(initial_json.clone()).unwrap();

    assert!(matches!(&pass, BasePass::RepeatPass { .. }));

    let reencoded_json = serde_json::to_value(&pass).unwrap();
    assert_json_eq!(reencoded_json, initial_json);

    let roundtrip: BasePass = serde_json::from_value(reencoded_json).unwrap();
    assert_eq!(pass, roundtrip);
}

#[test]
fn repeat_with_metric_clifford_roundtrip() {
    let initial_json: Value = serde_json::from_str(REPEAT_WITH_METRIC).unwrap();
    let pass: BasePass = serde_json::from_value(initial_json.clone()).unwrap();

    assert!(matches!(&pass, BasePass::RepeatWithMetricPass { .. }));

    let reencoded_json = serde_json::to_value(&pass).unwrap();
    assert_json_eq!(reencoded_json, initial_json);
}

#[test]
fn repeat_until_remove_no_mid_measure_roundtrip() {
    let initial_json: Value = serde_json::from_str(REPEAT_UNTIL).unwrap();
    let pass: BasePass = serde_json::from_value(initial_json.clone()).unwrap();

    assert!(matches!(&pass, BasePass::RepeatUntilSatisfiedPass { .. }));

    let reencoded_json = serde_json::to_value(&pass).unwrap();
    assert_json_eq!(reencoded_json, initial_json);
}
