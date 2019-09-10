use nannou_audio as audio;
use audio::Buffer;
use std::f64::consts::PI;

#[derive(Default)]
pub struct Audio {
    pub phase: f64,
    pub hz: f64,
    pub min_hz: f64,
    pub max_hz: f64,
    pub volume: f32,
}

impl Audio {
    pub fn new(min_hz: f64, max_hz: f64) -> Self {
        Self {
            min_hz,
            max_hz,
            volume: 1.0,
            ..Default::default()
        }
    }
}