mod config;
mod physics;

use crate::config::load_config;
use crate::physics::do_physics;

fn main() {
    let config = load_config("config.yaml");
    do_physics(config);
}
