use super::axon_input::AxonInput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Axon {
    pub left: usize,
    weight: f64,
}

impl Axon {
    pub(crate) fn new(left: usize) -> Axon {
        Axon {
            left,
            weight: 12.34,
        }
    }
}

#[typetag::serde]
impl AxonInput for Axon {
    fn get_value(&self) -> f64 {
        123.456
    }

    fn get_id(&self) -> Option<usize> {
        Some(self.left)
    }

    fn get_weight(&self) -> f64 {
        16.6
    }
}
