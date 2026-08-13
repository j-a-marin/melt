use std::collections::HashMap;

use crate::action::Action;
use crate::observation::{Observation, ObservationPayload};

#[derive(Debug)]
pub struct WorldState {
    pub latest: HashMap<String, Observation>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            latest: HashMap::new(),
        }
    }

    pub fn update(&mut self, observation: Observation) {
        self.latest
            .insert(observation.source_id.clone(), observation);
    }

    pub fn candidate_actions(&self) -> Vec<Action> {
        let mut actions: Vec<Action> = Vec::new();

        for observation in self.latest.values() {
            match &observation.payload {
                ObservationPayload::WaterReading {
                    spectral_anomaly,
                    turbidity,
                    ..
                } => {
                    if observation.confidence >= 0.80
                        && (*spectral_anomaly > 0.70 || *turbidity > 0.70)
                    {
                        actions.push(Action::InvestigateWater {
                            source_id: observation.source_id.clone(),
                            reason: format!(
                                "water anomaly detected: spectral={:.2}, turbidity={:.2}",
                                spectral_anomaly, turbidity,
                            ),
                        });
                    }
                }

                ObservationPayload::SurvivorSignal { strength } => {
                    if observation.confidence >= 0.80 && *strength > 0.75 {
                        actions.push(Action::InvestigateSurvivorSignal {
                            source_id: observation.source_id.clone(),
                            reason: format!("survivor signal detected: strength={:.2}", strength),
                        });
                    }
                }

                _ => {}
            }
        }
        if actions.is_empty() {
            actions.push(Action::Hold);
        }
        actions
    }}
