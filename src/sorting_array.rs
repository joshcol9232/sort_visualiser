use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

use nannou::{
    color::{Hsv, RgbHue},
    draw::Draw,
    geom::point::Point2,
};

use crate::{tools, DATA_LEN, TWO_PI};

const SWAP_SLEEP: Duration = Duration::from_millis(1);
const BUBBLE_SLEEP: Duration = Duration::from_secs(40);    // For 1 element/len squared


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

    pub fn edit(&mut self, instruction: SortInstruction) {
        let data_arc_cln = Arc::clone(&self.data);
        match instruction {
            SortInstruction::Shuffle(rounds) => {
                self.sort_thread = Some(thread::spawn(move || {
                    Self::shuffle(data_arc_cln, rounds);
                }));
            },
            SortInstruction::BubbleSort => {
                self.sort_thread = Some(thread::spawn(move || {
                    Self::bubble_sort(data_arc_cln);
                }));
            }
            SortInstruction::QuickSort => {
                let len = self.data.read().unwrap().len();
                self.sort_thread = Some(thread::spawn(move || {
                    Self::quick_sort(data_arc_cln, 0, len);
                }));
            }
        }
    }

    #[inline]
    fn get_color(&self, i: usize, d: &usize, max_col_val: f32, offset: f32) -> Hsv {
        let rgbhue = if i as isize == self.active {
            RgbHue::from(0.0)
        } else {
            RgbHue::from((*d as f32/self.max_val as f32) * max_col_val + offset)
        };

        Hsv::new(rgbhue, 1.0, 1.0)
    }

    #[inline]
    pub fn display(&self, draw: &Draw, mode: DisplayMode, window_dims: (f32, f32), transform: (f32, f32)) {
        let data_read = self.data.read().unwrap();

        match mode {
            DisplayMode::Bars => {
                let scale = (window_dims.0/data_read.len() as f32, window_dims.1/self.max_val as f32);

                for (i, d) in data_read.iter().enumerate() {
                    let x = (i as f32 * scale.0) + scale.0/2.0;

                    let col = self.get_color(i, d, 120.0, 0.0);

                    draw.line()
                        .x_y(transform.0, transform.1)
                        .start(Point2::new(x, 0.0))
                        .end(Point2::new(x, (*d as f32 + 1.0) * scale.1))
                        .thickness(scale.0)
                        .color(col);
                }
            },
            DisplayMode::Circle => {
                let radius = if window_dims.0 > window_dims.1 { window_dims.1 } else { window_dims.0 } / 2.0;
                println!("Radius: {}", radius);

                let angle_interval = TWO_PI/DATA_LEN as f32;
                let mut angle = 0.0;

                for (i, d) in data_read.iter().enumerate() {
                    let connecting_angle = angle + angle_interval;

                    let col = self.get_color(i, d, 360.0, 0.0);

                    draw.tri()
                        .points(
                            [0.0, 0.0],
                            tools::get_point_on_radius(radius, angle),
                            tools::get_point_on_radius(radius, connecting_angle)
                        )
                        .color(col);

                    angle = connecting_angle;
                }
            }
        }
        
    }

    fn shuffle(data: Arc<RwLock<Vec<usize>>>, passes: u16) {
        let len = data.read().unwrap().len();

        for _ in 0..passes {
            for i in 0..len {
                {
                    let mut data_write = data.write().unwrap();
                    Self::swap(&mut data_write, i, nannou::rand::random_range(0usize, len));
                }
                thread::sleep(SWAP_SLEEP);
            }
        }
    }

    fn swap(data: &mut Vec<usize>, i: usize, j: usize) {
        let temp = data[i];
        data[i] = data[j];
        data[j] = temp;
    }


    fn bubble_sort(data_arc: Arc<RwLock<Vec<usize>>>) {
        let mut sorted = false;
        let len = data_arc.read().unwrap().len();

        while !sorted {
            sorted = true;

            for i in 0..len-1 {
                let (d1, d2) = {
                    let read = data_arc.read().unwrap();
                    (read[i], read[i+1])
                };
                if d1 > d2 {
                    {
                        let mut data_write = data_arc.write().unwrap();
                        Self::swap(&mut data_write, i, i+1);
                    }
                    sorted = false;
                    thread::sleep(BUBBLE_SLEEP/len.pow(2) as u32);
                }
            }
        }
    }

    // Uses indicies of array rather than making new ones
    fn quick_sort(data_arc: Arc<RwLock<Vec<usize>>>, low: usize, high: usize) {
        assert!(high > low);    // High should always be > low.
        println!("{} {}", high, low);

        let len = high - low;
        let pivot_index = high-1;
        let pivot = data_arc.read().unwrap()[pivot_index];

        for i in low..high {
            let item = data_arc.read().unwrap()[i];   // Read from array

        }
    }
}

pub enum SortInstruction {
    Shuffle(u16),
    BubbleSort,
    QuickSort,
}

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Bars,
    Circle,
}