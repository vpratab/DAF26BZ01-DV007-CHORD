# Evidence Scorecard Summary

- Topic: `DAF26BZ01-DV007 CHORD`
- Generated: `2026-05-03T04:53:36Z`
- Git Head: `UNCOMMITTED_SCAFFOLD`
- Scenario Pass Rate: `3/3 (100.0%)`
- Evidence Type: `deterministic synthetic autonomy traces for submission-stage feasibility review`

| Scenario | Mode | Result | Final Trust | First Reject | Scorecard |
| --- | --- | --- | --- | --- | --- |
| Clean Human-ACP Debrief | `nominal` | `PASS` | `1.000` | `None` | [scenario_01_clean_mission_review](scenario_01_clean_mission_review/trust_scorecard.json) |
| Commander Intent Divergence | `degraded` | `PASS` | `0.196` | `None` | [scenario_02_operator_mismatch](scenario_02_operator_mismatch/trust_scorecard.json) |
| Broken Debrief Chain | `fault` | `PASS` | `0.000` | `20` | [scenario_03_evidence_gap_chain](scenario_03_evidence_gap_chain/trust_scorecard.json) |

## Notes

- Nominal scenarios are expected to remain fully accepted.
- Degraded scenarios are expected to produce concern signals without hard reject behavior.
- Fault scenarios are expected to produce deterministic reject behavior.
- This summary is generated automatically from the underlying per-scenario scorecards.
