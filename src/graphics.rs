use kiss3d::light::Light;
use kiss3d::nalgebra::Translation3;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;

use rapier3d_f64::prelude::RigidBodySet;

use crate::config::Body;

use std::f64::consts::PI;

pub struct GraphicsBody {
    pub sphere: SceneNode,
}

pub fn init(cfg_bodies: &Vec<Body>) -> (Window, Vec<GraphicsBody>) {
    let mut window = Window::new("Three Body");
    let mut bodies: Vec<GraphicsBody> = Vec::new();
    window.set_light(Light::StickToCamera);

    for body in cfg_bodies.iter() {
        let radius = (body.mass * 3.0 / (4.0 * PI)).cbrt();
        let color = (body.color.r as f32 / 255.0, body.color.g as f32 / 255.0, body.color.b as f32 / 255.0);
        let mut s = window.add_sphere(radius as f32);
        s.set_color(color.0, color.1, color.2);

        bodies.push(GraphicsBody { sphere: s });
    }
    return (window, bodies);
}

pub fn draw_bodies(rigid_body_set: &RigidBodySet, graphics: &mut Vec<GraphicsBody>) {
    for (i, body) in rigid_body_set.iter().enumerate() {
        let b_trans = body.1.position().translation.vector;
        let b_trans: Vec<f32> = b_trans.iter().map(|f| *f as f32).collect();
        let s_trans: Translation3<f32> = Translation3 {
            vector: kiss3d::nalgebra::Vector3::new(b_trans[0], b_trans[1], b_trans[2]),
        };
        graphics[i].sphere.set_local_translation(s_trans);
    }
}
