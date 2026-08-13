mod action;
mod observation;
mod sensor;
mod state;

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

        let action = world.next_action();

        println!("{world:#?}");
        println!("NEXT ACTION: {action:#?}");
    }
}
