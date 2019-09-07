use super::DataArrWrapper;

use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::thread;

const BUBBLE_SLEEP: Duration = Duration::from_secs(80);    // For 1 element/len squared
const SHELL_SLEEP: Duration = Duration::from_secs(400);
const QUICK_SLEEP: Duration = Duration::from_secs(6);
const RADIX_SLEEP: Duration = Duration::from_secs(2);

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

macro_rules! check_for_stop {   // Place inside loop
    ($data_arc:expr) => {
        if $data_arc.read().unwrap().sorted { break }
    };
}

pub fn bubble_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let mut sorted = false;

    while !sorted && !data_arc.read().unwrap().sorted {
        sorted = true;

        for i in 0..len-1 {
            check_for_stop!(data_arc);
            data_arc.write().unwrap().active = Some(i+1);

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

pub fn quick_sort(data_arc: Arc<RwLock<DataArrWrapper>>, low: usize, high: usize, data_len: u32) {
    // Lomuto partition scheme: https://en.wikipedia.org/wiki/Quicksort#Lomuto_partition_scheme
    // Pretty much copied the pseudocode
    fn partition(data_arc: Arc<RwLock<DataArrWrapper>>, low: usize, high: usize, data_len: u32) -> usize {
        let pivot = data_arc.read().unwrap()[high];
        data_arc.write().unwrap().pivot = Some(high);

        let mut i = low;
        for j in low..high {
            check_for_stop!(data_arc);
            {   // Update active info
                let mut write = data_arc.write().unwrap();
                write.active = Some(i);
                write.active_2 = Some(j);
            }

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
            quick_sort(data_arc.clone(), low, p - 1, data_len);
        }
        if p < high {
            quick_sort(data_arc.clone(), p + 1, high, data_len);
        }   
    }
}

pub fn insertion_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let sleep_time = BUBBLE_SLEEP/len.pow(2) as u32;
    
    for i in 1..len {
        check_for_stop!(data_arc);
        data_arc.write().unwrap().pivot = Some(i);

        for j in (1..i+1).rev() {
            data_arc.write().unwrap().active = Some(j);
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

pub fn shell_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
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

pub fn comb_sort(data_arc: Arc<RwLock<DataArrWrapper>>) {
    let len = data_arc.read().unwrap().len();
    let mut comb_len = len/2;

    let sleep_time = SHELL_SLEEP/len.pow(2) as u32;

    while comb_len >= 1 {
        check_for_stop!(data_arc);
        comb!(data_arc, comb_len, len, sleep_time);
        comb_len /= 2;
    }    
}

pub fn radix_lsd(data_arc: Arc<RwLock<DataArrWrapper>>, base: usize) {
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
                        write.active = Some(i);
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