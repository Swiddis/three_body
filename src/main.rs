use std::collections::HashMap;

use config::{Config, File, FileFormat, Value};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn to_string(&self) -> String {
        format!("<{:.3} {:.3} {:.3}>", self.x, self.y, self.z)
    }

    fn add(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }

    fn sub(&self, v: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }

    fn norm(&self) -> f64 {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
}

#[derive(Deserialize)]
struct Body {
    mass: f64,
    position: Vector3,
    velocity: Vector3,
}

impl Body {
    fn to_string(&self) -> String {
        let position = self.position.to_string();
        let momentum = Vector3 {
            x: self.velocity.x * self.mass,
            y: self.velocity.y * self.mass,
            z: self.velocity.z * self.mass,
        }
        .to_string();
        format!("[{} {}]", position, momentum)
    }

    fn tick(&self, step: f64) -> Body {
        Body {
            mass: self.mass,
            position: Vector3 {
                x: self.position.x + self.velocity.x * step,
                y: self.position.y + self.velocity.y * step,
                z: self.position.z + self.velocity.z * step,
            },
            velocity: self.velocity.clone(),
        }
    }

    fn force_vector(&self, body: &Body) -> Vector3 {
        const G: f64 = 1.0;
        let d = self.position.sub(&body.position);
        let r = d.norm();
        if r == 0.0 {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        } else {
            let d_sum = d.x.abs() + d.y.abs() + d.z.abs();
            let f_g = G * self.mass * body.mass / (r * r);
            Vector3 {
                x: f_g * d.x / d_sum,
                y: f_g * d.y / d_sum,
                z: f_g * d.z / d_sum,
            }
        }
    }

    fn accelerate(&self, force: &Vector3, step: f64) -> Body {
        return Body {
            mass: self.mass,
            position: self.position.clone(),
            velocity: Vector3 {
                x: self.velocity.x - force.x / self.mass * step,
                y: self.velocity.y - force.y / self.mass * step,
                z: self.velocity.z - force.z / self.mass * step,
            },
        };
    }
}

struct Universe {
    time: f64,
    bodies: Vec<Body>,
}

impl Universe {
    fn to_string(&self) -> String {
        let bodies: Vec<String> = self.bodies.iter().map(|x| x.to_string()).collect();
        format!("{:.3}: {{{}}}", self.time, bodies.join(" "))
    }

    fn tick(&self) -> Universe {
        const STEP: f64 = 0.1;
        let forces = self.force_vectors();
        Universe {
            time: self.time + STEP,
            bodies: self
                .bodies
                .iter()
                .enumerate()
                .map(|(i, b)| b.accelerate(&forces[i], STEP).tick(STEP))
                .collect(),
        }
    }

    fn force_vectors(&self) -> Vec<Vector3> {
        let mut forces: Vec<Vector3> = self
            .bodies
            .iter()
            .map(|_| Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            })
            .collect();
        for (i, a) in self.bodies.iter().enumerate() {
            for b in self.bodies.iter() {
                forces[i] = forces[i].add(&a.force_vector(b));
            }
        }
        return forces;
    }
}

fn load_config(filename: &str) -> Config {
    Config::builder()
        .add_source(File::new(filename, FileFormat::Yaml))
        .build()
        .expect("Configuration is valid")
}

fn create_universe(config: HashMap<String, Value>) -> Universe {
    Universe {
        time: 0.0,
        bodies: config["bodies"].clone().try_deserialize().unwrap(),
    }
}

fn main() {
    let config = load_config("config.yaml");
    let mut universe = create_universe(config.get_table("universe").unwrap());
    let constants = config.get_table("constants").unwrap();
    let steps = (constants["duration"].clone().into_float().unwrap()
        / constants["time_step"].clone().into_float().unwrap()) as i32;
    println!("{}", universe.to_string());
    for _ in 0..steps {
        universe = universe.tick();
        println!("{}", universe.to_string());
    }
}
