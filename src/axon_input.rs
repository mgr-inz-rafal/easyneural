#[typetag::serde(tag = "type")]
pub(crate) trait AxonInput {
    fn get_value(&self) -> f64;
    fn get_id(&self) -> Option<usize>;
    fn get_weight(&self) -> f64;
}
