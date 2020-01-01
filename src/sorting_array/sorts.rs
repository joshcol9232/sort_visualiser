use super::DataArrWrapper;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

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

// Shared by bubble sort and cocktail shaker sort.
macro_rules! bubble {
    ($data_arc:expr, $swapped:expr, $i:expr, $sleep_time:expr) => {
        $data_arc.write().unwrap().set_active($i + 1);

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
            thread::sleep(*$sleep_time);
        }
    };
}

pub fn bubble_sort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration) {
    let len = data_arc.read().unwrap().len();
    let mut swapped = true;

    while swapped && !data_arc.read().unwrap().sorted {
        swapped = false;

        for i in 0..len - 1 {
            check_for_stop!(data_arc);
            bubble!(data_arc, swapped, i, sleep_time);
        }
    }
}

pub fn cocktail_shaker_sort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration) {
    let len = data_arc.read().unwrap().len();
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

pub fn insertion_sort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration, start: usize, end: usize) { // end is inclusive
    for i in start..end+1 {
        check_for_stop!(data_arc);
        data_arc.write().unwrap().set_pivot(i);

        for j in (start+1..=i).rev() {
            data_arc.write().unwrap().set_active(j);
            {
                let read = data_arc.read().unwrap();
                if read.sorted || read[j - 1] < read[j] {
                    break;
                }
            }
            data_arc.write().unwrap().swap(j, j - 1);
            thread::sleep(*sleep_time);
        }
    }
}

pub fn selection_sort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration) {
    let len = data_arc.read().unwrap().len();

    for done in 0..len-1 {
        check_for_stop!(data_arc);
        data_arc.write().unwrap().set_active_2(done);

        let mut min = (done, data_arc.read().unwrap()[done]); // (index, value) of minumum value in current part of list
        for i in done+1..len {
            check_for_stop!(data_arc);
            data_arc.write().unwrap().set_active(i);
            let val = data_arc.read().unwrap()[i];
            if val < min.1 {    // If value less than curent minimum
                min = (i, val);
                data_arc.write().unwrap().set_pivot(i);
            }
            thread::sleep(*sleep_time);
        }

        // Swap minumum with element at done
        data_arc.write().unwrap().swap(min.0, done);
    }
}

pub fn shell_sort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration) {
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

    let gaps: Vec<usize> = ShellSortGapsIter::default()
        .take_while(|i| *i < len)
        .collect();

    for gap in gaps.into_iter().rev() {
        check_for_stop!(data_arc);
        for i in gap..len {
            check_for_stop!(data_arc.clone());
            let temp = data_arc.read().unwrap()[i];

            let mut j = i;
            while j >= gap && data_arc.read().unwrap()[j - gap] > temp {
                {
                    let mut write = data_arc.write().unwrap();
                    write.set_active(j - gap);
                    write.set_active_2(j);
                    write[j] = write[j - gap];
                }

                j -= gap;
                thread::sleep(*sleep_time);
            }
            data_arc.write().unwrap()[j] = temp;
        }
    }
}

pub fn radix_lsd(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: &Duration, base: usize) {
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
                    .or_insert_with(|| Vec::with_capacity(array_len));

                bucket.push(*num);
            }
        }

        let mut i = 0;
        for key in 0..base {
            if let Some(bucket) = buckets.get(&key) {
                for element in bucket.iter() {
                    {
                        let mut write = data_arc.write().unwrap();
                        write.set_active(i);
                        write[i] = *element;
                        if write.sorted {
                            return;
                        }
                    }

                    i += 1;
                    thread::sleep(*sleep_time);
                }
            }
        }
    }
}

pub mod quick_sorting {
    use std::time::Duration;
    use std::thread;
    use std::sync::{Arc, RwLock};
    use super::*;

    const MAX_RUN_SIZE: usize = 16;     // Used in quicktimsort. If the array given is less than MAX_RUN_SIZE in length, then sort with insertion sort

    // Lomuto partition scheme: https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
    #[inline]
    fn lomuto_partitioning(
        data_arc: Arc<RwLock<DataArrWrapper>>,
        sleep_time: Arc<Duration>,
        l: usize,
        r: usize,
    ) -> usize {
        let pivot = data_arc.read().unwrap()[r];
        data_arc.write().unwrap().set_pivot(r);

        let mut i = l;
        for j in l..r {
            check_for_stop_break!(data_arc);
            {
                // Update active info
                let mut write = data_arc.write().unwrap();
                write.set_active(i);
                write.set_active_2(j);
            }

            if data_arc.read().unwrap()[j] < pivot {
                data_arc.write().unwrap().swap(i, j);
                i += 1;
                thread::sleep(*sleep_time);
            }
        }

        data_arc.write().unwrap().swap(i, r);
        i
    }

    pub fn quick_sort_lomuto(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {
        if l < r {
            // Not equal
            let p = lomuto_partitioning(data_arc.clone(), sleep_time.clone(), l, r);
            if p > 0 {
                quick_sort_lomuto(data_arc.clone(), sleep_time.clone(), l, p - 1);
            }
            if p < r {
                quick_sort_lomuto(data_arc.clone(), sleep_time, p + 1, r);
            }
        }
    }

    pub fn quick_sort_lomuto_multithreaded(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {
        if l < r {
            let mut child_threads: Vec<thread::JoinHandle<()>> = Vec::new();

            // Not equal
            let p = lomuto_partitioning(data_arc.clone(), sleep_time.clone(), l, r);
            if p > 0 {
                let cln = data_arc.clone();
                let slp_cln = sleep_time.clone();
                child_threads.push(thread::spawn(move || {
                    quick_sort_lomuto_multithreaded(cln, slp_cln, l, p - 1);
                }));
            }
            if p < r {
                let cln = data_arc.clone();
                let slp_cln = sleep_time.clone();
                child_threads.push(thread::spawn(move || {
                    quick_sort_lomuto_multithreaded(cln, slp_cln, p + 1, r);
                }));
            }

            for child in child_threads {
                child.join().unwrap();
            }
        } 
    }

    // Like timsort but for quicksort instead (because why not)
    // Does regular quicksort until the array size becomes less than MAX_RUN_SIZE, where it then switches to insertion
    // sort, since insertion sort works well with small arrays.
    pub fn quicktimsort(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {        
        if l < r {
            if r - l < MAX_RUN_SIZE {
                insertion_sort(data_arc, &sleep_time, l, r);
            } else {
                let p = lomuto_partitioning(data_arc.clone(), sleep_time.clone(), l, r);
                if p > 0 {
                    quicktimsort(data_arc.clone(), sleep_time.clone(), l, p - 1);
                }
                if p < r {
                    quicktimsort(data_arc.clone(), sleep_time, p + 1, r);
                }
            }
        }
    }

    pub fn quicktimsort_multithreaded(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {
        if l < r {
            if r - l < MAX_RUN_SIZE {
                insertion_sort(data_arc, &sleep_time, l, r);
            } else {
                let mut child_threads: Vec<thread::JoinHandle<()>> = Vec::new();

                // Not equal
                let p = lomuto_partitioning(data_arc.clone(), sleep_time.clone(), l, r);
                if p > 0 {
                    let cln = data_arc.clone();
                    let slp_cln = sleep_time.clone();
                    child_threads.push(thread::spawn(move || {
                        quick_sort_lomuto_multithreaded(cln, slp_cln, l, p - 1);
                    }));
                }
                if p < r {
                    let cln = data_arc.clone();
                    let slp_cln = sleep_time.clone();
                    child_threads.push(thread::spawn(move || {
                        quick_sort_lomuto_multithreaded(cln, slp_cln, p + 1, r);
                    }));
                }

                for child in child_threads {
                    child.join().unwrap();
                }
            }
        }
    }
}

// Works kind of like pushing the left array into the right array.
// Outside of "merge_sorting" sub module due to it's use in TimSort.
fn merge_in_place(
    data_arc: Arc<RwLock<DataArrWrapper>>,
    sleep_time: Arc<Duration>,
    mut start: usize,
    mut mid: usize,
    end: usize,
) {
    let mut start2 = mid + 1;

    let mid_less_than_start2 = {
        let read = data_arc.read().unwrap();
        read[mid] <= read[start2]
    };
    if mid_less_than_start2 {
        return; // Exit
    }

    while start <= mid && start2 <= end {
        check_for_stop!(data_arc);

        let start_less_than_start2 = {
            let read = data_arc.read().unwrap();
            read[start] <= read[start2]
        };
        if start_less_than_start2 { // Then it is in the correct place.
            start += 1;
        } else {
            // if element 1 is not in the right place, move it until it is.
            let value = data_arc.read().unwrap()[start2]; // Element 2
            let mut index = start2;

            {
                // Shift all elements between element 1 and element 2 right by 1 to insert this element.
                let mut write = data_arc.write().unwrap();
                write.set_pivot(start2);

                while index != start {
                    write.set_active(index);
                    write[index] = write[index - 1];
                    index -= 1;
                }
                write[start] = value;
            }
            start += 1;
            mid += 1;
            start2 += 1;

            thread::sleep(*sleep_time);
        }
    }
}

pub mod merge_sorting {
    use std::time::Duration;
    use std::thread;
    use std::sync::{Arc, RwLock};
    use super::*;

    pub fn merge_sort_in_place(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {
        if l < r {
            check_for_stop!(data_arc);

            let m = (l + r) / 2;

            merge_sort_in_place(data_arc.clone(), sleep_time.clone(), l, m);
            merge_sort_in_place(data_arc.clone(), sleep_time.clone(), m + 1, r);

            merge_in_place(data_arc, sleep_time, l, m, r);
        }
    }

    pub fn merge_sort_in_place_multithreaded(data_arc: Arc<RwLock<DataArrWrapper>>, sleep_time: Arc<Duration>, l: usize, r: usize) {
        if l < r {
            check_for_stop!(data_arc);
            let mut child_threads: Vec<thread::JoinHandle<()>> = Vec::new();

            let m = (l + r) / 2;

            let cln = data_arc.clone();
            let slp_cln = sleep_time.clone();
            child_threads.push(thread::spawn(move || {
                merge_sort_in_place_multithreaded(cln, slp_cln, l, m)
            }));

            let cln = data_arc.clone();
            let slp_cln = sleep_time.clone();
            child_threads.push(thread::spawn(move || {
                merge_sort_in_place_multithreaded(cln, slp_cln, m + 1, r)
            }));

            for child in child_threads {
                child.join().unwrap();
            }

            merge_in_place(data_arc, sleep_time, l, m, r);
        }
    }
}
