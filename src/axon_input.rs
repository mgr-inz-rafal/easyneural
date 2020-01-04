#[typetag::serde(tag = "type")]
pub(crate) trait AxonInput {
  fn get_value(&self) -> f64;
  fn get_id(&self) -> Option<usize>;
}
