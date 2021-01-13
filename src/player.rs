mod util;

use util::Vector2d;

pub struct Player {
    pos: util::Vector2d,
    dir: util::Vector2d,
    plane: util::Vector2d,
}

impl Default for Player {
    fn Default() -> Self {
        Self {
            pos: util::Vector2d::new(10,10),
            dir: util::Vector2d::new(-1, 0),
            plane: util::Vector2d::new(0, 0.66),
        }
    }
}
