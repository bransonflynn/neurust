#[allow(dead_code)]
pub struct Neuron {
    weights: (i32, i32),
    bias: i32,
}
#[allow(dead_code)]
#[allow(unused_variables)]
impl Neuron {
    pub fn feed_forward(&self, inputs: (i32, i32)) {
        //
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x))
}

#[allow(unused_variables)]
pub fn exp(input: &mut (i32, i32)) {
    //utils::math::sigmoid(0.6)
}
