mod map;

use map::Map;

pub fn run() {
    let tiles = vec![ vec![1, 2, 3, 4, 5], 
                      vec![6, 7, 8, 9, 10]];


    let level = Map::new(tiles, String::from("Level 1"));
    println!("{:?}",level);
}
