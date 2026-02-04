#[derive(Clone, Copy, Debug)]
pub struct Velocity2D {
    pub vel: Vec2,
}

impl Velocity2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vel: Vec2::new(x, y) }
    }
}
