///static mut ID_COUNTER_NEURON: u64 = 0;
//static mut ID_COUNTER_NEURALNETWORK: u64 = 0;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Neuron {
    id: u64,
    weights: Vec<i32>,
    bias: i32,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Neuron {
    pub fn feed_forward(&self, inputs: Vec<i32>) {
        //let total = dot_bound(self.weights, input);
    }
}

pub fn make_neuron(a_weights: Vec<i32>, a_bias: i32) -> Neuron {
    let result: Neuron = Neuron {
        id: 0, // todo setup static counter for ID
        weights: a_weights,
        bias: a_bias,
    };
    //mod_id_count();
    result
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sigmoid(x: f64) -> f64 {
    // todo - doesnt work
    1.0 / (1.0 + (-x).exp())
}

#[allow(dead_code)]
pub struct NeuralNetwork {
    id: u64,
    neurons_map: HashMap<u64, Neuron>,
}
impl NeuralNetwork {
    pub fn make_neural_network() {}

    pub fn add_neuron(&mut self, neur: Neuron) {
        self.neurons_map.insert(neur.id, neur);
    }

    pub fn get_neuron(&self, id: u64) -> Option<&Neuron> {
        if self.neurons_map.contains_key(&id) {
            let result: Option<&Neuron> = self.neurons_map.get(&id);
            match result {
                Some(neur) => return Some(neur),
                None => return None,
            }
        }
        None
    }
}

pub unsafe trait Identifier {
    fn mod_id_count(self);

    fn get_id_count(self) -> u64;
}
