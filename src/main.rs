mod config;
use crate::config::load_config;
mod physics;
use crate::physics::load_physics;

fn main() {
    let config = load_config("config.yaml");
    load_physics(config);
}
