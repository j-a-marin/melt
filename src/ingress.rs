use crate::observation::Observation;

pub trait ObservationIngress {
    fn normalize(&self) -> Observation;
}
