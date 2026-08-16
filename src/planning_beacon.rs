use crate::beacon::Beacon;
use crate::falsifier::Falsifier;
use crate::latent_state::Motion;

pub struct PlanningBeacon;

impl Beacon for PlanningBeacon {
    type Observation = ();
    type State = ();
    type Exposure = ();

    fn observations(&self, observation: &[Self::Observation]) -> Self::State {
        todo!()
    }

    fn motion(&self, state: &Self::State) -> Motion {
        todo!()
    }

    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure> {
        Vec::new()
    }

    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier> {
        todo!()
    }
}
