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
        let lines = self.calculate_lines(&canvas);
        
        unimplemented!()
    }

    fn calculate_lines(&self, canvas: &Canvas<video::Window>) -> Vec<util::Line> {
        let (height, width) = canvas.window().size();

        let to_draw = Vec::<util::Line>::new();
        for x in 0..width {
            let camera_x = (2 * x) as f64 / width as f64 - 1.0;
            let ray_dir = util::Vector2d::new(self.player.dir.x + self.player.plane.x * camera_x,
                                              self.player.dir.y + self.player.plane.y * camera_x);

            let mut map_pos = util::Vector2d::new(self.player.pos.x.trunc(), self.player.pos.y.trunc());
            let delta_dist = util::Vector2d::new( (1.0 /ray_dir.x).abs(), (1.0 /ray_dir.y).abs() );

            let mut stepX = 0;
            let mut stepY = 0;
            let mut side_dist = util::Vector2d::new(0.0, 0.0);

            if ray_dir.x < 0.0 {
                stepX = -1;
                side_dist.x = (self.player.pos.x - map_pos.x) * delta_dist.x;
            } else {
                stepX = 1;
                side_dist.x = (1.0 + self.player.pos.x - map_pos.x) * delta_dist.x;
            }

            if ray_dir.y < 0.0 {
                stepY = -1;
                side_dist.y = (self.player.pos.y - map_pos.y) * delta_dist.y;
            } else {
                stepY = 1;
                side_dist.y = (1.0 + self.player.pos.y - map_pos.y) * delta_dist.y;
            }

            let mut hit_state = util::Hit::Miss;
            'dda: loop {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += stepX as f64;
                    hit_state = util::Hit::XSide;
                } else {
                    side_dist.y += delta_dist.x;
                    map_pos.y += stepY as f64;
                    hit_state = util::Hit::YSide;
                }

                if self.map[map_pos.x as usize][map_pos.y as usize] > 0 {
                    break 'dda
                }
            }
            
        }
        to_draw
    }
}
