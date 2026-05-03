# Solicitation Alignment

## Topic Basis

- **Track posture:** Direct-to-Phase-II style topic (confirm against official DSIP release at submission time)
- **Source basis:** Public pre-release mirrors for DAF26BZ01-DV007; validate final wording against the official DSIP topic PDF before submission.
- **Objective summary:** Provide a common operational review capability that fuses ACP and human decision data into shared playback, event, map, and status products for mission debrief and trust assessment.

## What This Repository Intentionally Covers

- common ingest for autonomy outputs, human interaction records, and mission context
- map, timeline, event-log, and status playback for post-mission review
- operator-facing reconstruction of why a collaborative autonomy sequence succeeded or failed
- data format and visualization discipline that reduces bespoke debrief tooling burden

## How The Repository Maps To The Topic

| Solicitation Need | Repository Response |
| --- | --- |
| Topic-specific runtime checks | `core/src/profile.rs` encodes five topic-shaped trust properties tied to this mission area. |
| Repeatable proof and replay | `tooling/replay`, `tooling/eval`, `evidence/`, and `package_manifest.json` provide deterministic reproduction. |
| Integration path | `bindings/include/rt_vlas.h` and `bindings/src/lib.rs` define the C ABI boundary for autonomy-stack insertion. |
| Reviewer-verifiable evidence | `evidence/scorecard_summary.md`, `proof_log.txt`, `timeline.json`, and `trace.svg` make the behavior inspectable. |
| Clear scope discipline | This repository is scoped as: This repository focuses on evidence ingest, timeline reconstruction, and reviewer playback rather than developing new autonomy behaviors. |

## What The Package Is Not Claiming

- it is not a replacement for the underlying autonomy stack
- it is not a certification package
- it is not based on classified program data
- it is not claiming operational fielding approval

## Why The Current Shape Is Credible

The strongest near-term value of RTVLAS is the ability to make autonomy behavior observable,
explainable, and rejectable when it drifts outside mission or safety expectations. That is the
thread this repository follows for this specific topic.
