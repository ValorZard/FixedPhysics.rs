use crate::type_defs::FP;
use crate::physics::structs::vector2::Vector2;

pub struct RectCollider {
    pub width : FP,
    pub height : FP,
    pub position : Vector2,
}

impl RectCollider {
    pub fn new(width : FP, height : FP, position : Vector2) -> Self {
        Self {
            width : width,
            height : height,
            position : position,
        }
    }

    pub fn get_top_left(&self) -> Vector2 {
        Vector2::new(self.position.x - self.width / 2, self.position.y + self.height / 2)
    }

    pub fn get_top_right(&self) -> Vector2 {
        Vector2::new(self.position.x + self.width / 2, self.position.y + self.height / 2)
    }

    pub fn get_bottom_left(&self) -> Vector2 {
        Vector2::new(self.position.x - self.width / 2, self.position.y - self.height / 2)
    }

    pub fn get_bottom_right(&self) -> Vector2 {
        Vector2::new(self.position.x + self.width / 2, self.position.y - self.height / 2)
    }
}