use std::time::Duration;
use std::thread;
use std::sync::{Arc, RwLock};

use nannou::{
    draw::Draw,
    geom::point::Point2,
};

use crate::{tools, TWO_PI};

mod shell_sort {
    pub struct ShellSortGapsIter {
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
}

const SWAP_SLEEP: Duration = Duration::from_millis(1);
const BUBBLE_SLEEP: Duration = Duration::from_secs(40);    // For 1 element/len squared
const QUICK_SLEEP: Duration = Duration::from_secs(5);

#[derive(Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct DataArrWrapper {
    #[shrinkwrap(main_field)] pub arr: Vec<usize>,
    pub active: Option<usize>,
    pub pivot: Option<usize>,
}

impl DataArrWrapper {
    pub fn new(arr: Vec<usize>) -> Self {
        Self {
            arr,
            active: None,
            pivot: None,
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
                self.sort_thread = Some(thread::spawn(move || {
                    Self::shuffle(data_arc_cln.clone(), rounds);
                    Self::reset_arr_info(data_arc_cln);
                }));
            },
            SortInstruction::BubbleSort => {
                self.sort_thread = Some(thread::spawn(move || {
                    Self::bubble_sort(data_arc_cln.clone());
                    Self::reset_arr_info(data_arc_cln);
                }));
            },
            SortInstruction::QuickSort(partition_type) => {
                let len = self.data.read().unwrap().len();
                match partition_type {
                    QuickSortType::LomutoPartitioning => {
                        self.sort_thread = Some(thread::spawn(move || {
                            Self::quick_sort(data_arc_cln.clone(), 0, len-1, len as u32);
                            Self::reset_arr_info(data_arc_cln);
                        }));
                    },
                    QuickSortType::Overwriting => {
                        self.sort_thread = Some(thread::spawn(move || {
                            Self::overwriting_quicksort(data_arc_cln.clone(), 0, len, len as u32);
                            Self::reset_arr_info(data_arc_cln);
                        }));
                    }
                }
            },
            SortInstruction::InsertionSort => {
                self.sort_thread = Some(thread::spawn(move || {
                    Self::insertion_sort(data_arc_cln.clone());
                    Self::reset_arr_info(data_arc_cln);
                }));
            },
            SortInstruction::ShellSort => {
                self.sort_thread = Some(thread::spawn(move || {
                    Self::shell_sort(data_arc_cln.clone());
                    Self::reset_arr_info(data_arc_cln);
                }));
            },

            SortInstruction::Reset => {
                self.data.write().unwrap().sort_by(|a, b| a.cmp(b));
            },
            SortInstruction::Reverse => {
                self.data.write().unwrap().reverse();
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
                /* Polyline is broken in nannou atm, so waiting for change to lyon which they are implementing.
                    see: https://github.com/nannou-org/nannou/issues/185
                
                let mut points: Vec<vertex::Srgba> = Vec::with_capacity(DATA_LEN);
                let scale = (window_dims.0/data_read.len() as f32, window_dims.1/self.max_val as f32);
                
                for (i, d) in data_read.iter().enumerate() {
                    let col = self.get_color(i, d, 360.0, 0.0);
                    points.push(
                        vertex::Srgba(
                            [(i as f32 * scale.0) + scale.0/2.0 + transform.0, (*d as f32 + 1.0) * scale.1 + transform.1].into(),
                            col.into()
                        )
                    );
                }

                draw.polyline()
                    .vertices(1.0, points);
                */
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

    fn write_over(data_arc: Arc<RwLock<DataArrWrapper>>, overwrite_data: &[usize], start_index: usize, end_index: usize, sleep: Option<Duration>) {
        assert!(end_index > start_index);
        let len = end_index - start_index;
        assert!(overwrite_data.len() == len);
        assert!(data_arc.read().unwrap().len() >= overwrite_data.len());

        for (i, data_index) in (start_index..end_index).enumerate() {
            data_arc.write().unwrap()[data_index] = overwrite_data[i];
            if let Some(sleep_time) = sleep {
                thread::sleep(sleep_time);
            }
        }
    }

    fn bubble_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let len = data_arc.read().unwrap().len();

        let mut sorted = false;

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

    fn overwriting_quicksort(data_arc: Arc<RwLock<DataArrWrapper>>, l: usize, h: usize, data_len: u32) {
        assert!(h > l);
        assert!(h > 0);
        let len = h - l;

        if len < 2 {
            return
        }

        let mut left = Vec::with_capacity(len);
        let mut mid = vec![];
        let mut right = Vec::with_capacity(len);
        let pivot = data_arc.read().unwrap()[l + len/2];

        for index in l..h {
            let element = data_arc.read().unwrap()[index];

            if element < pivot {
                left.push(element);
            } else if element > pivot {
                right.push(element);
            } else {
                mid.push(element);
            }
        }

        let lens = [left.len(), mid.len(), right.len()];

        // Write partitioning
        left.append(&mut mid);
        left.append(&mut right);
        Self::write_over(
            data_arc.clone(),
            left.as_slice(),
            l,
            h,
            Some(QUICK_SLEEP/data_len)
        );

        if lens[0] > 0 {
            Self::overwriting_quicksort(data_arc.clone(), l, l + lens[0], data_len);  // Sort lower
        }
        if lens[1] > 0 {
            Self::overwriting_quicksort(data_arc.clone(), l + lens[0], l + lens[0] + lens[1], data_len);  // Sort mid
        }
        if lens[2] > 0 {
            Self::overwriting_quicksort(data_arc.clone(), l + lens[0] + lens[1], l + lens[0] + lens[1] + lens[2], data_len);  // Sort right
        }
    }

    fn insertion_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        let len = data_arc.read().unwrap().len();
        let sleep_time = BUBBLE_SLEEP/len.pow(2) as u32;
        
        for i in 1..len {
            for j in (1..i+1).rev() {
                {
                    let read = data_arc.read().unwrap();
                    if read[j-1] < read[j] {
                        break
                    }
                }
                data_arc.write().unwrap().swap(j, j-1);
                thread::sleep(sleep_time);
            }
        }
    }

    fn shell_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
        use shell_sort::ShellSortGapsIter;

        let len = data_arc.read().unwrap().len();
        let sleep_time = BUBBLE_SLEEP/len.pow(2) as u32;

        let gaps: Vec<usize> = ShellSortGapsIter::default().take_while(|i| *i < len).collect();
        dbg!(&gaps);

        for gap in gaps.into_iter().rev() {
            println!("Gap: {}", gap);
            for i in gap..len {
                let temp = data_arc.read().unwrap()[i];

                let mut j = i;
                while j >= gap && data_arc.read().unwrap()[j - gap] > temp {
                    {
                        let mut write = data_arc.write().unwrap();
                        write[j] = write[j - gap];
                    }

                    j -= gap;
                    thread::sleep(sleep_time);
                }
                data_arc.write().unwrap()[j] = temp;
            }
        }
    }
}


// Commands and options

#[derive(Copy, Clone)]
pub enum SortInstruction {
    Shuffle(u16),
    Reset,
    Reverse,

    BubbleSort,
    QuickSort(QuickSortType),
    InsertionSort,
    ShellSort,
}

#[derive(Copy, Clone)]
pub enum QuickSortType {
    LomutoPartitioning,
    Overwriting,
}

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Bars,
    Circle,
    Line,
    Dots,
    Pixels,
}

impl Default for DisplayMode {
    fn default() -> DisplayMode {
        DisplayMode::Circle
    }
}
