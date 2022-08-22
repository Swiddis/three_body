use kiss3d::light::Light;
use kiss3d::nalgebra::geometry::Translation3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use crate::physics::{Universe, Body, Vector3};
use crate::sim_config::{Universe as SimUniverse, load_config};

pub mod physics;
pub mod sim_config;

fn create_universe(config: SimUniverse) -> Universe {
    physics::Universe {
        time: 0.0,
        grav_const: config.grav_const,
        bodies: config.bodies.iter().map(|b| Body {
            mass: b.mass,
            velocity: Vector3 {
                x: b.velocity.x,
                y: b.velocity.y,
                z: b.velocity.z
            },
            position: Vector3 {
                x: b.position.x,
                y: b.position.y,
                z: b.position.z
            }
        }).collect()
    }
}

fn main() {
    let config = load_config("config.yaml");
    let mut universe = create_universe(config.universe);

    let mut window = Window::new("Kiss3d: cube");
    let mut spheres: Vec<SceneNode> = Vec::new();

    for (i, body) in universe.bodies.iter().enumerate() {
        spheres.push(window.add_sphere(1.0));
        spheres[i].set_local_translation(Translation3::new(
            body.position.x as f32,
            body.position.y as f32,
            body.position.z as f32,
        ))
    }

    window.set_light(Light::StickToCamera);
    while window.render() {
        universe = universe.tick(config.constants.time_step);
        for (i, body) in universe.bodies.iter().enumerate() {
            spheres[i].set_local_translation(Translation3::new(
                body.position.x as f32,
                body.position.y as f32,
                body.position.z as f32,
            ))
        }
    }
}
