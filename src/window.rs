use sdl2::event::Event;

pub struct KeyEvent {
    pub up: bool,
    pub up_event: Option<Event>,
    pub down_event: Option<Event>,
}

pub fn handle_keys(_event: KeyEvent) {
    println!("Here!");
}
