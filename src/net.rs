
// ===== Imports =====
use crate::layer::Layer;
use crate::yaml::NeuralNetworkDescriptor;
// ===================

pub struct NeuralNet {
  pub input_size: usize,
  pub output_size: usize,
  pub(crate) layers: Vec<Layer>,
}

impl NeuralNet {
  pub fn new(layers: Vec<Layer>) -> Self {
    let input_size = layers.first().unwrap().input_size;
    let output_size = layers.last().unwrap().output_size;
    Self { input_size, output_size, layers }
  }

  pub fn from_yaml(yaml: &str) -> Self {
    let descriptor: NeuralNetworkDescriptor = serde_yaml::from_str(yaml)
      .expect("Could not process YAML Descriptor file.");

    let mut layers = vec![];
    let mut layer_input_size = descriptor.input_size;

    for layer_desc in &descriptor.layers {
      let layer = Layer::new(layer_desc.neurons, layer_input_size);
      layer_input_size = layer_desc.neurons;
      layers.push(layer);
    }

    let input_size = descriptor.input_size;
    let output_size = descriptor.layers.last()
      .unwrap().neurons;
    Self { input_size, output_size, layers }
  }
}