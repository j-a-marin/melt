use crate::falsifier::Falsifier;
use crate::latent_state::Motion;

pub trait Beacon {
    type Observation;
    type State;
    type Exposure;

    fn observations(&self, observation: &[Self::Observation]) -> Self::State;

    fn motion(&self, state: &Self::State) -> Motion;

    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure>;

    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier>;
}
