use crate::game::util;

pub struct Player {
    pub pos: util::Vector2d,
    pub dir: util::Vector2d,
    pub plane: util::Vector2d,
}

impl Player {
    //pub fn update(&mut self) {
    //}
}

impl Default for Player {
    fn default() -> Self {
        Self {
            pos: util::Vector2d::new(15.0,8.0),
            dir: util::Vector2d::new(0.0, 1.0),
            plane: util::Vector2d::new(0.66, 0.0),
        }
    }
}
