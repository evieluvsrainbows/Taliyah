pub mod locale;
pub mod time;
pub mod user_utils;

use crate::config::ConfigurationData;

pub fn read_config(file: &str) -> ConfigurationData {
    let contents = std::fs::read_to_string(file).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn format_int(int: u64) -> String {
    let mut result = String::new();
    for (idx, val) in int.to_string().chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            result.push(',');
        }
        result.push(val);
    }
    result.chars().rev().collect()
}

pub fn calculate_average_sum(ints: &[i64]) -> f64 {
    if ints.is_empty() {
        return 0.0;
    }
    ints.iter().sum::<i64>() as f64 / ints.len() as f64
}
