use crate::observation::Observation;

pub trait ObservationIngress {
    fn normalize(&self) -> Observation;
}

pub struct HumanFieldReport {
    pub source_id: String,
    pub x: f64,
    pub y: f64,
    pub confidence: f64,
    pub category: String,
}

impl ObservationIngress for HumanFieldReport {
    fn normalize(&self) -> Observation {
        unimplemented!("normalize HumanFieldReport into Observation")
    }
}
