mod config;
mod graphics;
mod physics;

use crate::config::load_config;
use crate::physics::do_simulation;

fn main() {
    let config = load_config("config.yaml");
    do_simulation(config);
}
