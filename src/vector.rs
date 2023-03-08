
// ===== Imports =====
use std::ops::Add;
// ===================

pub struct Vector {
  data: Box<[f32]>,
}

impl From<Vec<f32>> for Vector {
  fn from(value: Vec<f32>) -> Self {
    Self {
      data: value.into_boxed_slice(),
    }
  }
}

impl Vector {
  pub fn zeroes(len: usize) -> Self {
    Self {
      data: vec![0_f32; len].into_boxed_slice(),
    }
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn dot(&self, rhs: Vector) -> f32 {
    let mut res = 0_f32;
    if self.data.len() != rhs.data.len() {
      panic!("Cannot find Dot Product for vectors of unequal size.");
    }

    for i in 0..self.data.len() {
      res += self.data[i]*rhs.data[i];
    }

    res
  }
}

impl Add<Vector> for Vector {
  type Output = Vector;

  fn add(self, rhs: Vector) -> Self::Output {
    let mut data = self.data;
    for (i, item) in rhs.data.iter().enumerate() {
      if i >= data.len() { break; }
      data[i] += item;
    }

    Self { data }
  }
}