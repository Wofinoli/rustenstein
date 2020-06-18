mod map;

use map::Map;

pub fn run() {

    let map = Map::new(String::from("test.txt"));
    match map {
        Ok(m) => println!("{:?}", m),
        Err(e) => println!("{}", e),
    }
}
