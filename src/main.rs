#[derive(Clone)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {
    fn to_string(&self) -> String {
        format!("<{}, {}, {}>", self.x, self.y, self.z)
    }
}

struct Body {
    mass: f64,
    position: Vector3,
    velocity: Vector3
}

impl Body {
    fn to_string(&self) -> String {
        let position = self.position.to_string();
        let momentum = Vector3 {
            x: self.velocity.x * self.mass,
            y: self.velocity.y * self.mass,
            z: self.velocity.z * self.mass
        }.to_string();
        format!("[{} {}]", position, momentum)
    }

    fn tick(&self, step: f64) -> Body {
        Body {
            mass: self.mass,
            position: Vector3 {
                x: self.position.x + self.velocity.x * step,
                y: self.position.y + self.velocity.y * step,
                z: self.position.z + self.velocity.z * step
            },
            velocity: self.velocity.clone()
        }
    }
}

struct Universe {
    time: f64,
    bodies: Vec<Body>
}

impl Universe {
    fn to_string(&self) -> String {
        let bodies: Vec<String> = self.bodies
                .iter()
                .map(|x| x.to_string())
                .collect();
        format!("{}: {{{}}}", self.time, bodies.join(" "))
    }

    fn tick(&self) -> Universe {
        const STEP: f64 = 0.5;
        Universe {
            time: self.time + STEP,
            bodies: self.bodies.iter().map(|b| b.tick(STEP)).collect()
        }
    }
}

fn create_universe() -> Universe {
    Universe {
        time: 0.0,
        bodies: vec![
            Body {
                mass: 1.0,
                position: Vector3 { x: 1.0, y: 0.0, z: 0.0 },
                velocity: Vector3 { x: 0.0, y: 1.0, z: 0.0 }
            },
            Body {
                mass: 1.0,
                position: Vector3 { x: -1.0, y: 0.0, z: 0.0 },
                velocity: Vector3 { x: 0.0, y: -1.0, z: 0.0 }
            }
        ]
    }
}

fn main() {
    let mut universe = create_universe();
    println!("{}", universe.to_string());
    for _ in 0..2 {
        universe = universe.tick();
        println!("{}", universe.to_string());
    }
}
