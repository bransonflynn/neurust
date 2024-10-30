#[allow(dead_code)]
#[allow(unused_variables)]
pub mod neu {
    pub struct Neuron {
        id: u64,
        weights: Vec<i32>,
        bias: i32,
    }
    impl Neuron {
        pub fn feed_forward(&self, inputs: Vec<i32>) {
            //let total = dot_bound(self.weights, input);
        }

        pub fn id(&self) -> u64 {
            self.id
        }
    }

    pub fn create(a_weights: Vec<i32>, a_bias: i32) -> Neuron {
        let result: Neuron = Neuron {
            id: 0, // todo setup static counter for ID
            weights: a_weights,
            bias: a_bias,
        };
        //bump_id_counter();
        result
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sigmoid(x: f64) -> f64 {
    // todo - doesnt work
    let result: f64 = 1.0 / (1.0 + (-x).exp());
    result
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub mod net {
    use std::collections::HashMap;

    use super::neu;

    pub struct NeuralNetwork {
        id: u64,
        neurons_map: HashMap<u64, neu::Neuron>,
    }
    impl NeuralNetwork {
        pub fn add_neuron(&mut self, neur: neu::Neuron) {
            self.neurons_map.insert(neur.id(), neur);
        }

        pub fn add_neurons(&mut self, neurons: Vec<neu::Neuron>) {
            for neur in neurons {
                if !self.neurons_map.contains_key(&neur.id()) {
                    self.neurons_map.insert(neur.id(), neur);
                }
            }
        }

        pub fn get_neuron(&self, id: u64) -> Option<&neu::Neuron> {
            if self.neurons_map.contains_key(&id) {
                let result: Option<&neu::Neuron> = self.neurons_map.get(&id);
                match result {
                    Some(neur) => return Some(neur),
                    None => return None,
                }
            }
            None
        }

        pub fn has_neuron(&self, id: u64) -> bool {
            if self.neurons_map.contains_key(&id) {
                return true;
            }
            false
        }
    }

    pub fn create() -> NeuralNetwork {
        let neurons_map_new: HashMap<u64, neu::Neuron> = HashMap::new();
        let result: NeuralNetwork = NeuralNetwork {
            id: 0, // todo setup static counter for ID
            neurons_map: neurons_map_new,
        };
        //bump_id_counter();
        result
    }
}

pub unsafe trait Identifier {
    fn bump_id_counter(self);

    fn get_id_counter(self) -> u64;
}
