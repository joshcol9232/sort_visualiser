#[macro_use] extern crate shrinkwraprs;

mod tools;
mod sorting_array;

use nannou::prelude::*;
use nannou::draw::Draw;

use crate::sorting_array::{SortArray, SortInstruction, QuickSortType, DisplayMode};

use std::f32::consts::PI;

pub const TWO_PI: f32 = 2.0 * PI;
pub const DEFAULT_DATA_LEN: usize = 500;


fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

#[derive(Default)]
struct Model {
    arrays: Vec<SortArray>,
    current_display_mode: DisplayMode,
    window_dims: (f32, f32),
    array_len: usize,
}

impl Model {
    // Sends instruction to all arrays
    fn instruction(&mut self, instruction: SortInstruction) {
        for arr in self.arrays.iter_mut() {
            arr.instruction(instruction);
        }
    }

    fn display(&self, draw: &Draw, transform: (f32, f32)) {
        for (i, arr) in self.arrays.iter().enumerate() {
            arr.display(draw, i, self.arrays.len(), arr.len(), self.current_display_mode, self.window_dims, transform);
        }
    }

    fn set_to_single_array(&mut self) {
        self.arrays.clear();
        self.arrays.push(SortArray::new(self.array_len));
    }

    fn set_to_multi_array(&mut self, len: usize) {
        self.arrays.clear();
        for _ in 0..len {
            self.arrays.push(SortArray::new(self.array_len));
        }
    }
}


fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let model = Model {
        arrays: vec![SortArray::new(DEFAULT_DATA_LEN)],
        current_display_mode: DisplayMode::Circle,
        window_dims: (0.0, 0.0),
        array_len: DEFAULT_DATA_LEN
    };

    model
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window_rect = app.window_rect();
    model.window_dims = (window_rect.w(), window_rect.h());
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        // Keyboard events
        KeyPressed(key) => {
            match key {
                Key::S => model.instruction(SortInstruction::Shuffle(3)),
                Key::R => model.instruction(SortInstruction::Reset),
                Key::I => model.instruction(SortInstruction::Reverse),

                Key::C | Key::B | Key::D => {
                    if model.arrays.len() > 1 {
                        model.set_to_single_array();
                    }
                    
                    match key {
                        Key::C => model.current_display_mode = DisplayMode::Circle,
                        Key::B => model.current_display_mode = DisplayMode::Bars,
                        Key::D => model.current_display_mode = DisplayMode::Dots,
                        _ => ()
                    }
                },
                Key::P => {     // Pixel display mode (multi-array)
                    if model.arrays.len() == 1 {
                        model.set_to_single_array();
                    }

                    // Make it so that each pixel is square.
                    let pixel_height = model.window_dims.1/model.array_len as f32;
                    let array_num = (model.window_dims.0/pixel_height).ceil() as usize;

                    model.set_to_multi_array(array_num);
                    model.current_display_mode = DisplayMode::Pixels;
                }

                Key::Key1 => model.instruction(SortInstruction::BubbleSort),
                Key::Key2 => model.instruction(SortInstruction::InsertionSort),
                Key::Key3 => model.instruction(
                    SortInstruction::QuickSort(QuickSortType::LomutoPartitioning)
                ),
                // Key::Key4 => model.instruction(
                //     SortInstruction::QuickSort(QuickSortType::Overwriting)
                // ),
                _ => ()
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

fn view(app: &App, model: &Model, frame: &Frame) {
    let transformation = (-model.window_dims.0/2.0, -model.window_dims.1/2.0);      // Axis starts bottom left corner

    let draw = app.draw();
    draw.background().color(BLACK);

    model.display(&draw, transformation);

    draw.to_frame(app, &frame).unwrap();
}