pub type Idx2 = (i32, i32);

#[derive(Debug, Default)]
pub struct Level {
    pub current: usize,
    pub grid: Option<LevelFile>
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
        LevelFile {
            dims: (grid[0].len() as i32 - 1, grid.len() as i32),
            grid,
        }
    }

    pub fn get(&self, pos: Idx2) -> char {
        self.grid[(self.dims.1 - 1 - pos.1) as usize][pos.0 as usize]
    }
}
