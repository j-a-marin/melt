use crate::action::CandidateAction;

#[derive(Clone, Debug)]
pub struct DecisionRecord {
    pub cycle: u64,
    pub candidates: Vec<CandidateAction>,
    pub selected: Option<CandidateAction>,
}
