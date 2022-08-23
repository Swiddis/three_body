use std::f64::consts::PI;

use kiss3d::window::Window;
use rapier3d_f64::prelude::*;
use ::config::ConfigError;

use crate::config::{Body, ThreeBodyConfig};
use crate::graphics::{draw_bodies, init, GraphicsBody};

struct PhysicsBody {
    handle: RigidBodyHandle,
    mass: f64,
    translation: Vector<Real>,
}

struct Physics {
    gravity: Vector<f64>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    bodies: RigidBodySet,
    colliders: ColliderSet,
}

impl Physics {
    fn new(rigid_body_set: RigidBodySet, collider_set: ColliderSet) -> Self {
        Self {
            gravity: vector![0.0, 0.0, 0.0],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            bodies: rigid_body_set,
            colliders: collider_set,
        }
    }

    fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

fn add_bodies(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    bodies: &Vec<Body>,
) -> Result<(), ConfigError> {
    for body in bodies {
        if body.mass <= 0.0 {
            return Err(ConfigError::Message("body mass must be positive".to_owned()));
        }
        let radius = (body.mass * 3.0 / (4.0 * PI)).cbrt();
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![body.position.x, body.position.y, body.position.z])
            .linvel(vector![body.velocity.x, body.velocity.y, body.velocity.z])
            .ccd_enabled(true)
            .build();
        let collider = ColliderBuilder::ball(radius)
            .restitution(1.0)
            .density(1.0)
            .friction(0.0)
            .build();
        let handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, handle, rigid_body_set);
    }
    Ok(())
}

fn get_bodies(rigid_body_set: &RigidBodySet) -> Vec<PhysicsBody> {
    rigid_body_set
        .iter()
        .map(|body| PhysicsBody {
            handle: body.0,
            mass: body.1.mass(),
            translation: body.1.translation().clone(),
        })
        .collect()
}

fn calculate_forces(rigid_body_set: &mut RigidBodySet, grav_const: &f64) {
    let bodies = get_bodies(rigid_body_set);
    for body in rigid_body_set.iter_mut() {
        body.1.reset_forces(true);
        for p_body in bodies.iter() {
            if p_body.handle == body.0 {
                continue;
            }
            let dsp = body.1.translation() - p_body.translation;
            let f_grav = -grav_const * p_body.mass * body.1.mass() * dsp / dsp.norm().powf(3.0);
            body.1.add_force(f_grav, true);
        }
    }
}

fn setup_simulation(config: &ThreeBodyConfig) -> Result<(Physics, Window, Vec<GraphicsBody>), ConfigError> {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    add_bodies(
        &mut rigid_body_set,
        &mut collider_set,
        &config.universe.bodies,
    )?;
    let physics = Physics::new(rigid_body_set, collider_set);
    let (window, graphics) = init(&config.universe.bodies);
    return Ok((physics, window, graphics));
}

pub fn do_simulation(config: ThreeBodyConfig) -> Result<(), ConfigError> {
    let (mut physics, mut window, mut graphics) = setup_simulation(&config)?;
    while window.render() {
        calculate_forces(&mut physics.bodies, &config.universe.grav_const);
        physics.step();
        draw_bodies(&physics.bodies, &mut graphics);
    }
    Ok(())
}
