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

    let mut i = l.neurons().iter();
    assert_eq!(i.next().unwrap().inputs.len(), 1);
    assert_eq!(i.next().unwrap().inputs.len(), 2);
    assert_eq!(i.next().unwrap().inputs.len(), 3);
    assert!(i.next().is_none());
}
