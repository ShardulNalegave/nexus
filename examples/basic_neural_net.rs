
use nexus::prelude::*;
use nannou::prelude::*;

fn main() {
  nannou::app(model)
    .event(event)
    .simple_window(view)
    .run();
}

fn model(app: &App) -> InteractiveMode {
  let net = NeuralNet::from_yaml(include_str!("basic_neural_net.yaml"));
  InteractiveMode::with(net, app)
}

fn event(app: &App, model: &mut InteractiveMode, event: Event) {
  model.event(app, event);
}

fn view(app: &App, model: &InteractiveMode, frame: Frame) {
  model.draw(app, frame);
}