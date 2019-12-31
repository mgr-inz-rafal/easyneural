extern crate easyneural;

/*
use easyneural::layer::Layer;
use easyneural::network::Network;
use easyneural::neuron::NeuronBuilder;
*/

/*
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
    assert_eq!(i.next().unwrap().inputs().len(), 1);
    assert_eq!(i.next().unwrap().inputs().len(), 2);
    assert_eq!(i.next().unwrap().inputs().len(), 3);
    assert!(i.next().is_none());
}

#[test]
fn build_network_with_two_layers() {
    let n1 = NeuronBuilder::new().with_inputs(1).build();
    let n2 = NeuronBuilder::new().with_inputs(2).build();
    let n3 = NeuronBuilder::new().with_inputs(3).build();

    let mut l1 = Layer::new();
    l1.add_neuron(&n1);
    l1.add_neuron(&n2);
    let mut l2 = Layer::new();
    l2.add_neuron(&n3);

    let mut n = Network::new();
    n.add_layer(l1);
    n.add_layer(l2);

    assert_eq!(n.layers().len(), 2);
}
*/
