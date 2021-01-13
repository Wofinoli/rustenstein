mod player;
mod map;
mod window;
pub mod util;

pub struct Game {
    map: map::WorldMap,
    player: player::Player,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            map: map::WorldMap::default(),
            player: player::Player::default(),
        }
    }
}

impl Game {

    pub fn draw(canvas: &mut sdl2::Canvas){
        unimplemented!()
    }
}
