
// ===== Imports =====
use serde::{Deserialize, Serialize};
// ===================

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct NeuralNetworkDescriptor {
  pub(crate) input_size: usize,
  pub(crate) layers: Vec<LayerDescriptor>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub(crate) struct LayerDescriptor {
  pub(crate) neurons: usize,
}