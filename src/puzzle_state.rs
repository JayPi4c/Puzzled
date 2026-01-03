use crate::puzzle::PuzzleConfig;
use gtk::Widget;
use ndarray::Array2;

#[derive(Default, Debug)]
pub struct CellData {
    pub allowed: bool,
}

#[derive(Debug)]
pub enum Cell {
    Empty(CellData),
    One(CellData, Widget),
    Many(CellData, Vec<Widget>),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty(CellData::default())
    }
}

#[derive(Debug)]
pub struct PuzzleState {
    pub grid: Array2<Cell>,
}

impl PuzzleState {
    pub fn new(puzzle_config: &PuzzleConfig) -> Self {
        let layout = &puzzle_config.board_layout;

        let dim = layout.dim();
        let mut grid: Array2<Cell> = Array2::default(dim);

        for ((r, c), cell) in grid.indexed_iter_mut() {
            let allowed = layout[[r, c]];
            *cell = Cell::Empty(CellData { allowed });
        }

        PuzzleState { grid }
    }
}
