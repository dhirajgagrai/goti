mod colors;
mod goti;
mod model;

use nannou::prelude::*;

use goti::Boundary;
use model::Model;

const TITLE: &str = "goti";

fn main() {
    nannou::app(model)
        .simple_window(view)
        .update(update)
        .event(event)
        .loop_mode(LoopMode::loop_once())
        .run();
}

trait Render {
    fn display(&self, draw: &Draw);
    fn update(&mut self, boundary: Boundary);
}

fn model(_app: &App) -> Model {
    Model::default()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.update(app.window_rect().into());
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Render the background and goti
    let draw = app.draw();
    model.display(&draw);
    draw.to_frame(app, &frame).unwrap();

    // Set title for the window
    app.main_window().set_title(TITLE);
}

fn event(app: &App, model: &mut Model, e: Event) {
    match e {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(MouseMoved(pointer)) => {
                if !model.running {
                    model.pointer_angle = pointer.angle().to_degrees();
                }
            }
            Some(MousePressed(MouseButton::Left)) => {
                if !model.running {
                    app.set_loop_mode(LoopMode::refresh_sync());
                    model.goti.launch_angle = Some(model.pointer_angle);
                    model.running = !model.running;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
