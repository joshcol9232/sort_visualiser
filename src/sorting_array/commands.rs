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
    CombSort,

    QuickSort(QuickSortType),
    MergeSort(MergeSortType),
    
    RadixSort(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum QuickSortType {
    Lomuto {
        multithreaded: bool,
    },
}

impl FromStr for QuickSortType {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        match s.to_lowercase().as_str() {
            "lomuto" => Ok(QuickSortType::Lomuto { multithreaded: false }),
            "lomuto_multi" => Ok(QuickSortType::Lomuto { multithreaded: true }),
            x => Err(io::Error::new(
                ErrorKind::Other,
                format!("Invalid quicksort_partitioning format in config file: {}. Options are: lomuto, lomuto_multi", x)
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

#[derive(Clone, Copy)]
pub enum DisplayMode {
    Bars,
    Circle,
    // Line,    // Not currently working due to nannou
    Dots,
    Pixels,
}

impl Default for DisplayMode {
    fn default() -> DisplayMode {
        DisplayMode::Circle
    }
}
