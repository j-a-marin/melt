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
use planner::select_best;
use policy::{Policy, RescuePolicy};
use render::render_cycle;
use sensor::{MockDrone, MockWaterSensor, Sensor};
use state::WorldState;

fn main() {
    let mut drone = MockDrone::new("drone-01", 34.0219, -118.4814);
    let mut water_sensor = MockWaterSensor::new("water-01", 34.0195, -118.4900);
    let mut world = WorldState::new();
    let constraint = Constraint::MinimumConfidence(0.88);
    let policy = RescuePolicy;

    for cycle in 1..=6 {
        let survivor_observation = drone.observe();
        let water_observation = water_sensor.observe();
        world.update(survivor_observation);
        world.update(water_observation);

        let mut candidates = world.derive_candidates();

        for candidate in &mut candidates {
            candidate.evaluate_feasibility(&constraint);
            if candidate.feasible == Some(true) {
                let score = policy.score(candidate, &world);
                candidate.score = Some(score);
            }
        }
        // println!("{world:#?}");
        let selected = select_best(&candidates);

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
