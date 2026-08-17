use crate::beacon::Beacon;
use crate::falsifier::Falsifier;
use crate::latent_state::{LatentState, Motion, ObservationId};
use crate::observation::{Observation, ObservationPayload};
use crate::state::WorldState;
const EPSILON: f64 = 0.05;

pub struct PlanningBeacon;

impl Beacon for PlanningBeacon {
    type Observation = crate::observation::Observation;
    type State = PlanningState;
    type Exposure = String;

    fn infer_state(&self, observations: &[Self::Observation]) -> Self::State {
        let mut world = WorldState::new();
        let mut ordered = observations.to_vec();
        ordered.sort_by_key(|o| o.timestamp);
        for observation in ordered {
            world.update(observation);
        }
        let motion = classify_motion(&world);
        let lineage: Vec<ObservationId> = world
            .last_two("drone-01")
            .map(|(prior, newest)| vec![ObservationId::of(prior), ObservationId::of(newest)])
            .unwrap_or_default();

        let confidence = match world.last_two("drone-01") {
            Some((prior, newest)) => (prior.confidence + newest.confidence) / 2.0,
            None => 0.0,
        };
        PlanningState {
            latent: LatentState {
                confidence,
                persistence: 0.0, // TODO(session 3): ticks the current motion has held
                motion,
                lineage,
            },
            world,
        }
    }

    fn motion(&self, state: &Self::State) -> Motion {
        state.latent.motion
    }


    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure> {
        Vec::new()
    }

    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier> {
        let mut falsifiers = Vec::new();

        for observation in state.world.latest.values() {
            match &observation.payload {
                ObservationPayload::WaterReading {
                    turbidity,
                    spectral_anomaly,
                    ..
                } => {
                    if *turbidity > 0.70 || *spectral_anomaly > 0.70 {
                        falsifiers.push(Falsifier {
                            description: format!(
                                "{}: expect turbidity {:.2} / spectral {:.2} to fall below 0.70",
                                observation.source_id, turbidity, spectral_anomaly,
                            ),
                            expected_by: observation.timestamp + 3,
                        });
                    }
                }
                _ => {}
            }
        }
        falsifiers
    }
}

fn classify_motion(world: &WorldState) -> Motion {
    match world.last_two("drone-01") {
        None => Motion::Unassessed,
        Some((prior, newest)) => match (&prior.payload, &newest.payload) {
            (
                ObservationPayload::SurvivorSignal {
                    strength: prior_strength,
                },
                ObservationPayload::SurvivorSignal {
                    strength: newest_strength,
                },
            ) => {
                if *newest_strength > *prior_strength + EPSILON {
                    Motion::Strengthening
                } else if *newest_strength < *prior_strength - EPSILON {
                    Motion::Weakening
                } else {
                    Motion::Stable
                }
            }
            _ => {
                if std::mem::discriminant(&prior.payload) != std::mem::discriminant(&newest.payload) {
                    Motion::Discontinuous
                } else {
                    Motion::Unassessed
                }
            }
        },
    }
}

pub struct PlanningState {
    pub latent: LatentState,
    pub world: WorldState,
}

#[cfg(test)]
mod tests {
    use crate::observation::Observation;
    use super::*;
    #[test]
    fn motion_is_unassessed_with_no_observations() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[]);
        assert_eq!(beacon.motion(&world), Motion::Unassessed)
    }

    fn obs(timestamp: u64, strength: f64) -> Observation {
        Observation {
            source_id: "drone-01".to_string(),
            timestamp,
            x: 0.0,
            y: 0.0,
            confidence: 0.9,
            payload: ObservationPayload::SurvivorSignal { strength },
        }
    }

    fn obs_water(timestamp: u64) -> Observation {
        Observation {
            source_id: "drone-01".to_string(),
            timestamp,
            x: 0.0,
            y: 0.0,
            confidence: 0.9,
            payload: ObservationPayload::WaterReading {
                surface_temp_c: 18.0,
                turbidity: 0.5,
                spectral_anomaly: 0.5,
            },
        }
    }

    #[test]
    fn motion_strengthening_when_signal_rises() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.40), obs(2, 0.80)]);
        assert_eq!(beacon.motion(&world), Motion::Strengthening)
    }

    #[test]
    fn motion_weakening_when_signal_falls() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.80), obs(2, 0.40)]);
        assert_eq!(beacon.motion(&world), Motion::Weakening)
    }

    #[test]
    fn motion_stable_within_deadband() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.50), obs(2, 0.52)]);
        assert_eq!(beacon.motion(&world), Motion::Stable) // delta 0.02 < EPSILON 0.05 — deadband absorbs jitter
    }

    #[test]
    fn motion_stable_at_exact_epsilon_rising() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.50), obs(2, 0.55)]);
        assert_eq!(beacon.motion(&world), Motion::Stable) // delta exactly EPSILON: strict > means this is Stable, deliberately
    }

    #[test]
    fn motion_stable_at_exact_epsilon_falling() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.55), obs(2, 0.50)]);
        assert_eq!(beacon.motion(&world), Motion::Stable) // delta exactly EPSILON: strict < means this is Stable, deliberately
    }

    #[test]
    fn motion_discontinuous_when_payload_kind_changed() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(1, 0.80), obs_water(2)]);
        assert_eq!(beacon.motion(&world), Motion::Discontinuous)
    }
    #[test]
    fn motion_respects_timestamps_not_arrival_order() {
        let beacon = PlanningBeacon;
        let world = beacon.infer_state(&[obs(2, 0.80), obs(1, 0.40)]);
        assert_eq!(beacon.motion(&world), Motion::Strengthening)
    }
}
