use crate::offset::{CellOffset, PixelOffset};
use crate::presenter::board::BoardPresenter;
use crate::presenter::tile::TilePresenter;
use crate::state::get_state;
use crate::view::{BoardView, TileView};
use gtk::prelude::{FixedExt, WidgetExt};
use gtk::{Fixed, Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub const WINDOW_TO_BOARD_RATIO: f64 = 2.5;

/// Configuration for the puzzle grid layout.
#[derive(Debug, Default)]
pub struct GridConfig {
    pub grid_h_cell_count: u32,
    pub cell_width_pixel: u32,
    pub board_offset_cells: PixelOffset,
}

#[derive(Debug, Default)]
pub struct PuzzleAreaData {
    pub fixed: Option<Fixed>,
    pub elements_in_fixed: Vec<Widget>,
    pub board_view: Option<BoardView>,
    pub tile_views: Vec<TileView>,
    pub grid_config: GridConfig,
}

impl PuzzleAreaData {
    pub fn add_to_fixed(&mut self, widget: &Widget, pos: &PixelOffset) {
        match &self.fixed {
            Some(fixed) => {
                fixed.put(widget, pos.0, pos.1);
                self.elements_in_fixed.push(widget.clone());
            }
            None => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct PuzzleAreaPresenter {
    data: Rc<RefCell<PuzzleAreaData>>,
    board_presenter: BoardPresenter,
    tile_presenter: TilePresenter,
}

impl PuzzleAreaPresenter {
    pub fn set_view(&self, view: Fixed) {
        self.data.borrow_mut().fixed = Some(view);
        self.clear_elements();
    }

    /// Set up the puzzle configuration from the current state.
    ///
    /// This adds the board and tiles to the puzzle area based on the current puzzle configuration.
    /// Final positions and layout are handled in `update_layout()`. Before that, all elements are
    /// added at position (0, 0) and will be moved later.
    pub fn setup_puzzle_config_from_state(&self) {
        self.clear_elements();

        let state = get_state();
        let puzzle_config = &state.puzzle_config;

        self.board_presenter.setup(puzzle_config);
        let mut position_start = CellOffset(1, 1);
        for tile in puzzle_config.tiles.iter() {
            self.tile_presenter.setup(tile, &position_start);

            let (rows, cols) = tile.base.dim();
            position_start.0 += (rows + 1) as i32;
            if position_start.0 > 10 {
                position_start.0 = 1;
                position_start.1 += (cols + 1) as i32;
            }
        }

        self.update_layout();
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_cell_width();
        self.board_presenter.update_layout();
        self.tile_presenter.update_layout();
    }

    fn update_cell_width(&self) {
        let width = {
            let data = self.data.borrow();
            match &data.fixed {
                Some(fixed) => fixed.width(),
                None => 0,
            }
        };

        let grid_config = &mut self.data.borrow_mut().grid_config;
        grid_config.cell_width_pixel = width as u32 / grid_config.grid_h_cell_count;
    }

    fn clear_elements(&self) {
        let mut data = self.data.borrow_mut();
        if let Some(fixed) = data.fixed.clone() {
            data.elements_in_fixed
                .drain(..)
                .for_each(|e| fixed.remove(&e));
            data.tile_views.clear();
            data.board_view = None;
        }
    }
}

impl Default for PuzzleAreaPresenter {
    fn default() -> Self {
        let data = Rc::new(RefCell::new(PuzzleAreaData::default()));
        let mut board_presenter = BoardPresenter::default();
        board_presenter.set_data(data.clone());
        let mut tile_presenter = TilePresenter::default();
        tile_presenter.set_data(data.clone());
        Self {
            data,
            board_presenter,
            tile_presenter,
        }
    }
}
