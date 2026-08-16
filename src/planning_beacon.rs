use crate::beacon::Beacon;
use crate::falsifier::Falsifier;
use crate::latent_state::Motion;
use crate::observation::ObservationPayload;
use crate::state::WorldState;
const EPSILON: f64 = 0.05;

pub struct PlanningBeacon;

impl Beacon for PlanningBeacon {
    type Observation = crate::observation::Observation;
    type State = crate::state::WorldState;
    type Exposure = String;

    fn infer_state(&self, observations: &[Self::Observation]) -> Self::State {
        let mut world = WorldState::new();
        for observation in observations {
            world.update(observation.clone());
        }
        world
    }

    fn motion(&self, state: &Self::State) -> Motion {
        match state.last_two("drone-01") {
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
                _ => Motion::Unassessed,
            },
        }
    }

    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure> {
        Vec::new()
    }

    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier> {
        let mut falsifiers = Vec::new();

        for observation in state.latest.values() {
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
