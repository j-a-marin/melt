use crate::falsifier::Falsifier;
use crate::latent_state::{LatentState, Motion};
use crate::state::WorldState;

pub struct PlanningState {
    pub latent: LatentState,
    pub world_state: WorldState
}

pub trait Beacon {
    
    type Observation;
    type State;
    type Exposure;
    
    fn infer_state(&self, observations: &[Self::Observation]) -> Self::State; 

    fn motion(&self, state: &Self::State) -> Motion;

    fn exposures(&self, state: &Self::State) -> Vec<Self::Exposure>;

    fn falsifiers(&self, state: &Self::State) -> Vec<Falsifier>;
}
