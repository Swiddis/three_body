use std::collections::HashMap;

use config::{Config, File, FileFormat, Value};
use serde::Deserialize;

use crate::physics::Universe;

pub mod physics;

#[derive(Deserialize)]
struct Constants {
    time_step: f64,
    duration: f64,
}

fn load_config(filename: &str) -> Config {
    Config::builder()
        .add_source(File::new(filename, FileFormat::Yaml))
        .build()
        .expect("Configuration is valid")
}

fn load_constants(config: HashMap<String, Value>) -> Constants {
    Constants {
        time_step: config["time_step"].clone().try_deserialize().unwrap(),
        duration: config["duration"].clone().try_deserialize().unwrap(),
    }
}

fn create_universe(config: HashMap<String, Value>) -> Universe {
    Universe {
        time: 0.0,
        grav_const: config["grav_const"].clone().try_deserialize().unwrap(),
        bodies: config["bodies"].clone().try_deserialize().unwrap(),
    }
}

fn main() {
    let config = load_config("config.yaml");
    let constants = load_constants(config.get_table("constants").unwrap());
    let mut universe = create_universe(config.get_table("universe").unwrap());
    let steps = (constants.duration / constants.time_step) as i32;
    println!("{}", universe.to_string());
    for _ in 0..steps {
        universe = universe.tick(constants.time_step);
        println!("{}", universe.to_string());
    }
}
