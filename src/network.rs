use super::layer::Layer;

pub struct Network<'a> {
    layers: Vec<Layer<'a>>,
}

impl<'a> Network<'a> {
    pub fn new() -> Network<'a> {
        Network { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Layer<'a>) {
        self.layers.push(layer);
    }

    pub fn layers(&self) -> &Vec<Layer<'a>> {
        &self.layers
    }
}
