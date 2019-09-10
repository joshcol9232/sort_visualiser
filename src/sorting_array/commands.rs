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
    MergeSort,
    
    RadixSort(usize),
}

#[derive(Copy, Clone)]
pub enum QuickSortType {
    Lomuto,
}

impl FromStr for QuickSortType {
    type Err = io::Error;

    fn from_str(s: &str) -> io::Result<Self> {
        match s.to_lowercase().as_str() {
            "lomuto" => Ok(QuickSortType::Lomuto),
            x => Err(io::Error::new(ErrorKind::Other, format!("Invalid quicksort_partitioning format in config file: {}. Options are: lomuto", x))),
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
