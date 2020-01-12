use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Axon {
    pub left: usize,
    weight: f64,
}

impl Axon {
    pub(crate) fn new(left: usize, weight: f64) -> Axon {
        Axon { left, weight }
    }

    #[allow(dead_code)]
    pub(crate) fn get_id(&self) -> usize {
        self.left
    }

    #[allow(dead_code)]
    pub(crate) fn get_weight(&self) -> f64 {
        self.weight
    }
}
