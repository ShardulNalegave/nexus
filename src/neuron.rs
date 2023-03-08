
// ===== Imports =====
use crate::vector::Vector;
// ===================

pub struct Neuron {
  input_size: usize,
  weights: Vector,
  bias: f32,
}

impl Neuron {
  pub fn new(input_size: usize) -> Self {
    Self {
      input_size,
      bias: 1.0,
      weights: Vector::from(vec![0.5; input_size]),
    }
  }

  pub fn process(&self, input: &Vector) -> f32 {
    assert_eq!(input.len(), self.input_size);
    self.weights.dot(input) + self.bias
  }

  pub fn set_bias(&mut self, bias: f32) {
    self.bias = bias
  }

  pub fn set_weights(&mut self, weights: Vector) {
    self.weights = weights;
  }
}