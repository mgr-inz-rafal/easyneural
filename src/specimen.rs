use crate::network::Network;

pub enum SpecimenStatus {
    ALIVE(f64),
    DEAD(f64),
}

pub(crate) struct Specimen {
    pub brain: Network,
    pub(crate) fitness: f64, // TODO: Move to SpecimenStatus
}

impl Specimen {
    pub(crate) fn tick(&mut self, world_input: &[f64]) -> Vec<f64> {
        self.brain.fire(world_input);
        self.brain.get_output()
    }
}
