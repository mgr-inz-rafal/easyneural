use crate::simulation::SimulationStatus;

/// Represents a structure that handles the logic
/// that is about to be learned by the neural network.
///
/// Use this trait to build a structure that will run the logic
/// for the neural network to be learned upon.
/// The general idea is to use the following feedback loop for training:
/// 1. "Ask" you world for its current state
/// 2. Pass this state to the network
/// 3. Let the network decide how to respond to given state
/// 4. Let the world know what the network decided and let it act accordingly
/// 5. go to 1
pub trait SimulatingWorld {
    fn new() -> Self;

    /// Gives feedback from the network to the world
    ///
    /// `easyneural` will call this function regularly, passing in the current
    /// response from the neural network. Implementation should run any
    /// simulation necessary and return correct status.
    fn tick(&mut self, input: &[f64]) -> SimulationStatus;

    /// Gives feedback from the world to the network
    ///
    /// `easyneural` will call this function regularly in order
    /// to retrieve current world state. These will be used as an input
    /// to the neural network being trained.
    fn get_world_state(&self) -> Vec<f64>; // TODO: &[f64]
}
