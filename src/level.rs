#[derive(Debug, Default)]
pub struct Level {
    pub current: usize,
    pub grid: Option<LevelFile>,
}

#[derive(Debug, Default)]
pub struct LevelFile {
    pub n_rows: usize,
    pub n_columns: usize,
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
            n_rows: grid.len(),
            n_columns: grid[0].len(),
            grid,
        }
    }

    pub fn get(&self, row: usize, column: usize) -> char {
        self.grid[row][column]
    }
}
