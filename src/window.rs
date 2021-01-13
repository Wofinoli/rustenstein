use sdl2::event::Event;

#[derive(Debug)]
pub struct KeyEvent {
    pub up: bool,
    pub up_event: Option<Event>,
    pub down_event: Option<Event>,
}

pub fn handle_keys(event: KeyEvent) {
    println!("{:#?}", event);
}
