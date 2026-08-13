mod action;
mod constraint;
mod observation;
mod sensor;
mod state;

use crate::constraint::Constraint;
use sensor::{MockDrone, MockWaterSensor, Sensor};
use state::WorldState;

fn main() {
    let mut drone = MockDrone::new("drone-01", 34.0219, -118.4814);
    let mut water_sensor = MockWaterSensor::new("water-01", 34.0195, -118.4900);
    let mut world = WorldState::new();

    for _ in 0..5 {
        let survivor_observation = drone.observe();
        let water_observation = water_sensor.observe();
        world.update(survivor_observation);
        world.update(water_observation);

        let mut candidates = world.generate_candidates();

        println!("{world:#?}");

        let constraint = Constraint::MinimumConfidence(0.88);

        for candidate in &mut candidates {
            candidate.evaluate_feasibility(&constraint);
        }
        println!("CANDIDATE ACTIONS: {candidates:#?}");
    }
}
