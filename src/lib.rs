mod map;
mod window;

use map::Map;
use window::WinitState;

pub fn run() {

    let map = Map::new(String::from("test.txt"));
    match map {
        Ok(m) => println!("{:?}", m),
        Err(e) => println!("{}", e),
    }

    let window_state = WinitState::default();
    WinitState::run(window_state);
}
