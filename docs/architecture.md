
# Architecture

This repository adapts RTVLAS for **DAF26BZ01-DV007 CHORD**.

## System Role

**Opening angle:** autonomy black-box recorder; decision review layer; operator trust and mission reconstruction tool

```mermaid
flowchart LR
    A["Autonomy State Snapshot"] --> B["RTVLAS Monitor"]
    B --> C["Topic-Specific Property Set"]
    C --> D["Trust Verdict"]
    D --> E["Structured Evidence Logger"]
    D --> F["Replay / Evaluation Tooling"]
    E --> G["Proposal Evidence Bundle"]
```

## Runtime Elements

- `core/`: monitor, property framework, evidence writer
- `bindings/`: C ABI for external autonomy stacks
- `tooling/replay/`: deterministic replay of autonomy traces
- `tooling/eval/`: scenario evaluator and artifact generation
- `evidence/`: pre-generated scenario outputs for reviewers

## Topic Adaptation

The property set in this repository is tuned for:

- Decision Chain Latency Bound
- Human-ACP Intent Alignment
- Debrief Evidence Completeness
- Mission Reconstruction Validity
- Collaborative Timeline Continuity
