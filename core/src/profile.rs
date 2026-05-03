
        // SPDX-License-Identifier: Apache-2.0
        //
        // Copyright (c) 2025 RTVLAS contributors

        use crate::model::{AutonomySnapshot, BoolField, NumericField, PropertyKind, PropertySpec, TrustInputs};
        use crate::monitor::MonitorProfile;

        pub fn default_profile() -> MonitorProfile {
            MonitorProfile {
                topic_id: "DAF26BZ01-DV007".to_string(),
                title: "CHORD".to_string(),
                framing: "autonomy black-box recorder; decision review layer; operator trust and mission reconstruction tool".to_string(),
                properties: vec![
        PropertySpec::new(
            "decision_latency",
            "Decision Chain Latency Bound",
            "Flags autonomy or human-decision records that arrive too late to remain operationally meaningful during playback and review.",
            PropertyKind::MaxValue { field: NumericField::DecisionLatencyMs, max: 350.0 },
            0.9,
        ),
        PropertySpec::new(
            "operator_intent_alignment",
            "Human-ACP Intent Alignment",
            "Tracks how closely the observed autonomy actions remain aligned with the human commander's stated intent or review rubric.",
            PropertyKind::MinValue { field: NumericField::OperatorIntentAlignment, min: 0.8 },
            1.0,
        ),
        PropertySpec::new(
            "evidence_completeness",
            "Debrief Evidence Completeness",
            "Ensures enough mission context and state are present to support a defensible collaborative autonomy debrief.",
            PropertyKind::MinValue { field: NumericField::EvidenceCompleteness, min: 0.85 },
            1.1,
        ),
        PropertySpec::new(
            "mission_plan_validity",
            "Mission Reconstruction Validity",
            "Detects when the replay chain is operating against an invalidated or incomplete mission-plan model during reconstruction.",
            PropertyKind::BooleanGate { field: BoolField::MissionPlanValid, reject_on_false: true },
            1.2,
        ),
        PropertySpec::new(
            "temporal_continuity",
            "Collaborative Timeline Continuity",
            "Captures dropped, duplicated, or significantly delayed decision events that break collaborative mission timeline continuity.",
            PropertyKind::MaxValue { field: NumericField::TemporalSkewMs, max: 50.0 },
            0.8,
        )
                ],
            }
        }

        pub fn nominal_snapshot() -> AutonomySnapshot {
            AutonomySnapshot {
    timestamp_ms: 0,
    position_m: [0.0, 0.0, 180.0],
    velocity_mps: [22.0, 1.5, 0.0],
    heading_rad: 0.08,
    trust_inputs: TrustInputs {
        gps_valid: true,
        operator_link: true,
        autonomy_solution_feasible: true,
        mission_plan_valid: true,
        emergency_response_ready: true,
        temporal_skew_ms: 12.0,
        corridor_error_m: 8.0,
        corridor_half_width_m: 24.0,
        command_speed_mps: 18.0,
        max_safe_speed_mps: 38.0,
        deconfliction_margin_m: 55.0,
        min_deconfliction_margin_m: 25.0,
        formation_spacing_m: 40.0,
        desired_spacing_m: 40.0,
        heading_error_rad: 0.05,
        threat_distance_m: 76.0,
        threat_min_distance_m: 46.0,
        wez_exposure: 0.18,
        route_efficiency: 0.91,
        decision_latency_ms: 140.0,
        operator_intent_alignment: 0.94,
        evidence_completeness: 0.97,
        hazard_distance_m: 74.0,
        min_hazard_distance_m: 42.0,
        safe_altitude_margin_m: 48.0,
        recovery_zone_distance_m: 920.0,
        max_recovery_zone_distance_m: 1600.0,
        autonomy_solution_optimality: 0.91,
    },
}
        }
