use crate::tile::Tile;

mod array_util;
mod bitmask;
pub mod board;
pub mod tile;

pub struct Solution {
    pub placements: Vec<TilePlacement>,
}

pub struct TilePlacement {
    pub tile: Tile,
    pub position: (usize, usize),
}

pub enum UnsolvableReason {
    TooFewTiles,
    TooManyTiles,
    NoFit,
}

pub fn solve_all_filling(
    board: board::Board,
    tiles: &[Tile],
) -> Result<Solution, UnsolvableReason> {
    board.debug_print();
    for tile in tiles {
        tile.debug_print();
    }
    Err(UnsolvableReason::NoFit)
}
