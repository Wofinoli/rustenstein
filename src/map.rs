#[derive(Debug)]
pub struct Map {
    tiles: Vec<Vec<u32>>,
    name: String,
}

impl Map {
    pub fn new(tiles: Vec<Vec<u32>>, name: String) -> Map {
        Map {
            tiles,
            name,
        }
    }
}
