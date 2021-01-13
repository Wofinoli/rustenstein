use crate::game::util;
use sdl2::{
    render::Canvas,
    video,
};

pub struct Window;

impl Window {

    pub fn calculate_lines() -> Vec<util::Line>  {
        unimplemented!()
    }

    pub fn ver_line(canvas: &mut Canvas<video::Window>, line: &util::Line) {
        canvas.set_draw_color(line.color);
        match canvas.draw_line(line.start, line.end) {
            Err(e) => println!("Something went wrong: {:?}", e),
            _ => (),
        };
    }
}

