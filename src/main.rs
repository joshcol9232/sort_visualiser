extern crate nannou;

mod sorting_array;

use nannou::prelude::*;

use crate::sorting_array::{SortArray, SortInstruction};

const DATA_LEN: usize = 200;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    arr: SortArray,
}

impl Model {

}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let model = Model {
        arr: SortArray::new(DATA_LEN),
    };

    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        // Keyboard events
        KeyPressed(key) => {
            if key == Key::S {
                model.arr.edit(SortInstruction::Shuffle(3));
            } else if key == Key::Key1 {
                model.arr.edit(SortInstruction::BubbleSort);
            }
        }
        KeyReleased(_key) => {}

        // Mouse events
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseWheel(_amount, _phase) => {}
        MouseEntered => {}
        MouseExited => {}

        // Touch events
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}

        // Window events
        Moved(_pos) => {}
        Resized(_size) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let window_rect = app.window_rect();
    let window_dims = (window_rect.w(), window_rect.h());

    let transformation = (-window_dims.0/2.0, -window_dims.1/2.0);      // Axis starts bottom left corner

    let draw = app.draw();
    draw.background().color(BLACK);

    model.arr.display(&draw, window_dims, transformation);

    draw.to_frame(app, &frame).unwrap();

    frame
}