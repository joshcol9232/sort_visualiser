#[macro_use]
extern crate shrinkwraprs;

mod sorting_array;
mod tools;
pub mod config;

use nannou::draw::Draw;
use nannou::prelude::*;
use nannou_audio::Buffer;
use yaml_rust::Yaml;

use crate::{
    sorting_array::{DisplayMode, SortArray, SortInstruction, audio::{Audio, Waveform}},
    config::Config,
};

use std::f32::consts::PI;
use std::f64::consts::PI as PIf64;
use std::io::{self, Read};
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};

const CONFIG_FILE_LOCATION: &str = "./config.yaml";

pub const TWO_PI: f32 = 2.0 * PI;
const SOUND_DURATION: Duration = Duration::from_millis(100);


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    arrays: Vec<SortArray>,
    current_display_mode: DisplayMode,
    window_dims: (f32, f32),
    audio_stream: nannou_audio::Stream<Audio>,
    audio_time_started: Option<Instant>,
    array_len: usize,
    config: Config,

    shift_key_down: bool,
}

impl Model {
    fn new() -> io::Result<Self> {
        let conf = &Self::load_config_file()?[0];
        let config_obj = Config::from(conf);
 
        // Sound. Sound settings cannot be reloaded without restarting the program.
        let waveform = Waveform::from_str(conf["waveform"].as_str()
            .expect("Could not parse waveform field in config as a string.")
        ).unwrap();
        let maximum_pitch = conf["maximum_pitch"].as_f64()
            .expect("Could not parse maximum_pitch field in config as a float.");
        let minimum_pitch = conf["minimum_pitch"].as_f64()
            .expect("Could not parse minimum_pitch field in config as a float.");

        // Load audio.
        let audio_host = nannou_audio::Host::new();

        let mut audio_obj = Audio::new(minimum_pitch, maximum_pitch, waveform);
        if !config_obj.sound_enabled {
            audio_obj.volume = 0.0;
        }

        let stream = audio_host
            .new_output_stream(audio_obj)
            .render(audio_render)
            .build()
            .unwrap();

        stream.pause().unwrap();

        Ok(Self {
            arrays: vec![SortArray::new(
                config_obj.array_len,
                Arc::clone(&config_obj.sleep_times),
            )],
            current_display_mode: DisplayMode::Bars,
            window_dims: (0.0, 0.0),
            audio_stream: stream,
            audio_time_started: None,
            array_len: config_obj.array_len,
            config: config_obj,
            shift_key_down: false,
        })
    }

    fn load_config_file() -> io::Result<Vec<Yaml>> {
        use yaml_rust::YamlLoader;
        use std::fs;

        let mut conf_file_string = String::new();
        fs::File::open(CONFIG_FILE_LOCATION)?
            .read_to_string(&mut conf_file_string)?;

        let confs = YamlLoader::load_from_str(&conf_file_string).unwrap();
        if confs.is_empty() { panic!("Error: Config file is empty.") }
        Ok(confs)
    }

    // Sends instruction to all arrays
    fn instruction(&mut self, instruction: SortInstruction) {
        for arr in self.arrays.iter_mut() {
            arr.instruction(instruction);
        }
    }

    fn display(&self, draw: &Draw, transform: (f32, f32)) {
        for (i, arr) in self.arrays.iter().enumerate() {
            arr.display(
                draw,
                i,
                self.arrays.len(),
                arr.len(),
                self.current_display_mode,
                self.window_dims,
                transform,
                self.config.doughnut_ratio,
            );
        }
    }

    fn set_to_single_array(&mut self) {
        self.arrays.clear();
        self.array_len = self.config.array_len;
        self.arrays.push(SortArray::new(
            self.config.array_len,
            self.config.sleep_times.clone(),
        ));
    }

    fn set_to_multi_array(&mut self, array_num: usize) {
        self.arrays.clear();
        for _ in 0..array_num {
            self.arrays.push(SortArray::new(
                self.config.multi_array_len,
                self.config.sleep_times.clone(),
            ));
        }
    }

    #[inline]
    fn reload_config(&mut self) {
        self.config = Config::from(&Self::load_config_file().unwrap()[0]);
        self.set_to_single_array();
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .event(event)
        .view(view)
        .build()
        .unwrap();

    Model::new().unwrap()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let window_rect = app.window_rect();
    model.window_dims = (window_rect.w(), window_rect.h());

    if model.audio_stream.is_playing() {
        if let Some(time_playing) = model.audio_time_started {
            if time_playing.elapsed() >= SOUND_DURATION {
                model.audio_stream.pause().unwrap();
                model.audio_time_started = None;
            }
        }
    }

    if model.arrays.len() == 1 {        // If only a single array, then play a sound.
        let mut write = model.arrays[0].data.write().unwrap();
        if write.should_play_sound {
            if let Some(index) = write.active {     // If a sound should be played, and there is a current index
                let ratio = write[index] as f64/write.max_val as f64;
                model.audio_stream.send(move |audio| {
                    audio.hz = audio.min_hz + (audio.max_hz - audio.min_hz) * ratio;    // Interpolate
                }).unwrap();

                model.audio_stream.play().unwrap();
                model.audio_time_started = Some(Instant::now());

                write.should_play_sound = false;
            }
        }
    }
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        // Keyboard events
        KeyPressed(key) => {
            match key {
                Key::LShift => model.shift_key_down = true,

                Key::S => model.instruction(SortInstruction::Shuffle(model.config.shuffle_passes)),
                Key::R => model.instruction(SortInstruction::Reset),
                Key::I => model.instruction(SortInstruction::Reverse),
                Key::L => {
                    if model.shift_key_down {
                        model.reload_config()
                    } else {
                        model.current_display_mode = DisplayMode::DisparityLine
                    }
                },

                Key::C | Key::B | Key::D | Key::O | Key::Y | Key::N => {
                    if model.arrays.len() > 1 {
                        model.set_to_single_array();
                    }

                    match key {
                        Key::C => model.current_display_mode = DisplayMode::Circle,
                        Key::O => model.current_display_mode = DisplayMode::Doughnut,
                        Key::B => model.current_display_mode = DisplayMode::Bars,
                        Key::Y => model.current_display_mode = DisplayMode::Pyramid,
                        Key::D => model.current_display_mode = DisplayMode::Dots,
                        // Key::L => model.current_display_mode = DisplayMode::Line,
                        _ => (),
                    }
                }
                Key::P => {
                    // Pixel display mode (multi-array)
                    model.array_len = model.config.multi_array_len;
                    // Make it so that each pixel is square.
                    let pixel_size = model.window_dims.0 / model.array_len as f32;
                    let array_num = (model.window_dims.1 / pixel_size).floor() as usize;

                    model.set_to_multi_array(array_num);
                    model.current_display_mode = DisplayMode::Pixels;
                }
                Key::Q => model.instruction(SortInstruction::Stop),

                Key::Key1 => model.instruction(SortInstruction::BubbleSort),
                Key::Key2 => model.instruction(SortInstruction::CocktailShakerSort),
                Key::Key3 => model.instruction(SortInstruction::InsertionSort),
                Key::Key4 => model.instruction(SortInstruction::SelectionSort),
                Key::Key5 => model.instruction(SortInstruction::ShellSort),
                Key::Key6 => model.instruction(SortInstruction::QuickSort(model.config.quicksort_partition_type)),
                Key::Key7 => model.instruction(SortInstruction::MergeSort(model.config.merge_sort_type)),
                Key::Key8 => model.instruction(SortInstruction::RadixSort(model.config.radix_base)),
                _ => (),
            }
        }
        KeyReleased(key) => {
            match key {
                Key::LShift => model.shift_key_down = false,
                _ => (),
            }
        }

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
    let transformation = (-model.window_dims.0 / 2.0, -model.window_dims.1 / 2.0); // Axis starts bottom left corner

    let draw = app.draw();
    draw.background().color(BLACK);

    model.display(&draw, transformation);

    draw.to_frame(app, &frame).unwrap();
}

pub fn audio_render(audio: &mut Audio, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;

    for frame in buffer.frames_mut() {
        let sin_amp = (2.0 * PIf64 * audio.phase).sin() as f32;

        let waveform_amp = match audio.waveform {
            Waveform::Sine => sin_amp,
            Waveform::Haversine => sin_amp.max(0.0),    // > 0.0
            Waveform::Square => (((sin_amp + 1.0)/2.0).round() - 0.5) * 2.0,
            Waveform::Triangle => sin_amp.round(),
        };

        audio.phase += audio.hz / sample_rate;
        audio.phase %= sample_rate;
        for channel in frame {
            *channel = waveform_amp * audio.volume;
        }
    }
}