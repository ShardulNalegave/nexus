use nalgebra::{DMatrix};
use nexus::prelude::*;

fn main() {
  let layer = Layer::new(1, 3);
  let input = DMatrix::from_vec(1, 3, vec![0.5, 0.2, 0.85]);
  let res = layer.predict_batch(input.clone());

  println!("Weights: \n{:?}", layer.weights());
  println!("Biases: \n{:?}", layer.biases());
  println!("Input: \n{:?}", input);
  println!("Result: \n{:?}", res);
}