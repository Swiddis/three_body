mod config;
mod physics;

use crate::config::load_config;
use crate::physics::load_physics;

fn main() {
    let config = load_config("config.yaml");
    load_physics(config);
}
