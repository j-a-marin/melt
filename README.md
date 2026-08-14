# MELT

**Trust the Physics.**

MELT is a constrained planning framework in Rust for multisensor environments. It turns a
stream of noisy sensor readings into a single explainable decision per cycle — and shows its
work, so an operator can see not just what was chosen but what was rejected and why.

## Pipeline

```
   Sensors            drones, water sensors, ... → Observation
      │
      ▼
  Observations        timestamped, confidence-tagged readings
      │
      ▼
  World State         latest reading per source
      │
      ▼
  Candidate Gen       what could we do about this?
      │
      ▼
  Feasibility         hard constraints: is it allowed?
      │
      ▼
  Policy Scoring      soft preferences: how good is it?
      │
      ▼
  Planner Selection   highest-scoring feasible candidate
      │
      ▼
  Decision Feed       operator-readable output
```

Feasibility and scoring are deliberately separate stages. A constraint can veto an action
outright; a policy only ranks what survives. That split is what keeps the "why" recoverable.

## Try it

Requires Rust 1.85+ (edition 2024).

```bash
git clone git@github.com:j-a-marin/melt.git
cd melt
cargo run
```

Six decision cycles run against scripted mock sensors. Excerpt:

```
======================
MELT DECISION CYCLE #2
======================

2 observations received

Candidates:
  ✓ InvestigateSurvivorSignal(drone-01) score=0.90
  ✗ InvestigateWater(water-01)     infeasible (confidence=0.85)

Decision:
  InvestigateSurvivorSignal(drone-01) (score=0.90, confidence=0.90)
  reason: survivor signal detected: strength=0.81
```

Cycle 2 shows a veto: the water sensor spotted a real anomaly, but at 0.85 confidence it
fell under the 0.88 minimum, so it never reached scoring. By cycle 6 both candidates clear
the bar and the planner ranks them — survivor signal at 0.90 over water at 0.64.

## Layout

| Module | Responsibility |
| --- | --- |
| `sensor.rs` | `Sensor` trait; mock drone and water sensors emitting observations |
| `observation.rs` | `Observation` and its `ObservationPayload` variants |
| `state.rs` | `WorldState` — latest reading per source; generates candidates |
| `action.rs` | `Action` and the `CandidateAction` lifecycle |
| `constraint.rs` | Hard constraints applied during feasibility |
| `policy.rs` | `Policy` trait; `RescuePolicy` scoring |
| `planner.rs` | Selects the best feasible candidate |
| `render.rs` | Operator-facing decision feed |

## Status

Early prototype. The pipeline runs end to end, but sensors are scripted mocks rather than
real inputs, and each stage currently has one implementation:

- `Constraint` has a single variant, `MinimumConfidence` — no spatial, temporal, or resource limits yet
- `Weather`, `Wifi`, and `PowerGrid` observation payloads are defined but not yet consumed by candidate generation
- `RescuePolicy` scores by action type and confidence; it doesn't yet consult world state
- No test suite

The near-term goal is a real search-and-rescue example driving the same pipeline from
recorded sensor data.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
