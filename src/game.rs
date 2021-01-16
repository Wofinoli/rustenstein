mod player;
mod map;
pub mod util;

use sdl2::{
    render::Canvas,
    video,
    pixels::Color,
    rect::Point,
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
        
        for line in lines {
            Game::ver_line(canvas, line);
        }
    }

    fn calculate_lines(&self, canvas: &Canvas<video::Window>) -> Vec<util::Line> {
        let (height, width) = canvas.window().size();
        let (height, width) = (height as i32, width as i32);

        let mut to_draw = Vec::<util::Line>::new();
        for x in 0..width {
            let camera_x = (2 * x) as f64 / width as f64 - 1.0;
            let ray_dir = util::Vector2d::new(self.player.dir.x + self.player.plane.x * camera_x,
                                              self.player.dir.y + self.player.plane.y * camera_x);

            let mut map_pos = util::Vector2d::new(self.player.pos.x.trunc(), self.player.pos.y.trunc());
            let delta_dist = util::Vector2d::new( (1.0 /ray_dir.x).abs(), (1.0 /ray_dir.y).abs() );

            let mut step = util::Vector2d::new(0.0,0.0);
            let mut side_dist = util::Vector2d::new(0.0, 0.0);

            if ray_dir.x < 0.0 {
                step.x = -1.0;
                side_dist.x = (self.player.pos.x - map_pos.x) * delta_dist.x;
            } else {
                step.x = 1.0;
                side_dist.x = (1.0 - self.player.pos.x + map_pos.x) * delta_dist.x;
            }

            if ray_dir.y < 0.0 {
                step.y = -1.0;
                side_dist.y = (self.player.pos.y - map_pos.y) * delta_dist.y;
            } else {
                step.y = 1.0;
                side_dist.y = (1.0 - self.player.pos.y + map_pos.y) * delta_dist.y;
            }

            let mut hit_state;
            'dda: loop {
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    hit_state = util::Hit::XSide;
                } else {
                    side_dist.y += delta_dist.x;
                    map_pos.y += step.y;
                    hit_state = util::Hit::YSide;
                }

                if self.map[map_pos.x as usize][map_pos.y as usize] > 0 {
                    break 'dda
                }
            }

            let perp_wall_dist = {
                match hit_state {
                    util::Hit::XSide => (map_pos.x - self.player.pos.x + (1.0 - step.x) / 2.0) / ray_dir.x,
                    util::Hit::YSide => (map_pos.y - self.player.pos.y + (1.0 - step.y) / 2.0) / ray_dir.y,
                    util::Hit::Miss => panic!("Ray didn't hit a wall!"),
                }
            };

            let line_height = (height as f64 / perp_wall_dist) as i32;

            let start = Point::new(x as i32, std::cmp::max(0, -line_height / 2 + height as i32 / 2));
            let end = Point::new(x as i32, std::cmp::min(height as i32 - 1, line_height / 2 + height as i32 / 2));
            
            let mut color = match self.map[map_pos.x as usize][map_pos.y as usize] {
                1 => Color::RED,
                2 => Color::GREEN,
                3 => Color::BLUE,
                4 => Color::WHITE,
                _ => Color::YELLOW,
            };

            if hit_state == util::Hit::YSide {
                color.r /= 2;
                color.g /= 2;
                color.b /= 2;
            }
            
            to_draw.push(util::Line {
                start,
                end,
                color,
            });
        }
        to_draw
    }

    pub fn ver_line(canvas: &mut Canvas<video::Window>, line: util::Line) {
        canvas.set_draw_color(line.color);
        match canvas.draw_line(line.start, line.end) {
            Err(e) => println!("Something went wrong: {:?}", e),
            _ => (),
        };
    }

}
