use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

use nannou::{
    draw::Draw,
    geom::point::Point2,
};

use crate::{tools, TWO_PI};
use super::commands::*;

const SWAP_SLEEP: Duration = Duration::from_millis(1);
const BUBBLE_SLEEP: Duration = Duration::from_secs(40);    // For 1 element/len squared
const SHELL_SLEEP: Duration = Duration::from_secs(300);
const QUICK_SLEEP: Duration = Duration::from_secs(5);
const RADIX_SLEEP: Duration = Duration::from_secs(2);

macro_rules! check_for_stop {
    ($data_arc:expr) => {
        if $data_arc.read().unwrap().sorted { break }
    };
}

macro_rules! start_sort_thread {
    ($self:expr, $data_arc:expr, $operation:block) => {
        $data_arc.write().unwrap().sorted = false;
        $self.sort_thread = Some(thread::spawn(move || {
            $operation;
            SortArray::reset_arr_info($data_arc);
        }));
    };
}

// Duplicate code in shell sort and comb sort. Not a function because of borrowing issues.
macro_rules! comb {
    ($data_arc:expr, $gap:expr, $len:expr, $sleep_time:expr) => {
        for i in $gap..$len {
            check_for_stop!($data_arc.clone());
            let temp = $data_arc.read().unwrap()[i];

            let mut j = i;
            while j >= $gap && $data_arc.read().unwrap()[j - $gap] > temp {
                {
                    let mut write = $data_arc.write().unwrap();
                    write[j] = write[j - $gap];
                }

                j -= $gap;
                thread::sleep($sleep_time);
            }
            $data_arc.write().unwrap()[j] = temp;
        }
    };
}


#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct DataArrWrapper {
    #[shrinkwrap(main_field)] pub arr: Vec<usize>,
    pub active: Option<usize>,
    pub pivot: Option<usize>,
    pub sorted: bool,
}

impl DataArrWrapper {
    pub fn new(arr: Vec<usize>) -> Self {
        Self {
            arr,
            active: None,
            pivot: None,
            sorted: true,
        }
    }
}

pub struct SortArray {
    pub data: Arc<RwLock<DataArrWrapper>>,
    pub max_val: usize,
    sort_thread: Option<thread::JoinHandle<()>>,
}

impl SortArray {
    pub fn new(num_of_lines: usize) -> SortArray {
        SortArray {
            data: Arc::new(RwLock::new(
                DataArrWrapper::new((0..num_of_lines).collect())
            )),
            max_val: num_of_lines,
            sort_thread: None,
        }
    }

    pub fn instruction(&mut self, instruction: SortInstruction) {
        let data_arc_cln = Arc::clone(&self.data);
        match instruction {
            SortInstruction::Shuffle(rounds) => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::shuffle(data_arc_cln.clone(), rounds);
                });
            },
            SortInstruction::BubbleSort => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::bubble_sort(data_arc_cln.clone());
                });
            },
            SortInstruction::QuickSort(partition_type) => {
                let len = self.data.read().unwrap().len();

                start_sort_thread!(self, data_arc_cln, {
                    match partition_type {
                        QuickSortType::LomutoPartitioning => {
                            Self::quick_sort(data_arc_cln.clone(), 0, len-1, len as u32);
                        }
                    }
                });
            },
            SortInstruction::InsertionSort => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::insertion_sort(data_arc_cln.clone());
                });
            },
            SortInstruction::ShellSort => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::shell_sort(data_arc_cln.clone());
                });
            },
            SortInstruction::CombSort => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::comb_sort(data_arc_cln.clone());
                });
            },
            SortInstruction::RadixSort(base) => {
                start_sort_thread!(self, data_arc_cln, {
                    Self::radix_lsd(data_arc_cln.clone(), base);
                });
            },

            SortInstruction::Reset => {
                self.reset();
            },
            SortInstruction::Reverse => {
                self.data.write().unwrap().sorted = false;
                self.data.write().unwrap().reverse();
            },
            SortInstruction::Stop => {
                self.data.write().unwrap().sorted = true;
            },
        }
    }

    #[inline]
    pub fn display(&self, draw: &Draw, index: usize, max_index: usize, array_len: usize, mode: DisplayMode, window_dims: (f32, f32), transform: (f32, f32)) {
        let data_read = self.data.read().unwrap();

        match mode {
            DisplayMode::Bars => {
                let scale = (window_dims.0/array_len as f32, window_dims.1/self.max_val as f32);

                for (i, d) in data_read.iter().enumerate() {
                    let x = (i as f32 * scale.0) + scale.0/2.0;
                    draw.line()
                        .x_y(transform.0, transform.1)
                        .start(Point2::new(x, 0.0))
                        .end(Point2::new(x, (*d as f32 + 1.0) * scale.1))
                        .thickness(scale.0)
                        .hsv((*d as f32/self.max_val as f32)/3.0, 1.0, 1.0);
                }
            },
            DisplayMode::Circle => {
                let radius = if window_dims.0 > window_dims.1 { window_dims.1 } else { window_dims.0 } / 2.0;

                let angle_interval = TWO_PI/array_len as f32;
                let mut angle = 0.0;

                for d in data_read.iter() {
                    let connecting_angle = angle + angle_interval;

                    draw.tri()
                        .points(
                            [0.0, 0.0],
                            tools::get_point_on_radius(radius, angle),
                            tools::get_point_on_radius(radius, connecting_angle)
                        )
                        .hsv(*d as f32/self.max_val as f32, 1.0, 1.0);

                    angle = connecting_angle;
                }
            },
            DisplayMode::Line => {
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
            },
            DisplayMode::Dots => {
                let scale = (window_dims.0/array_len as f32, window_dims.1/self.max_val as f32);

                for (i, d) in data_read.iter().enumerate() {
                    draw.ellipse()
                        .x_y(transform.0 + ((i as f32 * scale.0) + scale.0/2.0), transform.1 + ((*d as f32 + 0.5) * scale.1))
                        .radius(scale.0/2.0)
                        .hsv((*d as f32/self.max_val as f32)/3.0, 1.0, 1.0);
                }
            },
            DisplayMode::Pixels => {
                let scale = (window_dims.0/max_index as f32, window_dims.1/self.max_val as f32);

                let x = (index as f32 + 0.5) * scale.0;

                for (i, d) in data_read.iter().enumerate() {
                    draw.rect()
                        .x_y(transform.0 + x, transform.1 + (i as f32 + 0.5) * scale.1)
                        .w_h(scale.0, scale.1)
                        .hsv((1.0 - (*d as f32/self.max_val as f32))/3.0, 1.0, 1.0);
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
        write.pivot = None;
        write.sorted = true;
    }

    fn reset(&mut self) {
        let mut write = self.data.write().unwrap();
        write.sorted = true;
        write.arr = (0..write.len()).collect();
    }

    fn shuffle(data: Arc<RwLock<DataArrWrapper>>, passes: u16) {
        let len = data.read().unwrap().len();

        for _ in 0..passes {
            for i in 0..len {
                {
                    let mut data_write = data.write().unwrap();
                    data_write.swap(i, nannou::rand::random_range(0usize, len));
                    data_write.active = Some(i);
                }
                thread::sleep(SWAP_SLEEP/len as u32);
            }
        }
    }

    fn bubble_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let len = data_arc.read().unwrap().len();
        let mut sorted = false;

        while !sorted && !data_arc.read().unwrap().sorted {
            sorted = true;

            for i in 0..len-1 {
                check_for_stop!(data_arc);

                let (d1, d2) = {
                    let read = data_arc.read().unwrap();
                    (read[i], read[i+1])
                };
                if d1 > d2 {
                    {
                        let mut data_write = data_arc.write().unwrap();
                        data_write.swap(i, i+1);
                    }
                    sorted = false;
                    thread::sleep(BUBBLE_SLEEP/len.pow(2) as u32);
                }
            }
        }
    }

    fn quick_sort(data_arc: Arc<RwLock<DataArrWrapper>>, low: usize, high: usize, data_len: u32) {
        // Lomuto partition scheme: https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
        // Pretty much copied the pseudocode
        fn partition(data_arc: Arc<RwLock<DataArrWrapper>>, low: usize, high: usize, data_len: u32) -> usize {
            let pivot = data_arc.read().unwrap()[high];

            let mut i = low;
            for j in low..high {
                check_for_stop!(data_arc);

                if data_arc.read().unwrap()[j] < pivot {
                    data_arc.write().unwrap().swap(i, j);
                    i += 1;
                    thread::sleep(QUICK_SLEEP/data_len);
                }
            }

            data_arc.write().unwrap().swap(i, high);
            i
        }

        if low < high {  // Not equal
            let p = partition(data_arc.clone(), low, high, data_len);
            if p > 0 {
                Self::quick_sort(data_arc.clone(), low, p - 1, data_len);
            }
            if p < high {
                Self::quick_sort(data_arc.clone(), p + 1, high, data_len);
            }   
        }
    }

    fn insertion_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let len = data_arc.read().unwrap().len();
        let sleep_time = BUBBLE_SLEEP/len.pow(2) as u32;
        
        for i in 1..len {
            check_for_stop!(data_arc);
            for j in (1..i+1).rev() {
                {
                    let read = data_arc.read().unwrap();
                    if read.sorted ||  read[j-1] < read[j] {
                        break
                    }
                }
                data_arc.write().unwrap().swap(j, j-1);
                thread::sleep(sleep_time);
            }
        }
    }

    fn shell_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        pub struct ShellSortGapsIter {      // Iterator to generate gaps
            count: usize,
        }

        impl Default for ShellSortGapsIter {
            fn default() -> ShellSortGapsIter {
                ShellSortGapsIter {
                    count: 1,
                }
            }
        }

        impl Iterator for ShellSortGapsIter {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                // 2^k - 1
                let next_val = (2_usize).pow(self.count as u32) - 1;
                self.count += 1;

                Some(next_val)
            }
        }

        let len = data_arc.read().unwrap().len();
        let sleep_time = SHELL_SLEEP/len.pow(2) as u32;

        let gaps: Vec<usize> = ShellSortGapsIter::default().take_while(|i| *i < len).collect();

        for gap in gaps.into_iter().rev() {
            check_for_stop!(data_arc);
            comb!(data_arc, gap, len, sleep_time);
        }
    }

    fn comb_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let len = data_arc.read().unwrap().len();
        let mut comb_len = len/2;

        let sleep_time = SHELL_SLEEP/len.pow(2) as u32;

        while comb_len >= 1 {
            check_for_stop!(data_arc);
            comb!(data_arc, comb_len, len, sleep_time);
            comb_len /= 2;
        }    
    }

    fn radix_lsd(data_arc: Arc<RwLock<DataArrWrapper>>, base: usize) {
        use std::collections::HashMap;
        use radix::RadixNum;

        #[inline]
        fn get_max_digits(array: &[usize], base: usize) -> usize {
            let mut largest = 0usize;

            for item in array.iter() {
                let item_digits = RadixNum::from(*item)
                    .with_radix(base)
                    .unwrap()
                    .as_str()
                    .len();

                if item_digits > largest {
                    largest = item_digits;
                }
            }
            largest
        }

        #[inline]
        fn get_digit_at(num: usize, i: usize, base: usize) -> usize {
            (num/base.pow(i as u32)) % base
        }

        let (largest_digits, array_len) = {
            let data_read = data_arc.read().unwrap();
            (get_max_digits(&data_read, base), data_read.len())
        };

        for digit_num in 0..largest_digits {
            // Counting sort
            let mut buckets: HashMap<usize, Vec<usize>> = HashMap::new();

            {
                let data_read = data_arc.read().unwrap();

                for num in data_read.iter() {
                    let digit = get_digit_at(*num, digit_num, base);
                    let bucket = buckets.entry(digit)
                        .or_insert(Vec::with_capacity(array_len));
                    
                    bucket.push(*num);
                }
            }

            let mut i = 0;
            for key in 0..base {
                if let Some(bucket) = buckets.get(&key) {
                    for element in bucket.iter() {
                        {
                            let mut write = data_arc.write().unwrap();
                            write[i] = *element;
                            if write.sorted {
                                return
                            }
                        }
                        
                        i += 1;
                        thread::sleep(RADIX_SLEEP/array_len as u32);
                    }
                }
            }
        }
    }
}