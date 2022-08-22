use rapier3d_f64::prelude::*;
use std::f64::consts::PI;

use crate::config::{ThreeBodyConfig, Body};

fn add_bodies(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet, bodies: &Vec<Body>) {
    for body in bodies {
        let radius = (body.mass * 3.0 / (4.0 * PI)).cbrt();
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![body.position.x, body.position.y, body.position.z])
            .build();
        let collider = ColliderBuilder::ball(radius).restitution(1.0).build();
        let handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, handle, rigid_body_set);
    }
}

pub fn load_physics(config: ThreeBodyConfig) {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    add_bodies(&mut rigid_body_set, &mut collider_set, &config.universe.bodies);
}
