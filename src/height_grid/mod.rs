pub mod cell;
pub mod corner;
pub mod mesh_builder;

use bevy::prelude::*;
use cell::{Cell, Coord};
use corner::Corner;

/// A grid where each cell contains 4 height values, one for each of its corners.
/// Small example:
/// 8--9,C--D
/// |   |   |
/// |   |   |
/// A--B,E--F
/// 0--1,4--5
/// |   |   |
/// |   |   |
/// 2--3,6--7
pub struct HeightGrid {
    pub cells_count: (u32, u32),
    pub cells: Vec<Cell>,
}

impl HeightGrid {
    pub fn new(cells_count: (u32, u32), cells: Vec<Cell>) -> Self {
        let (cells_width, cells_depth) = cells_count;
        assert!(cells_width > 0);
        assert!(cells_depth > 0);
        assert!(!cells.is_empty());
        assert_eq!((cells_width * cells_depth) as usize, cells.len());

        Self { cells_count, cells }
    }

    fn valid_coord(&self, coord: Coord) -> bool {
        coord.0 < self.cells_count.0 && coord.1 < self.cells_count.1
    }

    fn get_cell_index(&self, cell: (u32, u32)) -> usize {
        let (cells_width, cells_depth) = self.cells_count;
        let (cell_x, cell_y) = cell;
        assert!(cell_x < cells_width);
        assert!(cell_y < cells_depth);

        (cells_width * cell_y + cell_x) as usize
    }

    fn try_get_cell(&self, coord: Coord) -> Option<&Cell> {
        if !self.valid_coord(coord) {
            return None;
        }

        return Some(self.get_cell(coord));
    }
    fn get_cell(&self, coord: Coord) -> &Cell {
        let cell_index = self.get_cell_index(coord);

        self.cells.get(cell_index).expect("index out of bounds")
    }

    fn get_position(&self, coord: Coord, corner: Corner) -> Vec3 {
        let cell_data = self.get_cell(coord);
        let height = cell_data.get_height(corner);

        let (col_offset, row_offset) = corner.get_corner_offset();
        Vec3::new(
            coord.0 as f32 + col_offset,
            coord.1 as f32 + row_offset,
            height as f32,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_on_wrong_height_size() {
        HeightGrid::new((4, 4), vec![]);
    }

    #[test]
    #[should_panic]
    fn fail_on_zero_width() {
        HeightGrid::new((0, 1), vec![(1, 1, 1, 1).into()]);
    }

    #[test]
    #[should_panic]
    fn fail_on_zero_depth() {
        HeightGrid::new((1, 0), vec![(1, 1, 1, 1).into()]);
    }

    #[test]
    #[should_panic]
    fn fail_on_empty_array() {
        HeightGrid::new((1, 1), vec![]);
    }

    #[test]
    fn works_on_flat_grid() {
        HeightGrid::new((2, 2), vec![(0, 0, 0, 0).into(); 4]);
    }

    #[test]
    fn get_index() {
        let grid = HeightGrid::new((2, 2), vec![(0, 0, 0, 0).into(); 4]);

        assert_eq!(grid.get_cell_index((0, 0)), 0);

        assert_eq!(grid.get_cell_index((1, 0)), 1);

        assert_eq!(grid.get_cell_index((0, 1)), 2);

        assert_eq!(grid.get_cell_index((1, 1)), 3);
    }
}
