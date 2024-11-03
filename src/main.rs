pub mod neurust;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let neu0: neurust::Neuron = neurust::create_neuron(vec![2, 3], 4);
    let neu1: neurust::Neuron = neurust::create_neuron(vec![2, 3], 4);
    let neu2: neurust::Neuron = neurust::create_neuron(vec![2, 3], 4);
    let neu3: neurust::Neuron = neurust::create_neuron(vec![2, 3], 4);
    let mut net0: neurust::NeuralNetwork = neurust::create_network();
    net0.add_neuron(neu0.clone());
    std::println!("net1 has neuron id:0 -> {}", net0.has_neuron(0));
    std::println!("net1 has neuron id:1 -> {}", net0.has_neuron(1));

    std::println!("neu0: {}", neu0.display());
    std::println!("neu1: {}", neu1.display());
    std::println!("neu2: {}", neu2.display());
    std::println!("neu3: {}", neu3.display());
    std::println!("net0: {}", net0.display());
}
