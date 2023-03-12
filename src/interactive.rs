
// ===== Imports =====
use crate::net::NeuralNet;
use nannou::prelude::*;
// ===================

pub struct InteractiveMode {
  net: NeuralNet,
}

impl InteractiveMode {
  pub fn with(net: NeuralNet, app: &App) -> Self {
    app.main_window().set_maximized(true);
    app.main_window().set_resizable(false);
    Self { net }
  }

  pub fn event(&mut self, _app: &App, _event: Event) {
    //
  }

  pub fn draw(&self, app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.25, 0.25, 0.25);
    draw.to_frame(app, &frame).unwrap();
  }
}