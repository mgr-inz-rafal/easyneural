extern crate easyneural;

use easyneural::layer::Layer;
use easyneural::neuron::NeuronBuilder;

#[test]
fn build_single_layer() {
    let n1 = NeuronBuilder::new().with_inputs(1).build();
    let n2 = NeuronBuilder::new().with_inputs(2).build();
    let n3 = NeuronBuilder::new().with_inputs(3).build();

    let mut l = Layer::new();
    l.add_neuron(&n1);
    l.add_neuron(&n2);
    l.add_neuron(&n3);

    assert_eq!(3, l.neurons().len());
}
