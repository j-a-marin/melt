mod action;
mod constraint;
mod observation;
mod policy;
mod sensor;
mod state;

mod beacon;
mod decision_record;
mod exposure;
mod falsifier;
mod ingress;
mod latent_state;
mod planner;
mod render;
mod transmission;
mod planning_beacon;

use crate::constraint::Constraint;
use beacon::Beacon;
use observation::Observation;
use planner::select_best;
use policy::{Policy, RescuePolicy};
use render::render_cycle;
use sensor::{MockDrone, MockWaterSensor, Sensor};
use planning_beacon::{PlanningBeacon, PRIMARY_SOURCE};

fn main() {
    let mut drone = MockDrone::new(PRIMARY_SOURCE, 34.0219, -118.4814);
    let mut water_sensor = MockWaterSensor::new("water-01", 34.0195, -118.4900);

    let beacon = PlanningBeacon;
    let mut all_observations: Vec<Observation> = Vec::new();
    let constraint = Constraint::MinimumConfidence(0.88);
    let policy = RescuePolicy;

    for cycle in 1..=6 {
        let survivor_observation = drone.observe();
        let water_observation = water_sensor.observe();
        all_observations.push(survivor_observation);
        all_observations.push(water_observation);
        let state = beacon.infer_state(&all_observations);

        let mut candidates = state.world.derive_candidates();

        for candidate in &mut candidates {
            candidate.evaluate_feasibility(&constraint);
            if candidate.feasible == Some(true) {
                let score = policy.score(candidate, &state.world);
                candidate.score = Some(score);
            }
        }
        // println!("{world:#?}");
        let selected = select_best(&candidates);

        println!("motion: {:?} falsifiers open: {}",
            beacon.motion(&state),
            beacon.falsifiers(&state).len());

        use decision_record::DecisionRecord;
        let _record = DecisionRecord {
            cycle,
            candidates: candidates.clone(),
            selected: selected.cloned(),
        };

        let output = render_cycle(cycle, 2, &candidates, selected);

        println!("{output}");
    }
}
