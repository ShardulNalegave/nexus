
// ===== Imports =====
use crate::neuron::Neuron;
use crate::vector::Vector;
// ===================

pub struct Layer {
  input_size: usize,
  neurons: Vec<Neuron>,
}

impl Layer {
  pub fn new(input_size: usize, num_neurons: usize) -> Self {
    let mut neurons = vec![];
    for _ in 0..num_neurons {
      neurons.push(Neuron::new(input_size));
    }
    Self { input_size, neurons }
  }

  pub fn process(&mut self, input: &Vector) -> Vector {
    assert_eq!(input.len(), self.input_size);
    let mut data = vec![];
    for neuron in self.neurons.iter_mut() {
      let res = neuron.process(input);
      data.push(res);
    }

    Vector::from(data)
  }
}