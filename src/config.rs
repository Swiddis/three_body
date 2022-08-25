use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize)]
pub struct ColorRGB {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

#[derive(Deserialize)]
pub struct Body {
    pub mass: f64,
    pub position: Vector3,
    pub velocity: Vector3,
    pub color: ColorRGB,
}

#[derive(Deserialize)]
pub struct Universe {
    pub grav_const: f64,
    pub bodies: Vec<Body>,
}

#[derive(Deserialize)]
pub struct ThreeBodyConfig {
    pub universe: Universe,
}

pub fn load_config(filename: &str) -> Result<ThreeBodyConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new(filename, FileFormat::Yaml))
        .build()?;
    return config
        .try_deserialize::<ThreeBodyConfig>();
}
