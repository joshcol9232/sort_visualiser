use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use nannou::{
    draw::Draw,
    geom::point::Point2,
};

use super::{commands::*, sorts};
use crate::{tools, TWO_PI, config::SleepTimes};

macro_rules! start_sort_thread {
    // Starts a sorting thread (common pattern)
    ($self:expr, $data_arc:expr, $operation:block) => {
        $data_arc.write().unwrap().sorted = false;
        $self.sort_thread = Some(thread::spawn(move || {
            $operation;
            SortArray::reset_arr_info($data_arc);
        }));
    };
}

// Colour the element when using red -> green colours (uses purple and blues to display pivot etc).
// Used in both dots and bars vis.
macro_rules! colour_element_red_grn_clrs {
    ($data_read:expr, $i:expr, $drawing:expr, $max_val:expr, $d:expr) => {
        if Some($i) == $data_read.active || Some($i) == $data_read.active_2 {
            $drawing.rgb(0.0, 0.2, 1.0);
        } else if Some($i) == $data_read.pivot {
            $drawing.rgb(0.8516, 0.4023, 0.8945); // Purple colour
        } else {
            $drawing.hsv((*$d as f32 / $max_val as f32) / 3.0, 1.0, 1.0);
        }
    };
}

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct DataArrWrapper {
    // Wrapper arround array that is passed to sorting thread, containing info about current sort.
    #[shrinkwrap(main_field)]
    pub arr: Vec<usize>,
    pub active: Option<usize>,
    pub active_2: Option<usize>,
    pub pivot: Option<usize>,
    pub should_play_sound: bool,
    pub sorted: bool,
    pub max_val: usize,
}

impl DataArrWrapper {
    pub fn new(arr: Vec<usize>, max_val: usize) -> Self {
        Self {
            arr,
            active: None,
            active_2: None,
            pivot: None,
            should_play_sound: false,
            sorted: true,
            max_val,
        }
    }

    #[inline]
    pub fn set_active(&mut self, index: usize) {
        self.active = Some(index);
        self.should_play_sound = true;
    }

    #[inline]
    pub fn set_active_2(&mut self, index: usize) {
        self.active_2 = Some(index);
    }

    #[inline]
    pub fn set_pivot(&mut self, index: usize) {
        self.pivot = Some(index);
    }
}

pub struct SortArray {
    pub data: Arc<RwLock<DataArrWrapper>>,
    sleep_times: Arc<SleepTimes>,
    sort_thread: Option<thread::JoinHandle<()>>,
}

impl SortArray {
    pub fn new(num_of_lines: usize, sleep_times: Arc<SleepTimes>) -> SortArray {
        SortArray {
            data: Arc::new(RwLock::new(
                DataArrWrapper::new(
                    (0..num_of_lines).collect(), // Make an array of incrementing numbers up to the length of the array.
                    num_of_lines,
                ),
            )), // Then when drawing you can scale it however you want.
            sleep_times,
            sort_thread: None,
        }
    }

    // Easier to handle in here rather than in main
    pub fn instruction(&mut self, instruction: SortInstruction) {
        let data_arc_cln = Arc::clone(&self.data);
        let sleep_times_cln = Arc::clone(&self.sleep_times);
        let data_len = data_arc_cln.read().unwrap().len();

        match instruction {
            SortInstruction::Shuffle(rounds) => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.shuffle/data_len.pow(2) as u32;
                    Self::shuffle(data_arc_cln.clone(), &sleep_time, rounds);
                });
            }
            SortInstruction::BubbleSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.bubble/data_len.pow(2) as u32;
                    sorts::bubble_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::QuickSort(partition_type) => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = Arc::new(sleep_times_cln.quick/data_len as u32); //sleep_times_cln.quick/((data_len as f32).log10().floor() as u32 * data_len as u32);
                    match partition_type {
                        QuickSortType::Lomuto {
                            multithreaded,
                        } => {
                            if multithreaded {
                                sorts::quick_sorting::quick_sort_lomuto_multithreaded(data_arc_cln.clone(), sleep_time, 0, data_len - 1)
                            } else {
                                sorts::quick_sorting::quick_sort_lomuto(data_arc_cln.clone(), sleep_time, 0, data_len - 1)
                            }
                        }
                    }
                });
            }
            SortInstruction::InsertionSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.insertion/(data_len).pow(2) as u32;
                    sorts::insertion_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::SelectionSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.selection/(data_len).pow(2) as u32;
                    sorts::selection_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::CocktailShakerSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.cocktail/(data_len).pow(2) as u32;
                    sorts::cocktail_shaker_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::ShellSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.shell/((data_len as f32).powf(3.0/2.0).floor() as u32);
                    sorts::shell_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::CombSort => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.comb/((data_len as f32).powf(3.0/2.0).floor() as u32);
                    sorts::comb_sort(data_arc_cln.clone(), &sleep_time);
                });
            }
            SortInstruction::RadixSort(base) => {
                start_sort_thread!(self, data_arc_cln, {
                    let sleep_time = sleep_times_cln.radix/data_len as u32;
                    sorts::radix_lsd(data_arc_cln.clone(), &sleep_time, base);
                });
            }
            SortInstruction::MergeSort(merge_type) => {
                match merge_type {
                    MergeSortType::InPlace {
                        multithreaded,
                    } => {
                        if multithreaded {
                            start_sort_thread!(self, data_arc_cln, {
                                let sleep_time = Arc::new(sleep_times_cln.merge/data_len as u32); //sleep_times_cln.merge/((data_len as f32).log10().floor() as u32 * data_len as u32);
                                sorts::merge_sorting::merge_sort_in_place_multithreaded(data_arc_cln.clone(), sleep_time, 0, data_len - 1);
                            });
                        } else {
                            start_sort_thread!(self, data_arc_cln, {
                                let sleep_time = Arc::new(sleep_times_cln.merge/data_len as u32); //sleep_times_cln.merge/((data_len as f32).log10().floor() as u32 * data_len as u32);
                                sorts::merge_sorting::merge_sort_in_place(data_arc_cln.clone(), sleep_time, 0, data_len - 1);
                            });
                        }
                    }
                }
            }

            SortInstruction::Reset => {
                self.reset();
            }
            SortInstruction::Reverse => {
                self.data.write().unwrap().sorted = false;
                self.data.write().unwrap().reverse();
            }
            SortInstruction::Stop => {
                let mut write = self.data.write().unwrap();
                write.sorted = true;
            }
        }
    }

    #[inline]
    pub fn display(
        &self,
        draw: &Draw,
        index: usize,
        max_index: usize,
        array_len: usize,
        mode: DisplayMode,
        window_dims: (f32, f32),
        transform: (f32, f32),
    ) {
        let data_read = self.data.read().unwrap();

        match mode {
            DisplayMode::Bars => {
                let scale = (
                    window_dims.0 / array_len as f32,
                    window_dims.1 / data_read.max_val as f32,
                );

                for (i, d) in data_read.iter().enumerate() {
                    let x = (i as f32 * scale.0) + scale.0 / 2.0;
                    let drawing = draw
                        .line()
                        .x_y(transform.0, transform.1)
                        .start(Point2::new(x, 0.0))
                        .end(Point2::new(x, (*d as f32 + 1.0) * scale.1))
                        .thickness(scale.0);

                    colour_element_red_grn_clrs!(data_read, i, drawing, data_read.max_val, d);
                }
            }
            DisplayMode::Circle => {
                let radius = if window_dims.0 > window_dims.1 {
                    window_dims.1
                } else {
                    window_dims.0
                } / 2.0;

                let angle_interval = TWO_PI / array_len as f32;
                let mut angle = 0.0;

                for d in data_read.iter() {
                    let connecting_angle = angle + angle_interval;

                    draw.tri()
                        .points(
                            [0.0, 0.0],
                            tools::get_point_on_radius(radius, angle),
                            tools::get_point_on_radius(radius, connecting_angle),
                        )
                        .hsv(*d as f32 / data_read.max_val as f32, 1.0, 1.0);

                    angle = connecting_angle;
                }
            }
            // DisplayMode::Line => {
            // use nannou::geom::vertex;
            // use nannou::color::{Rgba, Hsv};
            // // Polyline is broken in nannou atm, so waiting for change to lyon which they are implementing.
            //  //   see: https://github.com/nannou-org/nannou/issues/185

            // let mut points: Vec<vertex::Srgba> = Vec::with_capacity(self.max_val);
            // let scale = (window_dims.0/data_read.len() as f32, window_dims.1/self.max_val as f32);

            // for (i, d) in data_read.iter().enumerate() {
            //     let col = Rgba::from(Hsv::new(*d as f32/self.max_val as f32, 1.0, 1.0));
            //     points.push(
            //         vertex::Srgba(
            //             [(i as f32 * scale.0) + scale.0/2.0 + transform.0, (*d as f32 + 1.0) * scale.1 + transform.1].into(),
            //             col.into()
            //         )
            //     );
            // }

            // draw.polyline()
            //     .vertices(1.0, points);
            // }
            DisplayMode::Dots => {
                let scale = (
                    window_dims.0 / array_len as f32,
                    window_dims.1 / data_read.max_val as f32,
                );

                for (i, d) in data_read.iter().enumerate() {
                    let drawing = draw
                        .ellipse()
                        .x_y(
                            transform.0 + ((i as f32 * scale.0) + scale.0 / 2.0),
                            transform.1 + ((*d as f32 + 0.5) * scale.1),
                        )
                        .radius(scale.0 / 2.0);

                    colour_element_red_grn_clrs!(data_read, i, drawing, data_read.max_val, d);
                }
            }
            DisplayMode::Pixels => {
                let scale = (
                    window_dims.0 / data_read.max_val as f32,
                    window_dims.1 / max_index as f32,
                );
                let y = (index as f32 + 0.5) * scale.1;

                for (i, d) in data_read.iter().enumerate() {
                    draw.rect()
                        .x_y(transform.0 + (i as f32 + 0.5) * scale.0, transform.1 + y)
                        .w_h(scale.0, scale.1)
                        .hsv((1.0 - (*d as f32 / data_read.max_val as f32)) / 3.0, 1.0, 1.0);
                }
            }
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.read().unwrap().len()
    }

    #[inline]
    fn reset_arr_info(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let mut write = data_arc.write().unwrap();

        write.active = None;
        write.active_2 = None;
        write.pivot = None;
        write.sorted = true;
    }

    pub fn reset(&mut self) {
        Self::reset_arr_info(self.data.clone());
        let mut write = self.data.write().unwrap();
        write.arr = (0..write.len()).collect();
    }

    fn shuffle(data: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration, passes: u16) {
        let len = data.read().unwrap().len();

        for _ in 0..passes {
            for i in 0..len {
                {
                    let mut data_write = data.write().unwrap();
                    data_write.swap(i, nannou::rand::random_range(0usize, len));
                    data_write.set_active(i);
                }
                thread::sleep(*sleep_time);
            }
        }
    }
}