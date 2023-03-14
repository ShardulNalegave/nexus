
use nexus::prelude::*;
use nannou::prelude::*;

fn main() {
  nannou::app(model)
    .event(event)
    .update(update)
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

fn update(app: &App, model: &mut InteractiveMode, update: Update) {
  model.update(app, update);
}

fn view(app: &App, model: &InteractiveMode, frame: Frame) {
  model.draw(app, frame);
}