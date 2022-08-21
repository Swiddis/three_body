#[derive(Clone)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    fn to_string(&self) -> String {
        format!("<{}, {}, {}>", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
struct Body {
    mass: f32,
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
}

struct Universe {
    time: f32,
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
        Universe {
            time: self.time + 1.0,
            bodies: self.bodies.clone()
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
    for _ in 0..20 {
        universe = universe.tick();
        println!("{}", universe.to_string());
    }
}
