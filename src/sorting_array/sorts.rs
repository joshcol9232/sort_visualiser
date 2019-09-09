use super::DataArrWrapper;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// NOTE: Sorts of the same/simmilar time complexity will use the same sleep time.
const BUBBLE_SLEEP: Duration = Duration::from_secs(80); // For 1 element/len squared
const SELECTION_SLEEP: Duration = Duration::from_millis(500);   // Takes so long to scan array so it looks like it isn't doing anything, so needs to be short
const SHELL_SLEEP: Duration = Duration::from_secs(400);
const QUICK_SLEEP: Duration = Duration::from_secs(6);
const RADIX_SLEEP: Duration = Duration::from_secs(2);

macro_rules! check_for_stop {
    // Returns from function if sorted
    ($data_arc:expr) => {
        if $data_arc.read().unwrap().sorted {
            return;
        }
    };
}

macro_rules! check_for_stop_break {
    // Only does a break, so exits loop.
    ($data_arc:expr) => {
        if $data_arc.read().unwrap().sorted {
            break;
        }
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
                    write.active = Some(j - $gap);
                    write.active_2 = Some(j);
                    write[j] = write[j - $gap];
                }

                j -= $gap;
                thread::sleep($sleep_time);
            }
            $data_arc.write().unwrap()[j] = temp;
        }
    };
}

// Shared by bubble sort and cocktail shaker sort.
macro_rules! bubble {
    ($data_arc:expr, $swapped:expr, $i:expr, $sleep_time:expr) => {
        $data_arc.write().unwrap().active = Some($i + 1);

        let (d1, d2) = {
            let read = $data_arc.read().unwrap();
            (read[$i], read[$i + 1])
        };
        if d1 > d2 {
            {
                let mut data_write = $data_arc.write().unwrap();
                data_write.swap($i, $i + 1);
            }
            $swapped = true;
            thread::sleep($sleep_time);
        }
    };
}

pub fn bubble_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let sleep_time = BUBBLE_SLEEP / len.pow(2) as u32;
    let mut swapped = true;

    while swapped && !data_arc.read().unwrap().sorted {
        swapped = false;

        for i in 0..len - 1 {
            check_for_stop!(data_arc);
            bubble!(data_arc, swapped, i, sleep_time);
        }
    }
}

pub fn cocktail_shaker_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let sleep_time = BUBBLE_SLEEP / len.pow(2) as u32;
    let mut swapped = true; // if an element was swapped

    while swapped && !data_arc.read().unwrap().sorted {
        swapped = false;

        for i in 0..len - 1 {
            check_for_stop!(data_arc);
            bubble!(data_arc, swapped, i, sleep_time);
        }

        if swapped {
            swapped = false;
            for i in (1..len - 1).rev() {
                check_for_stop!(data_arc);
                bubble!(data_arc, swapped, i, sleep_time);
            }
        }
    }
}

pub fn quick_sort_lomuto(data_arc: Arc<RwLock<DataArrWrapper>>, l: usize, r: usize, data_len: u32) {
    // Lomuto partition scheme: https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
    // Pretty much copied the pseudocode
    #[inline]
    fn partition(
        data_arc: Arc<RwLock<DataArrWrapper>>,
        l: usize,
        r: usize,
        data_len: u32,
    ) -> usize {
        let pivot = data_arc.read().unwrap()[r];
        data_arc.write().unwrap().pivot = Some(r);

        let mut i = l;
        for j in l..r {
            check_for_stop_break!(data_arc);
            {
                // Update active info
                let mut write = data_arc.write().unwrap();
                write.active = Some(i);
                write.active_2 = Some(j);
            }

            if data_arc.read().unwrap()[j] < pivot {
                data_arc.write().unwrap().swap(i, j);
                i += 1;
                thread::sleep(QUICK_SLEEP / data_len);
            }
        }

        data_arc.write().unwrap().swap(i, r);
        i
    }

    if l < r {
        // Not equal
        let p = partition(data_arc.clone(), l, r, data_len);
        if p > 0 {
            quick_sort_lomuto(data_arc.clone(), l, p - 1, data_len);
        }
        if p < r {
            quick_sort_lomuto(data_arc.clone(), p + 1, r, data_len);
        }
    }
}

pub fn insertion_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let sleep_time = BUBBLE_SLEEP / len.pow(2) as u32;

    for i in 1..len {
        check_for_stop!(data_arc);
        data_arc.write().unwrap().pivot = Some(i);

        for j in (1..i + 1).rev() {
            data_arc.write().unwrap().active = Some(j);
            {
                let read = data_arc.read().unwrap();
                if read.sorted || read[j - 1] < read[j] {
                    break;
                }
            }
            data_arc.write().unwrap().swap(j, j - 1);
            thread::sleep(sleep_time);
        }
    }
}

pub fn selection_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();

    for done in 0..len-1 {
        data_arc.write().unwrap().active_2 = Some(done);

        let mut min = (done, data_arc.read().unwrap()[done]); // (index, value) of minumum value in current part of list
        for i in done+1..len {
            data_arc.write().unwrap().active = Some(i);
            let val = data_arc.read().unwrap()[i];
            if val < min.1 {    // If value less than curent minimum
                min = (i, val);
                data_arc.write().unwrap().pivot = Some(i);
            }
            thread::sleep(SELECTION_SLEEP/len as u32);
        }

        // Swap minumum with element at done
        data_arc.write().unwrap().swap(min.0, done);
    }
}

pub fn shell_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    pub struct ShellSortGapsIter {
        // Iterator to generate gaps
        count: usize,
    }

    impl Default for ShellSortGapsIter {
        fn default() -> ShellSortGapsIter {
            ShellSortGapsIter { count: 1 }
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
    let sleep_time = SHELL_SLEEP / len.pow(2) as u32;

    let gaps: Vec<usize> = ShellSortGapsIter::default()
        .take_while(|i| *i < len)
        .collect();

    for gap in gaps.into_iter().rev() {
        check_for_stop!(data_arc);
        comb!(data_arc, gap, len, sleep_time);
    }
}

pub fn comb_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let mut comb_len = len / 2;

    let sleep_time = SHELL_SLEEP / len.pow(2) as u32;

    while comb_len >= 1 {
        check_for_stop!(data_arc);
        comb!(data_arc, comb_len, len, sleep_time);
        comb_len /= 2;
    }
}

pub fn radix_lsd(data_arc: Arc<RwLock<DataArrWrapper>>, base: usize) {
    use radix::RadixNum;
    use std::collections::HashMap;

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
        (num / base.pow(i as u32)) % base
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
                let bucket = buckets
                    .entry(digit)
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
                        write.active = Some(i);
                        write[i] = *element;
                        if write.sorted {
                            return;
                        }
                    }

                    i += 1;
                    thread::sleep(RADIX_SLEEP / array_len as u32);
                }
            }
        }
    }
}

pub fn merge_sort(data_arc: Arc<RwLock<DataArrWrapper>>, l: usize, r: usize, data_len: u32) {
    // Works kind of like pushing the left array into the right array.
    fn merge(
        data_arc: Arc<RwLock<DataArrWrapper>>,
        mut start: usize,
        mut mid: usize,
        end: usize,
        data_len: u32,
    ) {
        let mut start2 = mid + 1;

        if {
            let read = data_arc.read().unwrap();
            read[mid] <= read[start2]
        } {
            return;
        }

        while start <= mid && start2 <= end {
            check_for_stop!(data_arc);

            if {
                let read = data_arc.read().unwrap();
                read[start] <= read[start2]
            } {
                start += 1;
            } else {
                // if element 1 is not in the right place, move it until it is.
                let value = data_arc.read().unwrap()[start2]; // Element 2
                let mut index = start2;

                {
                    // Shift all elements between element 1 and element 2 right by 1 to insert this element.
                    let mut write = data_arc.write().unwrap();
                    write.pivot = Some(start2);

                    while index != start {
                        write.active = Some(index);
                        write[index] = write[index - 1];
                        index -= 1;
                    }
                    write[start] = value;
                }
                start += 1;
                mid += 1;
                start2 += 1;

                thread::sleep(QUICK_SLEEP / data_len);
            }
        }
    }

    if l < r {
        check_for_stop!(data_arc);

        let m = (l + r) / 2;

        merge_sort(data_arc.clone(), l, m, data_len);
        merge_sort(data_arc.clone(), m + 1, r, data_len);

        merge(data_arc, l, m, r, data_len);
    }
}