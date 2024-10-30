pub mod neurust;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let neu1: neurust::neu::Neuron = neurust::neu::create(vec![2, 3], 4);
    let mut net1: neurust::net::NeuralNetwork = neurust::net::create();
    net1.add_neuron(neu1);
    std::println!("net1 has neuron id:0 -> {}", net1.has_neuron(0));
}
