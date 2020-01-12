use std::str::FromStr;
use std::io::{self, ErrorKind};

// Commands and options
#[derive(Copy, Clone)]
pub enum SortInstruction {
    Shuffle(u16),
    Reset,
    Reverse,
    Stop,

    BubbleSort,
    CocktailShakerSort,
    InsertionSort,
    SelectionSort,

    ShellSort,

    QuickSort(QuickSortType),
    MergeSort(MergeSortType),
    
    RadixSort(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum QuickSortType {
    Lomuto {
        multithreaded: bool,
		insertion_hybrid: bool,
    },
}

impl FromStr for QuickSortType {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        match s.to_lowercase().as_str() {
            "lomuto" => Ok(QuickSortType::Lomuto { multithreaded: false, insertion_hybrid: false }),
            "lomuto_multi" => Ok(QuickSortType::Lomuto { multithreaded: true, insertion_hybrid: false }),
            "lomuto_insertion_hybrid" => Ok(QuickSortType::Lomuto { multithreaded: false, insertion_hybrid: true }),
            "lomuto_insertion_hybrid_multi" => Ok(QuickSortType::Lomuto { multithreaded: true, insertion_hybrid: true }),
            x => Err(io::Error::new(
                ErrorKind::Other,
                format!("Invalid quicksort_partitioning format in config file: {}. Options are: lomuto, lomuto_multi, lomuto_insertion_hybrid, lomuto_insertion_hybrid_multi", x)
            )),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MergeSortType {
    InPlace {
        multithreaded: bool
    },
}

impl FromStr for MergeSortType {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        match s.to_lowercase().as_str() {
            "in_place" => Ok(MergeSortType::InPlace { multithreaded: false }),
            "in_place_multi" => Ok(MergeSortType::InPlace { multithreaded: true }),
            x => Err(
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("Invalid merge_sort_type format in config file: {}. Options are: in_place, in_place_multi", x)
                )
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Bars,
    Pyramid,
    Circle,
    Doughnut,
    Dots,

    DisparityLine,  // How far away it is from the place that it should be
    DisparityLoop,

    Pixels,
}

impl Default for DisplayMode {
    fn default() -> DisplayMode {
        DisplayMode::Circle
    }
}
