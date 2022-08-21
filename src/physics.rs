use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    pub fn to_string(&self) -> String {
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
pub struct Body {
    mass: f64,
    position: Vector3,
    velocity: Vector3,
}

impl Body {
    pub fn to_string(&self) -> String {
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

    fn force_vector(&self, body: &Body, grav: f64) -> Vector3 {
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
            let f_g = grav * self.mass * body.mass / (r * r);
            Vector3 {
                x: -f_g * d.x / d_sum,
                y: -f_g * d.y / d_sum,
                z: -f_g * d.z / d_sum,
            }
        }
    }

    fn accelerate(&self, force: &Vector3, step: f64) -> Body {
        return Body {
            mass: self.mass,
            position: self.position.clone(),
            velocity: Vector3 {
                x: self.velocity.x + force.x / self.mass * step,
                y: self.velocity.y + force.y / self.mass * step,
                z: self.velocity.z + force.z / self.mass * step,
            },
        };
    }
}

pub struct Universe {
    pub grav_const: f64,
    pub time: f64,
    pub bodies: Vec<Body>,
}

impl Universe {
    pub fn to_string(&self) -> String {
        let bodies: Vec<String> = self.bodies.iter().map(|x| x.to_string()).collect();
        format!("{:.3}: {{{}}}", self.time, bodies.join(" "))
    }

    pub fn tick(&self, step: f64) -> Universe {
        let forces = self.force_vectors();
        Universe {
            grav_const: self.grav_const,
            time: self.time + step,
            bodies: self
                .bodies
                .iter()
                .enumerate()
                .map(|(i, b)| b.accelerate(&forces[i], step).tick(step))
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
                forces[i] = forces[i].add(&a.force_vector(b, self.grav_const));
            }
        }
        return forces;
    }
}
