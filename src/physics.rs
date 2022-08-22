use rapier3d_f64::prelude::*;
use std::f64::consts::PI;

use crate::config::{ThreeBodyConfig, Body};

struct PhysicsBody {
    handle: RigidBodyHandle,
    mass: f64,
    translation: Vector<Real>
}

fn add_bodies(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, bodies: &Vec<Body>) {
    for body in bodies {
        let radius = (body.mass * 3.0 / (4.0 * PI)).cbrt();
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![body.position.x, body.position.y, body.position.z])
            .linvel(vector![body.velocity.x, body.velocity.y, body.velocity.z])
            .ccd_enabled(true)
            .build();
        let collider = ColliderBuilder::ball(radius).restitution(1.0).density(1.0).build();
        let handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, handle, rigid_body_set);
    }
}

fn get_bodies(rigid_body_set: &RigidBodySet) -> Vec<PhysicsBody> {
    rigid_body_set.iter().map(|body| PhysicsBody {
        handle: body.0,
        mass: body.1.mass(),
        translation: body.1.translation().clone()
    }).collect()
}

fn calculate_forces(rigid_body_set: &mut RigidBodySet) {
    let bodies = get_bodies(rigid_body_set);
    for body in rigid_body_set.iter_mut() {
        body.1.reset_forces(true);
        for p_body in bodies.iter() {
            if p_body.handle == body.0 {
                continue;
            }
            let dsp = body.1.translation() - p_body.translation;
            let f_grav = p_body.mass * body.1.mass() * dsp / dsp.norm().powf(3.0);
            body.1.add_force(f_grav, true);
        }
    }
}

pub fn do_physics(config: ThreeBodyConfig) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    add_bodies(&mut rigid_body_set, &mut collider_set, &config.universe.bodies);

    let gravity = vector![0.0, 0.0, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();
    for _ in 0..200 {
        calculate_forces(&mut rigid_body_set);
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );
        println!("=================");
        for body in rigid_body_set.iter() {
            println!("{:?}", body.1.translation());
        }
    }
}
