
// ===== Imports =====
use crate::net::NeuralNet;
use nannou::prelude::*;
// ===================

pub struct InteractiveMode {
  net: NeuralNet,
  origin: Point2,
}

impl InteractiveMode {
  pub fn with(net: NeuralNet, app: &App) -> Self {
    app.main_window().set_maximized(true);
    app.main_window().set_resizable(false);
    app.main_window().set_title("Nexus - Interactive Mode");

    let origin = Point2::new(0.0, 0.0);
    Self { net, origin }
  }

  pub fn update(&mut self, _app: &App, _update: Update) {
    //
  }

  pub fn event(&mut self, _app: &App, event: Event) {
    match event {
      Event::WindowEvent { simple, .. } => {
        if simple.is_none() { return; }
        match simple.unwrap() {
          KeyPressed(key) => match key {
            Key::Up => self.origin.y += 5.0,
            Key::Down => self.origin.y -= 5.0,
            Key::Left => self.origin.x -= 5.0,
            Key::Right => self.origin.x += 5.0,
            _ => {},
          },
          _ => {},
        }
      },
      Event::DeviceEvent(_, _) => {}
      Event::Update(_) => {}
      Event::Suspended => {}
      Event::Resumed => {}
    }
  }

  pub fn draw(&self, app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.1, 0.1, 0.1);

    let draw_origin = self.origin + Point2::new(app.window_rect().left() + 100.0, 0.0);
    let mut current_layer_positions: Vec<Point2> = vec![];

    let mut pos = draw_origin;
    pos.y -= (100.0*(self.net.input_size as f32 - 1.0))/2.0;
    for _ in 0..self.net.input_size {
      draw.ellipse()
        .xy(pos)
        .w_h(50.0, 50.0)
        .rgb(0.5, 0.5, 0.5);

      current_layer_positions.push(pos);

      pos.y += 100.0;
    }
    let mut prev_layer_positions = current_layer_positions;
    current_layer_positions = vec![];

    let mut pos_layer = draw_origin + Point2::new(150.0, 0.0);
    for layer in &self.net.layers {
      let vertical_shift = (100.0*(layer.num_neurons as f32 - 1.0))/2.0;
      pos_layer.y -= vertical_shift;
      for i in 0..layer.num_neurons {
        let mut pos = pos_layer;
        pos.y += (i as f32)*100.0;
        current_layer_positions.push(pos);

        draw.ellipse()
          .xy(pos)
          .w_h(50.0, 50.0)
          .rgb(0.5, 0.5, 0.8);

        for pt in &prev_layer_positions {
          draw.line()
            .start(*pt)
            .end(pos)
            .weight(2.0)
            .rgb(0.0, 0.0, 0.0);
        }
      }

      pos_layer.y += vertical_shift;
      pos_layer.x += 150.0;
      prev_layer_positions = current_layer_positions;
      current_layer_positions = vec![];
    }

    draw.ellipse()
      .xy(draw_origin)
      .w_h(10.0, 10.0)
      .rgb(1.0, 1.0, 1.0);

    draw.to_frame(app, &frame).unwrap();
  }
}