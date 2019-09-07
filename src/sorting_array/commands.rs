// Commands and options
#[derive(Copy, Clone)]
pub enum SortInstruction {
    Shuffle(u16),
    Reset,
    Reverse,
    Stop,

    BubbleSort,
    CocktailShakerSort,
    QuickSort(QuickSortType),
    InsertionSort,
    ShellSort,
    CombSort,
    RadixSort(usize),
}

#[derive(Copy, Clone)]
pub enum QuickSortType {
    LomutoPartitioning,
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