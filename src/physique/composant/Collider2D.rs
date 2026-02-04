use crate::math::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub enum Collider2D {
    AABB { half_size: Vec2 },

    Circle { radius: f32 },
}

impl Collider2D {
    // 1/2 pour calcul 
    pub fn new_aabb(half_width: f32, half_height: f32) -> Self {
        Collider2D::AABB {
            half_size: Vec2::new(half_width, half_height),
        }
    }

    pub fn new_circle(radius: f32) -> Self {
        Collider2D::Circle { radius }
    }
}
