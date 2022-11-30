pub type Idx2 = (usize, usize);

#[derive(Debug, Default)]
pub struct Level {
    pub current: usize,
    pub grid: Option<LevelFile>,
}

#[derive(Debug, Default)]
pub struct LevelFile {
    pub dims: Idx2,
    pub grid: Vec<Vec<char>>,
}

impl LevelFile {
    pub fn new(file_contents: &str) -> LevelFile {
        let grid: Vec<Vec<char>> = file_contents
            .lines()
            .map(|s| s.chars().collect())
            .filter(|s: &Vec<char>| !s.is_empty())
            .collect();
        dbg!(&grid[0].len(), &grid.len());
        LevelFile {
            dims: (grid[0].len(), grid.len()),
            grid,
        }
    }

    pub fn get(&self, pos: Idx2) -> char {
        self.grid[(self.dims.1 - 1 - pos.1)][pos.0]
    }
}
