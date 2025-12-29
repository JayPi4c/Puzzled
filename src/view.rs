use crate::application::GRID_SIZE;
use gtk::prelude::{GridExt, WidgetExt};
use gtk::Grid;
use ndarray::Array2;

pub struct TileView {
  pub  parent: Grid,
}

impl TileView {
    pub fn new(base: Array2<bool>) -> Self {
        let grid = Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true);

        let (rows, cols) = base.dim();

        for r in 0..rows {
            for c in 0..cols {
                if base[[r, c]] {
                    let cell = gtk::Frame::new(None);
                    cell.set_width_request(GRID_SIZE);
                    cell.set_height_request(GRID_SIZE);

                    grid.attach(&cell, c as i32, r as i32, 1, 1);
                }
            }
        }

        TileView { parent: grid }
    }
}
