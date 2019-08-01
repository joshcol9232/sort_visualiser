use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

use nannou::{
    color::{Hsv, RgbHue},
    draw::Draw,
    geom::point::Point2,
};

const SWAP_SLEEP: Duration = Duration::from_millis(5);

pub struct SortArray {
    pub data: Arc<RwLock<Vec<usize>>>,
    pub max_val: usize,
    active: isize,  // current active index
    sort_thread: Option<thread::JoinHandle<()>>,
}

impl SortArray {
    pub fn new(num_of_lines: usize) -> SortArray {
        SortArray {
            data: Arc::new(RwLock::new((0..num_of_lines).collect())),
            max_val: num_of_lines,
            active: -1,
            sort_thread: None,
        }
    }

    pub fn edit(&mut self, instruction: &str) {
        match instruction {
            "shuffle" => {
                let data_arc_cln = Arc::clone(&self.data);
                self.sort_thread = Some(thread::spawn(move || {
                    Self::shuffle(data_arc_cln, 3);
                }));
            },
            _ => ()
        }
    }

    pub fn display(&self, draw: &Draw, window_dims: (f32, f32), transform: (f32, f32)) {
        let data_read = self.data.read().unwrap();

        let scale = (window_dims.0/data_read.len() as f32, window_dims.1/self.max_val as f32);

        for (i, d) in data_read.iter().enumerate() {
            let x = (i as f32 * scale.0) + scale.0/2.0;

            let rgbhue = if i as isize == self.active {
                RgbHue::from(0.0)
            } else {
                RgbHue::from((*d as f32/self.max_val as f32) * 360.0)
            };

            let col = Hsv::new(rgbhue, 1.0, 1.0);

            draw.line()
                .x_y(transform.0, transform.1)
                .start(Point2::new(x, 0.0))
                .end(Point2::new(x, (*d as f32 + 1.0) * scale.1))
                .thickness(scale.0)
                .color(col);
        }
    }

    pub fn shuffle(data: Arc<RwLock<Vec<usize>>>, passes: u16) {
        let len = data.read().unwrap().len();

        for _ in 0..passes {
            for i in 0..len {
                let mut data_write = data.write().unwrap();
                Self::swap(&mut data_write, i, nannou::rand::random_range(0usize, len));
                thread::sleep(SWAP_SLEEP);
            }
        }
    }

    fn swap(data: &mut Vec<usize>, i: usize, j: usize) {
        let temp = data[i];
        data[i] = data[j];
        data[j] = temp;
    }
}