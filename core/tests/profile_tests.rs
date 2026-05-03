
            // SPDX-License-Identifier: Apache-2.0
            //
            // Copyright (c) 2025 RTVLAS contributors

            use rtvlas_core::{default_profile, evaluate_scenario, nominal_snapshot, EvidenceBundle, TrustVerdict, write_evidence_bundle};
            use std::fs;

#[test]
fn decision_latency_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "decision_latency")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.decision_latency_ms = 410.0;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "decision_latency");
}

#[test]
fn operator_intent_alignment_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "operator_intent_alignment")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.operator_intent_alignment = 0.74;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "operator_intent_alignment");
}

#[test]
fn evidence_completeness_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "evidence_completeness")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.evidence_completeness = 0.79;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "evidence_completeness");
}

#[test]
fn mission_plan_validity_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "mission_plan_validity")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.mission_plan_valid = false;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "mission_plan_validity");
}

#[test]
fn temporal_continuity_raises_signal() {
    let profile = default_profile();
    let index = profile
        .properties
        .iter()
        .position(|property| property.key == "temporal_continuity")
        .expect("property present");
    let property = profile.properties[index].clone();
    let mut snapshot = nominal_snapshot();
    snapshot.trust_inputs.temporal_skew_ms = 58.0;
    let outcome = property.evaluate(&snapshot);
    assert!(matches!(outcome.verdict, TrustVerdict::Flag | TrustVerdict::Reject));
    assert_eq!(outcome.property_key, "temporal_continuity");
}

            #[test]
            fn evidence_pipeline_writes_expected_files() {
                let profile = default_profile();
                let scenario_name = "test_scenario";
                let snapshots = vec![nominal_snapshot(), nominal_snapshot()];
                let (timeline, scorecard) = evaluate_scenario(profile, scenario_name, &snapshots);
                let bundle = EvidenceBundle { timeline, scorecard };
                let temp_dir = std::env::temp_dir().join("rtvlas_phase1_evidence");
                let _ = fs::remove_dir_all(&temp_dir);
                fs::create_dir_all(&temp_dir).expect("temp dir");
                let input_log = temp_dir.join("input.jsonl");
                fs::write(
                    &input_log,
                    snapshots
                        .iter()
                        .map(|snapshot| serde_json::to_string(snapshot).expect("json"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("input log");
                write_evidence_bundle(&temp_dir, &input_log, &snapshots, &bundle).expect("evidence bundle");
                assert!(temp_dir.join("trust_scorecard.json").exists());
                assert!(temp_dir.join("timeline.json").exists());
                assert!(temp_dir.join("proof_log.txt").exists());
                assert!(temp_dir.join("trace.svg").exists());
            }

            #[test]
            fn reject_path_drops_trust() {
                let mut snapshot = nominal_snapshot();
                snapshot.trust_inputs.decision_latency_ms = 720.0;
                let (timeline, scorecard) = evaluate_scenario(default_profile(), "reject_case", &[snapshot]);
                assert_eq!(timeline.len(), 1);
                assert!(scorecard.final_trust_score < 1.0);
                assert!(scorecard.reject_frames >= 1 || scorecard.flag_frames >= 1);
            }
