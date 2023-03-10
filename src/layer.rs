
// ===== Imports =====
use nalgebra::{DMatrix, DVector, Dyn, OMatrix, U1};
// ===================

pub struct Layer {
  weights: OMatrix<f32, Dyn, Dyn>,
  biases: OMatrix<f32, Dyn, U1>,
}

impl Layer {
  pub fn new(num_neurons: usize, num_weights: usize) -> Self {
    let weights = DMatrix::from_fn(num_neurons, num_weights, |_, _| rand::random());
    let biases = DVector::identity(num_neurons);

    Self { weights, biases }
  }
}