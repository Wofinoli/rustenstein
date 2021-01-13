use sdl2::{
    event::Event,
    rect::Point,
    render::Canvas,
    video::Window,
    pixels::Color,
};


#[derive(Debug)]
pub struct KeyEvent {
    pub up: bool,
    pub up_event: Option<Event>,
    pub down_event: Option<Event>,
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
}

pub fn handle_keys(event: KeyEvent) {
    println!("{:#?}", event);
}

pub fn ver_line(canvas: &mut Canvas<Window>, line: &Line) {
    canvas.set_draw_color(line.color);
    match canvas.draw_line(line.start, line.end) {
        Err(e) => println!("Something went wrong: {:?}", e),
        _ => (),
    };
}
