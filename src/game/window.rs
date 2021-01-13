mod util;
mod map;

use util::Line;
use map::WorldMap;

pub struct Window;

impl Window {

    pub fn calculate_lines() {

    }

    pub fn ver_line(canvas: &mut Canvas<Window>, line: &Line) {
        canvas.set_draw_color(line.color);
        match canvas.draw_line(line.start, line.end) {
            Err(e) => println!("Something went wrong: {:?}", e),
            _ => (),
        };
    }
}

