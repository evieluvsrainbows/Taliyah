use crate::config::ConfigurationData;
use std::{fs::File, io::Read};

pub fn read_config(file: &str) -> ConfigurationData {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str::<ConfigurationData>(&contents).unwrap()
}
