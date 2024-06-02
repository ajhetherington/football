use serde::Deserialize;
use std::fs;



#[derive(Deserialize, Default)]
pub struct Config {
    pub ai_addr: String,
    pub env_addr: String,
    pub seed: Option<u64>,
}

pub fn read_config(config_path: &str) -> Config {
    let yaml_string = fs::read_to_string(config_path).unwrap();
    return serde_yaml::from_str::<Config>(&yaml_string).unwrap()
}