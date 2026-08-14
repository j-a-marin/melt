use crate::observation::{Observation, ObservationPayload};

pub trait Sensor {
    fn observe(&mut self) -> Observation;
}

#[derive(Debug)]
pub struct MockDrone {
    pub id: String,
    pub x: f64,
    pub y: f64,
    step: u64,
}

impl MockDrone {
    pub fn new(id: &str, x: f64, y: f64) -> Self {
        Self {
            id: id.to_string(),
            x,
            y,
            step: 0,
        }
    }
}

#[derive(Debug)]
pub struct MockWaterSensor {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub step: u64,
}
impl MockWaterSensor {
    pub fn new(id: &str, x: f64, y: f64) -> Self {
        Self {
            id: id.to_string(),
            x,
            y,
            step: 0,
        }
    }
}

impl Sensor for MockWaterSensor {
    fn observe(&mut self) -> Observation {
        self.step += 1;
        let surface_temp_c = 18.0 + (self.step % 3) as f64;
        let turbidity = match self.step % 4 {
            0 => 0.20,
            1 => 0.35,
            2 => 0.75,
            _ => 0.50,
        };

        let confidence = match self.step {
            2 => 0.85,
            6 => 0.92,
            _ => 0.85,
        };

        let spectral_anomaly = match self.step % 5 {
            0 => 0.10,
            1 => 0.25,
            2 => 0.80,
            3 => 0.40,
            _ => 0.15,
        };

        Observation {
            source_id: self.id.clone(),
            timestamp: self.step,
            x: self.x,
            y: self.y,
            confidence,
            payload: ObservationPayload::WaterReading {
                surface_temp_c,
                turbidity,
                spectral_anomaly,
            },
        }
    }
}
impl Sensor for MockDrone {
    fn observe(&mut self) -> Observation {
        self.step += 1;

        let signal = match self.step % 4 {
            0 => 0.15,
            1 => 0.42,
            2 => 0.81,
            _ => 0.63,
        };

        Observation {
            source_id: self.id.clone(),
            timestamp: self.step,
            x: self.x,
            y: self.y,
            confidence: 0.90,
            payload: ObservationPayload::SurvivorSignal { strength: signal },
        }
    }
}
