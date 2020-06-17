use std::fs;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Vec<u32>>,
    name: String,
    start: (u32, u32),
    end: (u32, u32),
}

impl Map {
    pub fn new(filename: String) -> Map {

        let level_name = format!("src/levels/{}", filename);
        let tile_string = fs::read_to_string(level_name)
            .expect("Something went wrong reading the file.");

        let (start, end, tiles) = Map::generate_tiles(tile_string);

        Map {
            tiles,
            name: filename[..filename.len()-4].to_string(),
            start,
            end,
        }
    }

    // TODO: Add proper error handling
    fn generate_tiles(tile_string: String) -> 
        ( (u32, u32), (u32, u32), Vec<Vec<u32>>) {
        let mut tiles: Vec<Vec<u32>> = vec![];
        let mut start: (u32, u32) = (0, 0);
        let mut end: (u32, u32) = (0, 0);

        for (row, line) in tile_string.lines().enumerate() {
            let mut tile_line: Vec<u32> = vec![];
            for (col, tile) in line.chars().enumerate() {
                match tile.to_digit(10) {
                    Some(d) => {
                        tile_line.push(d);
                        if d == 2 {
                            start = (row as u32, col as u32);
                        } else if d == 3 {
                            end = (row as u32, col as u32);
                        }
                    }
                    None => tile_line.push(0),
                };
            }

            tiles.push(tile_line);
                
        }
        (start,end,tiles)
    }
}
