mod map;

use map::Map;

pub fn run() {

    let level = Map::new(String::from("test.txt"));
    println!("{:?}",level);
}
