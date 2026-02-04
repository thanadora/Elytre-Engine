use crate::math::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Transform2D {
    pub position: Vec2,
}

impl Transform2D {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }

    pub fn zero() -> Self {
        Self { position: Vec2::new(0.0, 0.0) }
    }
}
