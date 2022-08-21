use std::collections::HashMap;

use config::{Config, File, FileFormat, Value};
use kiss3d::light::Light;
use kiss3d::nalgebra::geometry::Translation3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use serde::Deserialize;

use crate::physics::Universe;

pub mod physics;

#[derive(Deserialize)]
struct Constants {
    time_step: f64,
}

fn load_config(filename: &str) -> Config {
    Config::builder()
        .add_source(File::new(filename, FileFormat::Yaml))
        .build()
        .expect("Configuration is valid")
}

fn load_constants(config: HashMap<String, Value>) -> Constants {
    Constants {
        time_step: config["time_step"].clone().try_deserialize().unwrap()
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

    let mut window = Window::new("Kiss3d: cube");
    let mut spheres: Vec<SceneNode> = Vec::new();

    for (i, body) in universe.bodies.iter().enumerate() {
        spheres.push(window.add_sphere(0.1));
        spheres[i].set_local_translation(Translation3::new(
            body.position.x as f32,
            body.position.y as f32,
            body.position.z as f32,
        ))
    }

    window.set_light(Light::StickToCamera);
    while window.render() {
        universe = universe.tick(constants.time_step);
        for (i, body) in universe.bodies.iter().enumerate() {
            spheres[i].set_local_translation(Translation3::new(
                body.position.x as f32,
                body.position.y as f32,
                body.position.z as f32,
            ))
        }
    }
}
