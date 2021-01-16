mod player;
mod map;
pub mod util;

use sdl2::{
    render::Canvas,
    keyboard::Scancode,
    video,
    pixels::Color,
    rect::Point,
};
use util::{
    Hit,
    Line,
    Vector2d,
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

    fn calculate_lines(&self, canvas: &Canvas<video::Window>) -> Vec<Line> {
        let (height, width) = canvas.window().size();
        let (height, width) = (height as i32, width as i32);

        let mut to_draw = Vec::<Line>::new();
        for x in 0..width {
            let camera_x = (2 * x) as f64 / (width as f64) - 1.0;
            let ray_dir = Vector2d::new(self.player.dir.x + self.player.plane.x * camera_x,
                                        self.player.dir.y + self.player.plane.y * camera_x);

            let mut map_pos = Vector2d::new(self.player.pos.x.trunc(), self.player.pos.y.trunc());
            let delta_dist = Vector2d::new( (1.0 /ray_dir.x).abs(), (1.0 /ray_dir.y).abs() );

            let mut step = Vector2d::new(0.0,0.0);
            let mut side_dist = Vector2d::new(0.0, 0.0);

            println!("{:?}", delta_dist);

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
                    hit_state = Hit::XSide;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;
                    hit_state = Hit::YSide;
                }

                if self.map[map_pos.x as usize][map_pos.y as usize] > 0 {
                    break 'dda
                }
            }

            let perp_wall_dist = {
                match hit_state {
                    Hit::XSide => (map_pos.x - self.player.pos.x + (1.0 - step.x) / 2.0) / ray_dir.x,
                    Hit::YSide => (map_pos.y - self.player.pos.y + (1.0 - step.y) / 2.0) / ray_dir.y,
                    Hit::Miss => panic!("Ray didn't hit a wall!"),
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

            if hit_state == Hit::YSide {
                color.r /= 2;
                color.g /= 2;
                color.b /= 2;
            }
            
            to_draw.push(Line {
                start,
                end,
                color,
            });
        }
        to_draw
    }

    pub fn ver_line(canvas: &mut Canvas<video::Window>, line: Line) {
        canvas.set_draw_color(line.color);
        match canvas.draw_line(line.start, line.end) {
            Err(e) => println!("Something went wrong: {:?}", e),
            _ => (),
        };
    }

    pub fn handle_keys(&mut self, key: Scancode, modifier: f64) {
        let speed = modifier * 5.0;
        let rot_rate = modifier * 3.0;
        

        let (prev_x, prev_y) = (self.player.pos.x, self.player.pos.y);
        let (prev_dir_x, _prev_dir_y) = (self.player.dir.x, self.player.dir.y);
        let (delta_x, delta_y) = (self.player.dir.x * speed, self.player.dir.y * speed);
        let size = self.map.size;

        let (pos_x, pos_y) = (&mut self.player.pos.x, &mut self.player.pos.y);
        let (dir_x, dir_y) = (&mut self.player.dir.x, &mut self.player.dir.y);
        let (plane_x, plane_y) = (&mut self.player.plane.x, &mut self.player.plane.y);

        match key {
            Scancode::W => {
                let new_x = (*pos_x + delta_x) as usize;
                let new_y = (*pos_y + delta_y) as usize;

                if new_x < size && self.map[new_x][*pos_y as usize] == 0 {
                    *pos_x += delta_x;
                }
                if new_y < size && self.map[*pos_x as usize][(*pos_y + delta_y) as usize] == 0 {
                    *pos_y += delta_y;
                }
                println!("Speed {:?}, Old: {:?}, New: {:?}", speed, (prev_x, prev_y), (*pos_x, *pos_y));
            },
            Scancode::S => {
                let new_x = (*pos_x - delta_x) as usize;
                let new_y = (*pos_y - delta_y) as usize;


                if new_x < size && self.map[new_x][*pos_y as usize] == 0 {
                        *pos_x -= delta_x;
                }
                if new_y < size && self.map[*pos_x as usize][new_y] == 0 {
                    *pos_y -= delta_y;
                }
                println!("Speed {:?}, Old: {:?}, New: {:?}", speed, (prev_x, prev_y), (*pos_x, *pos_y));
            },
            Scancode::A => {
                *dir_x = *dir_x * (rot_rate).cos() - *dir_y * (rot_rate).sin();
                *dir_y = prev_dir_x * (rot_rate).sin() + *dir_y * (rot_rate).cos();
                let prev_plane_x = *plane_x;
                *plane_x = *plane_x * (rot_rate).cos() - *plane_y * (rot_rate).sin();
                *plane_y = prev_plane_x * (rot_rate).sin() + *plane_y * (rot_rate).cos();
            },
            Scancode::D => {
                *dir_x = *dir_x * (-rot_rate).cos() - *dir_y * (-rot_rate).sin();
                *dir_y = prev_dir_x * (-rot_rate).sin() + *dir_y * (-rot_rate).cos();
                let prev_plane_x = *plane_x;
                *plane_x = *plane_x * (-rot_rate).cos() - *plane_y * (-rot_rate).sin();
                *plane_y = prev_plane_x * (-rot_rate).sin() + *plane_y * (-rot_rate).cos();
            },
            _ => (),
        }

    }

}
