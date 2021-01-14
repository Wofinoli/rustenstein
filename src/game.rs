mod player;
mod map;
mod window;
pub mod util;

use sdl2::{
    render::Canvas,
    video,
};

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

    pub fn draw(&mut self, canvas: &mut Canvas<video::Window>){
        let (height, width) = canvas.window().size();

        for x in 0..width {
            let camera_x = (2 * x) as f64 / width as f64 - 1.0;
            let ray_dir = util::Vector2d::new(self.player.dir.x + self.player.plane.x * camera_x,
                                              self.player.dir.y + self.player.plane.y * camera_x);

            let map_pos = util::Vector2d::new(self.player.pos.x.trunc(), self.player.pos.y.trunc());
        }
        unimplemented!()
    }
}
