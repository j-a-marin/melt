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
