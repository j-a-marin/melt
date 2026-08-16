#[derive(Debug, Clone)]
pub struct LatentState {
    pub confidence: f64,
    pub persistence: f64,
    pub motion: Motion,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
    Strengthening,
    Weakening,
    Stable,
    /// Insufficient data to assign motion - not informative
    Unassessed,
    /// A detected regime break - highly informative
    Discontinuous,
}

