use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

use crate::config::Body;

use std::f64::consts::PI;

pub struct GraphicsBody {
    pub sphere: SceneNode
}

pub fn init(cfg_bodies: &Vec<Body>) -> (Window, Vec<GraphicsBody>) {
    let mut window = Window::new("Three Body");
    let mut bodies: Vec<GraphicsBody> = Vec::new();
    window.set_light(Light::StickToCamera);

    for body in cfg_bodies.iter() {
        let radius = (body.mass * 3.0 / (4.0 * PI)).cbrt();
        let s = window.add_sphere(radius as f32);
        bodies.push(GraphicsBody {
            sphere: s
        });
    }
    return (window, bodies);
}
