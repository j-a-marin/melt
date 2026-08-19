use std::collections::BTreeMap;

use crate::action::{Action, CandidateAction};
use crate::observation::{Observation, ObservationPayload};

#[derive(Debug)]
pub struct WorldState {
    pub history: Vec<Observation>,
    pub latest: BTreeMap<String, Observation>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            latest: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    pub fn update(&mut self, observation: Observation) {
        self.history.push(observation.clone());
        self.latest
            .insert(observation.source_id.clone(), observation);

    }

    pub fn derive_candidates(&self) -> Vec<CandidateAction> {
        let mut actions: Vec<CandidateAction> = Vec::new();

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
                        actions.push(CandidateAction {
                            action: Action::InvestigateWater {
                                source_id: observation.source_id.clone(),
                                reason: format!(
                                    "water anomaly detected: spectral={:.2}, turbidity={:.2}",
                                    spectral_anomaly, turbidity,
                                ),
                            },
                            confidence: observation.confidence,
                            score: None,
                            feasible: None,
                        });
                    }
                }

                ObservationPayload::SurvivorSignal { strength } => {
                    if observation.confidence >= 0.80 && *strength > 0.75 {
                        actions.push(CandidateAction {
                            action: Action::InvestigateSurvivorSignal {
                                source_id: observation.source_id.clone(),
                                reason: format!(
                                    "survivor signal detected: strength={:.2}",
                                    strength,
                                ),
                            },
                            confidence: observation.confidence,
                            score: None,
                            feasible: None,
                        });
                    }
                }

                _ => {}
            }
        }

        if actions.is_empty() {
            actions.push(CandidateAction {
                action: Action::Hold,
                confidence: 1.0,
                score: None,
                feasible: None,
            });
        }

        actions
    }
}
