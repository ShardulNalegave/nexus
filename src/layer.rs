
// ===== Imports =====
use nalgebra::{DMatrix, DVector, Dyn, MatrixSum, OMatrix, U1};
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

  pub fn weights(&self) -> OMatrix<f32, Dyn, Dyn> {
    self.weights.clone()
  }

  pub fn biases(&self) -> OMatrix<f32, Dyn, U1> {
    self.biases.clone()
  }

  pub fn predict_batch(&self, input: OMatrix<f32, Dyn, Dyn>) -> MatrixSum<f32, Dyn, Dyn, Dyn, U1> {
    (input * &self.weights.transpose()) + &self.biases
  }
}