use crate::constraint::Constraint;

#[derive(Debug, Clone)]
pub enum Action {
    InvestigateWater { source_id: String, reason: String },
    InvestigateSurvivorSignal { source_id: String, reason: String },
    Hold,
}

#[derive(Debug, Clone)]
pub struct CandidateAction {
    pub action: Action,
    pub confidence: f64,
    pub score: Option<f64>,
    pub feasible: Option<bool>,
}

impl CandidateAction {
    pub fn evaluate_feasibility(&mut self, constraint: &Constraint) {
        match constraint {
            Constraint::MinimumConfidence(minimum) => {
                self.feasible = Some(self.confidence >= *minimum);
            }
        }
    }
}
