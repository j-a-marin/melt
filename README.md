# MELT

**Trust the Physics.**

MELT is a Rust framework for maintaining a continuously updated model of a domain from noisy
multisensor evidence, and turning that model into explainable decisions.

It has two halves. A **Beacon** infers latent state from a stream of observations, tracks how
that state is moving, and names what would change its mind. A **planning pipeline** turns the
inferred state into one decision per cycle — and shows its work, so an operator can see not
just what was chosen but what was rejected and why.

## The Beacon contract

A Beacon is anything that can answer four questions about a domain it watches:

```rust
pub trait Beacon {
    type Observation;
    type State;
    type Exposure;

    fn infer_state(&self, observations: &[Self::Observation]) -> Self::State;
    fn motion(&self, state: &Self::State) -> Motion;
    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure>;
    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier>;
}
```

| | |
| --- | --- |
| **STATE** | What appears to be happening, and which observations support that? |
| **MOTION** | Is it strengthening, weakening, stable, unassessed, or discontinuous? |
| **EXPOSURE** | Where does this state have consequences? |
| **FALSIFIER** | What observation would make us change our mind — and by when? |

The distinction the design rests on is that **measurements are not reality**. `WorldState` holds
what was observed; `LatentState` holds what is inferred, along with the evidence behind it:

```rust
pub struct LatentState {
    pub confidence: f64,
    pub persistence: u64,
    pub motion: Motion,
    /// EVIDENCE: which observations support this inference.
    pub lineage: Vec<ObservationId>,
}
```

`persistence` is measured in **patterns, not seconds** — the number of consecutive pairwise
classifications that agree with the current motion. Two readings make one classification, so
the pattern is one tick old; a change in motion resets the count. `Motion` separates
`Unassessed` ("no basis to judge") from `Discontinuous` ("the regime broke"), because those are
different statements and collapsing them loses information.

`PlanningBeacon` is the reference implementation, and the only one so far.

## Pipeline

```
   Sensors            drones, water sensors, ... → Observation
      │
      ▼
  Observations        timestamped, confidence-tagged readings
      │
      ▼
  World State         full history, plus the latest reading per source
      │
      ▼
  Latent State        inferred: confidence, persistence, motion, evidence
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
outright; a policy only ranks what survives. That split is what keeps the "why" recoverable —
rejections stay categorical instead of being buried inside a score.

## Try it

Requires Rust 1.85+ (edition 2024).

```bash
git clone git@github.com:j-a-marin/melt.git
cd melt
cargo run
cargo test
```

Six decision cycles run against scripted mock sensors. Excerpt:

```
motion: Strengthening falsifiers open: 1
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

Cycle 2 shows a veto: the water sensor spotted a real anomaly, but at 0.85 confidence it fell
under the 0.88 minimum, so it never reached scoring. By cycle 6 both candidates clear the bar
and the planner ranks them — survivor signal at 0.90 over water at 0.64.

## Layout

**Beacon layer**

| Module | Responsibility |
| --- | --- |
| `beacon.rs` | the `Beacon` trait — the contract above |
| `planning_beacon.rs` | `PlanningBeacon`: motion classification, persistence, lineage |
| `latent_state.rs` | `LatentState`, `Motion`, `ObservationId` |
| `falsifier.rs` | `Falsifier` — a claim plus the deadline it expires on |
| `exposure.rs` | placeholder; no concrete exposure type yet |
| `transmission.rs` | `NodeId`, `Transmission` — edges for a future beacon graph |

**Observations and state**

| Module | Responsibility |
| --- | --- |
| `observation.rs` | `Observation` and its `ObservationPayload` variants |
| `sensor.rs` | `Sensor` trait; mock drone and water sensors |
| `ingress.rs` | `ObservationIngress` — normalizing outside reports into observations |
| `state.rs` | `WorldState` — observation history and latest-per-source; generates candidates |

**Planning pipeline**

| Module | Responsibility |
| --- | --- |
| `action.rs` | `Action` and the `CandidateAction` lifecycle |
| `constraint.rs` | Hard constraints applied during feasibility |
| `policy.rs` | `Policy` trait; `RescuePolicy` scoring |
| `planner.rs` | Selects the best feasible candidate |
| `decision_record.rs` | `DecisionRecord` — what was chosen, what was rejected |
| `render.rs` | Operator-facing decision feed |

## Status

Early prototype. The pipeline runs end to end and 12 tests cover motion classification and
persistence, but sensors are scripted mocks rather than real inputs, and several parts of the
contract above are declared rather than implemented.

Known and deliberate:

- **`exposures()` returns an empty vector.** The EXPOSURE question has no answer yet, and
  `Exposure` is aliased to `String` rather than a real type.
- **Falsifiers are generated but never resolved.** Nothing checks whether a deadline passed
  unmet — which is the half that would make an unmet deadline informative.
- **Timestamps are per-sensor counters, not a shared clock.** Ordering is meaningful only
  within a single source, and `Falsifier::expected_by` inherits that limitation.
- **Motion is derived from one hardcoded source** (`PRIMARY_SOURCE`). A beacon should be
  configured with its sources, not compiled against them.
- **`DecisionRecord` is constructed each cycle and discarded.** It should be retained.
- **`lineage` lists every observation from the primary source**, but the current motion and
  persistence rest only on the unbroken tail — so it currently overstates its evidence.
- `Constraint` has a single variant, `MinimumConfidence` — no spatial, temporal, or resource
  limits, and feasibility takes one constraint rather than a set.
- `RescuePolicy` scores by action type and confidence; it receives world state and ignores it,
  so candidates cannot be ranked against one another.
- `Weather`, `Wifi`, `PowerGrid`, and `HumanFieldReport` payloads are defined but not consumed
  by candidate generation. `ObservationIngress::normalize` is unimplemented.
- `Transmission` and `NodeId` are defined and unused; beacons do not yet compose.

The near-term goal is a real search-and-rescue example driving the same pipeline from recorded
sensor data.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
