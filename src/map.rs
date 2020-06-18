use std::fs;
use std::error::Error;

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position { x, y }
    }
}

#[derive(Debug)]
pub struct Map {
    tiles: Vec<Vec<u32>>,
    name: String,
    start: Position,
    goal: Position,
}

impl Map {
    pub fn new(filename: String) -> Result<Map, Box<dyn Error>> {

        let level_name = format!("src/levels/{}", filename);
        let tile_string = fs::read_to_string(level_name)
            .expect("Something went wrong reading the file.");

        let map_data = Map::generate_tiles(tile_string)?;

        Ok(Map {
            tiles: map_data.0,
            name: filename[..filename.len()-4].to_string(),
            start: map_data.1,
            goal: map_data.2,
        })
    }

    fn generate_tiles(tile_string: String) -> 
        Result<(Vec<Vec<u32>>, Position, Position), &'static str> {

        let mut start: Option<Position> = None;
        let mut goal: Option<Position> = None;

        let mut tiles: Vec<Vec<u32>> = vec![];
        for (row, line) in tile_string.lines().enumerate() {
            let mut tile_line: Vec<u32> = vec![];
            for (col, tile) in line.chars().enumerate() {
                match tile.to_digit(10) {
                    Some(2) => {
                        tile_line.push(2);
                        match start {
                            None => start = Some(Position::new(row as f64, col as f64)),
                            _ => return Err("Map loading failed, more than one start tile."),
                        }
                    },
                    Some(3) => {
                        tile_line.push(3);
                        match goal {
                            None => goal = Some(Position::new(row as f64, col as f64)),
                            _ => return Err("Map loading failed, more than one goal tile."),
                        }
                    },
                    Some(d) => tile_line.push(d),
                    None => tile_line.push(0),
                };
            }

            tiles.push(tile_line);
                
        }

        if let None = start { return Err("Map loading failed, no start tile.") }
        if let None = goal { return Err("Map loading failed, no end tile.") }

        Ok((tiles, start.unwrap(),goal.unwrap()))
    }
}
