pub mod locale;

use crate::config::ConfigurationData;
use std::{fs::File, io::Read};

pub fn read_config(file: &str) -> ConfigurationData {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str::<ConfigurationData>(&contents).unwrap()
}

pub fn format_int(int: u64) -> String {
    let mut string = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}

pub fn calculate_average_sum(ints: &[i64]) -> f64 {
    ints.iter().sum::<i64>() as f64 / ints.len() as f64
}
