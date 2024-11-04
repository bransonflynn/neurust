pub mod neurust;

fn main() {
    let mut network: neurust::NeuralNetwork = neurust::create_network("test_net");
    let neu0: neurust::Neuron = neurust::create_neuron(vec![0, 1], 0);
    let neu1: neurust::Neuron = neurust::create_neuron(vec![0, 1], 0);
    network.add_neuron(neu0);
    network.add_neuron(neu1);

    
}
