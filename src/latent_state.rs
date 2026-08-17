use crate::observation::Observation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationId {
    pub source_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct LatentState {
    pub confidence: f64,
    pub persistence: f64,
    pub motion: Motion,
    /// EVIDENCE: which observations support this inference.
    pub lineage: Vec<ObservationId>
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Strengthening ,
    Weakening,
    Stable,
    /// Insufficient data to assign motion - not informative
    Unassessed,
    /// A detected regime break - highly informative
    Discontinuous,
}

