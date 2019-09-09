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
