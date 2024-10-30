#[allow(unused_imports)]
use numpy::dot_bound;

#[allow(dead_code)]
pub struct Neuron {
    weights: Vec<i32>,
    bias: i32,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Neuron {
    pub fn feed_forward(&self, inputs: Vec<i32>) {
        //let total = dot_bound(self.weights, input);
    }
    /*
    def feedforward(self, inputs):
        # Weight inputs, add bias, then use the activation function
        total = np.dot(self.weights, inputs) + self.bias
        return sigmoid(total)
    */
}

pub fn make_neuron(weights: Vec<i32>, bias: i32) -> Neuron {
    let result: Neuron = Neuron { weights, bias };
    result
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[allow(dead_code)]
pub struct NeuralNetwork {
    id: u64,
    neurons: Vec<Neuron>,
}
impl NeuralNetwork {}

