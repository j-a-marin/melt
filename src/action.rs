#[derive(Debug)]
pub enum Action {
    InvestigateWater { source_id: String, reason: String },
    InvestigateSurvivorSignal { source_id: String, reason: String },
    Hold,
}
