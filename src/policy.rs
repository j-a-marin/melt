use crate::action::{Action, CandidateAction};
use crate::state::WorldState;

pub trait Policy {
    fn score(&self, candidate: &CandidateAction, world: &WorldState) -> f64;
}

pub struct RescuePolicy;

impl Policy for RescuePolicy {
    fn score(&self, candidate: &CandidateAction, world: &WorldState) -> f64 {
        match &candidate.action {
            Action::InvestigateSurvivorSignal { .. } => 1.0 * candidate.confidence,
            Action::InvestigateWater { .. } => 0.7 * candidate.confidence,
            Action::Hold => 0.1,
        }
    }
}
