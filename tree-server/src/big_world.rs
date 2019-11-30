use crate::config::Config;
use std::fs::File;
use toml;
use std::io::Read;

fn prepare_config(path: &str) -> Config {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Err(_e) => {
            println!("Error while reading config file {}", path);
            Config::default()
        }
        _ => toml::from_str(contents.as_str()).unwrap()
    }
}

pub fn start(conf: Option<&str>) {
    let config = match conf {
        Some(path) => prepare_config(path),
        _ => Config::default()
    };
    println!("big world server run with config: {:?}", config)
}