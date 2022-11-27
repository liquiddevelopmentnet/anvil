pub mod main;
pub mod projects;

use std::io::{Read, Write};
use serde::Serialize;
use crate::*;

pub fn load_all() {
    let mut loads: Vec<bool> = vec![];

    loads.push(main::load());
    loads.push(projects::load());

    if loads.contains(&true) {
        cstm!("📁", "detected fresh configs, please configure the config files and restart the server");
        std::process::exit(0);
    }

    cstm!("📁", "loaded configurations");
}

pub fn save_config<T>(config_object: &T, path: &str) -> Result<(), io::Error>
where
    T: Serialize,
    T: YamlConfig
{
    let yaml = serde_yaml::to_string(&config_object).unwrap();

    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(yaml.as_bytes()).unwrap();

    Ok(())
}

pub fn load_config<T>(path: &str) -> Option<T>
where
    T: for<'de> serde::Deserialize<'de>,
    T: YamlConfig
{
    if !std::path::Path::new(path).exists() {
        let config = T::new();
        config.save().unwrap();
        cstm!("📁", "created configuration file: {}", path);
        return None;
    }

    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    serde_yaml::from_str(&contents).unwrap()
}

pub trait YamlConfig {
    const DEFAULT_CONFIG_NAME: &'static str;
    fn new() -> Self;
    fn save(&self) -> Result<(), io::Error>;
}