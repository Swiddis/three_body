use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Constants {
    pub time_step: f64,
}

#[derive(Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize)]
pub struct Body {
    pub mass: f64,
    pub position: Vector3,
    pub velocity: Vector3,
}

#[derive(Deserialize)]
pub struct Universe {
    pub grav_const: f64,
    pub bodies: Vec<Body>,
}

#[derive(Deserialize)]
pub struct ThreeBodyConfig {
    pub constants: Constants,
    pub universe: Universe,
}

pub fn load_config(filename: &str) -> ThreeBodyConfig {
    let config = Config::builder()
        .add_source(File::new(filename, FileFormat::Yaml))
        .build()
        .expect("Yaml is valid");
    config
        .try_deserialize::<ThreeBodyConfig>()
        .expect("Configuration is valid")
}
