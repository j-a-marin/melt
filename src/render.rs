use std::fmt::Write;

use crate::action::{Action, CandidateAction};

const RULE: &str = "======================";

pub fn render_cycle(
    cycle: u64,
    observation_count: usize,
    candidates: &[CandidateAction],
    selected: Option<&CandidateAction>,
) -> String {
    let mut out = String::new();

    writeln!(out, "{RULE}\nMELT DECISION CYCLE #{cycle}\n{RULE}\n").unwrap();
    writeln!(out, "{observation_count} observations received\n").unwrap();

    writeln!(out, "Candidates:").unwrap();
    for candidate in candidates {
        let name = action_name(&candidate.action);
        match (candidate.feasible, candidate.score) {
            (Some(true), Some(score)) => {
                writeln!(out, "  ✓ {name:<30} score={score:.2}").unwrap();
            }
            (Some(true), None) => {
                writeln!(out, "  ✓ {name:<30} unscored").unwrap();
            }
            (Some(false), _) => {
                writeln!(
                    out,
                    "  ✗ {name:<30} infeasible (confidence={:.2})",
                    candidate.confidence
                )
                .unwrap();
            }
            (None, _) => {
                writeln!(out, "  ? {name:<30} not evaluated").unwrap();
            }
        }
    }

    writeln!(out, "\nDecision:").unwrap();
    match selected {
        Some(candidate) => {
            writeln!(
                out,
                "  {} (score={:.2}, confidence={:.2})",
                action_name(&candidate.action),
                candidate
                    .score
                    .expect("selected candidate must have a score"),
                candidate.confidence
            )
            .unwrap();
            if let Some(reason) = action_reason(&candidate.action) {
                writeln!(out, "  reason: {reason}").unwrap();
            }
        }
        None => {
            writeln!(out, "  no feasible action — holding").unwrap();
        }
    }

    out
}

fn action_name(action: &Action) -> String {
    match action {
        Action::InvestigateWater { source_id, .. } => {
            format!("InvestigateWater({source_id})")
        }
        Action::InvestigateSurvivorSignal { source_id, .. } => {
            format!("InvestigateSurvivorSignal({source_id})")
        }
        Action::Hold => "Hold".to_string(),
    }
}

fn action_reason(action: &Action) -> Option<&str> {
    match action {
        Action::InvestigateWater { reason, .. }
        | Action::InvestigateSurvivorSignal { reason, .. } => Some(reason),
        Action::Hold => None,
    }
}
