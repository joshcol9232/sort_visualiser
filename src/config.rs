use yaml_rust::Yaml;
use std::time::Duration;
use std::sync::Arc;
use std::str::FromStr;

use crate::sorting_array::{QuickSortType, MergeSortType};

#[derive(Debug)]
pub struct Config {
    pub array_len: usize,
    pub multi_array_len: usize,
    pub sleep_times: Arc<SleepTimes>,
    pub radix_base: usize,
    pub quicksort_partition_type: QuickSortType,
    pub merge_sort_type: MergeSortType,
    pub shuffle_passes: u16,
}


impl From<&Yaml> for Config {
    fn from(conf: &Yaml) -> Self {
        Self {
            array_len: conf["array_length"].as_i64()
                .expect("Could not parse array_length from config file.") as usize,
            multi_array_len: conf["multi_array_length"].as_i64()
                .expect("Could not parse multi_array_length from config file.") as usize,
            sleep_times: Arc::new(SleepTimes::from(conf)),
            radix_base: conf["radix_lsd_base"].as_i64()
                .expect("Could not parse radix_lsd_base as an integer.") as usize,
            quicksort_partition_type: QuickSortType::from_str(
                conf["quicksort_partitioning"].as_str().expect("Could not parse quicksort_partitioning field in config as a string.")
            ).unwrap(),
            merge_sort_type: MergeSortType::from_str(
                conf["merge_sort_type"].as_str().expect("Could not parse merge_sort_type field in config as a string.")
            ).unwrap(),
            shuffle_passes: conf["shuffle_passes"].as_i64()
                .expect("Could not parse shuffle_passes field in config as an integer.") as u16,
        }
    }
}

#[derive(Debug)]
pub struct SleepTimes {
    pub bubble: Duration,
    pub cocktail: Duration,
    pub insertion: Duration,
    pub selection: Duration,
    pub shell: Duration,
    pub quick: Duration,
    pub merge: Duration,
    pub radix: Duration,

    pub shuffle: Duration,
}

impl From<&Yaml> for SleepTimes {
    fn from(conf: &Yaml) -> Self {
        #[inline]
        fn get_sleep_time_from_yaml(yaml: &Yaml, sleep_name: &'static str) -> Duration {
            let yaml_field = &yaml[sleep_name];
            Duration::from_millis(
                yaml_field.as_i64()
                .expect(
                    &format!("Could not parse {} as an integer: {:?}", sleep_name, yaml_field)
                ) as u64
            )
        }

        Self {
            bubble: get_sleep_time_from_yaml(conf, "bubble_sleep"),
            cocktail: get_sleep_time_from_yaml(conf, "cocktail_shaker_sleep"),
            insertion: get_sleep_time_from_yaml(conf, "insertion_sleep"),
            selection: get_sleep_time_from_yaml(conf, "selection_sleep"),
            shell: get_sleep_time_from_yaml(conf, "shell_sleep"),
            quick: get_sleep_time_from_yaml(conf, "quick_sleep"),
            merge: get_sleep_time_from_yaml(conf, "merge_sleep"),
            radix: get_sleep_time_from_yaml(conf, "radix_sleep"),
            shuffle: get_sleep_time_from_yaml(conf, "shuffle_sleep"),
        }
    }
}