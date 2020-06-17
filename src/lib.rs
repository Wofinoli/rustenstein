mod map;

use map::Map;

pub fn run() {

    let level = Map::new(String::from("Level 1"));
    println!("{:?}",level);
}
