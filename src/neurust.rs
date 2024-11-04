use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::SeqCst;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Neuron {
    pub id: u64,
    weights: Vec<i32>,
    bias: i32,
}
impl Neuron {
    pub fn feed_forward(&self, _inputs: Vec<i32>) {
        //let total = dot_bound(self.weights, input);
    }

    pub fn display(&self) -> String {
        return "Neuron{id=".to_owned()
            + &self.id.to_string()
            + ", weights="
            + &format!("{:?}", self.weights)
            + ", bias="
            + &self.bias.to_string()
            + "}";
    }
}

pub fn create_neuron(w: Vec<i32>, b: i32) -> Neuron {
    return Neuron {
        id: unique_id_neuron(),
        weights: w,
        bias: b,
    };
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NeuralNetwork {
    pub id: u64,
    name: String,
    neurons_map: HashMap<u64, Neuron>,
}
impl NeuralNetwork {
    pub fn add_neuron(&mut self, neur: Neuron) {
        if !self.neurons_map.contains_key(&neur.id) {
            self.neurons_map.insert(neur.id, neur);
        }
    }

    pub fn add_neurons(&mut self, neurons: Vec<Neuron>) {
        for neur in neurons {
            if !self.neurons_map.contains_key(&neur.id) {
                self.neurons_map.insert(neur.id, neur);
            }
        }
    }

    pub fn get_neuron(&self, id: u64) -> Option<&Neuron> {
        if self.neurons_map.contains_key(&id) {
            match self.neurons_map.get(&id) {
                Some(neur) => return Some(neur),
                None => return None,
            }
        } else {
            return None;
        }
    }

    pub fn has_neuron(&self, id: u64) -> bool {
        return self.neurons_map.contains_key(&id);
    }

    pub fn feed_forward(&self, _input: Vec<i32>) {
        //
    }

    pub fn display(&self) -> String {
        return ("NeuralNetwork{id=".to_owned()
            + &self.id.to_string()
            + ", name="
            + &self.name
            + ", neurons_map_keys_len="
            + &self.neurons_map.keys().len().to_string()
            + "}")
            .to_string();
    }
}

pub fn create_network(n: &str) -> NeuralNetwork {
    return NeuralNetwork {
        id: unique_id_network(),
        name: n.to_string(),
        neurons_map: HashMap::new(),
    };
}

pub fn unique_id_neuron() -> u64 {
    static NEURON_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
    let id: u64 = NEURON_ID_COUNTER.fetch_add(1, SeqCst);
    assert_ne!(
        id,
        u64::MAX,
        "Network ID counter has overflowed and is no longer unique"
    );
    return id;
}

pub fn unique_id_network() -> u64 {
    static NETWORK_ID_COUNTER: AtomicU64 = AtomicU64::new(0);
    let id: u64 = NETWORK_ID_COUNTER.fetch_add(1, SeqCst);
    assert_ne!(
        id,
        u64::MAX,
        "Network ID counter has overflowed and is no longer unique"
    );
    return id;
}

pub fn sigmoid(v: f64) -> f64 {
    if v < -40.0 {
        0.0
    } else if v > 40.0 {
        1.0
    } else {
        1.0 / (1.0 + f64::exp(-v))
    }
}
