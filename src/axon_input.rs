use super::neuron::Neuron;
use id_arena::Id;

pub(crate) trait AxonInput {
  fn get_value(&self) -> f64;
  fn get_id(&self) -> Option<Id<Neuron>>;
}
