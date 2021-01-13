use crate::game::util;

pub struct Player {
    pos: util::Vector2d,
    dir: util::Vector2d,
    plane: util::Vector2d,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: util::Vector2d::new(10.0,10.0),
            dir: util::Vector2d::new(-1.0, 0.0),
            plane: util::Vector2d::new(0.0, 0.66),
        }
    }
}
