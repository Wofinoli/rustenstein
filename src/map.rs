use std::fs;

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Vec<u32>>,
    name: String,
    start: (u32, u32),
    end: (u32, u32),
}

impl Map {
    pub fn new(filename: String) -> Map {

        let level_name = "src/levels/test.txt";
        let tile_string = fs::read_to_string(level_name)
            .expect("Something went wrong reading the file.");

        let tiles = Map::generate_tiles(tile_string);

        Map {
            tiles,
            name: String::from("Test"),
            start: (0,0),
            end: (0,0),
        }
    }

    // TODO: Add proper error handling
    fn generate_tiles(tile_string: String) -> Vec<Vec<u32>> {
        let mut tiles: Vec<Vec<u32>> = vec![];

        for line in tile_string.lines() {
            let mut tile_line: Vec<u32> = vec![];
            for tile in line.chars() {
                match tile.to_digit(10) {
                    Some(d) => tile_line.push(d),
                    None => tile_line.push(0),
                };
            }

            tiles.push(tile_line);
                
            println!("{:?}", tiles);
        }
        vec![vec![0]]
    }
}
