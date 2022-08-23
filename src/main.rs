use ::config::ConfigError;

mod config;
mod graphics;
mod physics;

use crate::config::load_config;
use crate::physics::do_simulation;

fn main() -> Result<(), ConfigError> {
    let config = load_config("config.yaml")?;
    do_simulation(config);
    Ok(())
}
