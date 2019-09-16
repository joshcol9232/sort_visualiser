use std::str::FromStr;
use std::io;

#[derive(Default, Debug)]
pub struct Audio {
    pub phase: f64,
    pub hz: f64,
    pub min_hz: f64,
    pub max_hz: f64,
    pub volume: f32,
    pub waveform: Waveform
}

impl Audio {
    pub fn new(min_hz: f64, max_hz: f64, waveform: Waveform) -> Self {
        Self {
            min_hz,
            max_hz,
            volume: 0.25,
            waveform,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub enum Waveform {
    Sine,
    Haversine,
    Square,
    Triangle,
}

impl Default for Waveform {
    fn default() -> Waveform {
        Waveform::Haversine
    }
}

impl FromStr for Waveform {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        match s.to_lowercase().as_str() {
            "sin" => Ok(Waveform::Sine),
            "hsin" => Ok(Waveform::Haversine),
            "square" => Ok(Waveform::Square),
            "triangle" => Ok(Waveform::Triangle),
            x => Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid waveform format in config file: {}. Options are: sin, hsin, square, triangle", x)
                )
            ),
        }
    }
}