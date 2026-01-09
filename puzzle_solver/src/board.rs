use crate::array_util;
use log::debug;
use ndarray::Array2;
use std::ops::{Index, IndexMut};

pub struct Board(Array2<bool>);

impl Board {
    pub fn default(dim: (usize, usize)) -> Self {
        Board(Array2::from_elem(dim, false))
    }
}

impl Board {
    pub fn new(dims: (usize, usize)) -> Self {
        Board(Array2::default(dims))
    }

    pub fn debug_print(&self) {
        debug!("Board:");
        array_util::debug_print(&self.0);
    }
}

impl Index<[usize; 2]> for Board {
    type Output = bool;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[[index[0], index[1]]]
    }
}

impl IndexMut<[usize; 2]> for Board {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[[index[0], index[1]]]
    }
}
