use crate::action::CandidateAction;

pub fn select_best(candidates: &[CandidateAction]) -> Option<&CandidateAction> {
    candidates
        .iter()
        .filter(|candidate| candidate.feasible == Some(true))
        .filter(|candidate| candidate.score.is_some())
        .max_by(|a, b| {
            a.score
                .expect("filtered to scored candidates")
                .partial_cmp(&b.score.expect("scores should not be NaN"))
                .expect("scores should not be NaN")
        })
}
