use super::cell::{Cell, Coord};
use super::corner::Corner;
use bevy::prelude::*;

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
#[derive(Component, Debug)]
pub struct HeightGrid {
    pub cells_count: (u32, u32),
    pub cells: Box<[Cell]>,
}

impl HeightGrid {
    pub fn new(cells_count: (u32, u32), cells: impl Into<Box<[Cell]>>) -> Self {
        let cells: Box<[Cell]> = cells.into();
        let (cells_width, cells_depth) = cells_count;
        assert!(cells_width > 0);
        assert!(cells_depth > 0);
        assert!(!cells.is_empty());
        assert_eq!((cells_width * cells_depth) as usize, cells.len());
        Self { cells_count, cells }
    }

    pub fn valid_coord(&self, coord: Coord) -> bool {
        coord.0 < self.cells_count.0 && coord.1 < self.cells_count.1
    }

    pub fn get_cell_index(&self, cell: Coord) -> usize {
        let (cells_width, cells_depth) = self.cells_count;
        let (cell_x, cell_y) = cell;
        assert!(cell_x < cells_width);
        assert!(cell_y < cells_depth);

        (cells_width * cell_y + cell_x) as usize
    }

    pub fn try_get_cell(&self, coord: Coord) -> Option<&Cell> {
        if !self.valid_coord(coord) {
            return None;
        }

        return Some(self.get_cell(coord));
    }
    pub fn get_cell(&self, coord: Coord) -> &Cell {
        let cell_index = self.get_cell_index(coord);

        self.cells.get(cell_index).expect("index out of bounds")
    }

    pub fn get_position(&self, coord: Coord, corner: Corner) -> Vec3 {
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

    #[test]
    fn valid_coord_works() {
        let grid = HeightGrid::new((2, 2), vec![(0, 0, 0, 0).into(); 4]);

        assert!(grid.valid_coord((0, 0)));
        assert!(grid.valid_coord((1, 0)));
        assert!(grid.valid_coord((0, 1)));
        assert!(grid.valid_coord((1, 1)));

        assert!(!grid.valid_coord((2, 2)));
    }

    #[test]
    fn try_get_cell_works() {
        let grid = HeightGrid::new((2, 2), vec![(0, 0, 0, 0).into(); 4]);
        let cell: Cell = (0, 0, 0, 0).into();
        assert_eq!(grid.try_get_cell((0, 0)), Some(&cell));
        assert_eq!(grid.try_get_cell((2, 2)), None);
    }

    #[test]
    #[should_panic]
    fn get_cell_panics() {
        let grid = HeightGrid::new((2, 2), vec![(0, 0, 0, 0).into(); 4]);
        grid.get_cell((2, 2));
    }

    #[test]
    fn get_position() {
        let grid = HeightGrid::new(
            (2, 2),
            [
                (0, 0, 0, 0).into(),
                (0, 0, 0, 1).into(),
                (1, 1, 2, 1).into(),
                (1, 0, 1, 1).into(),
            ],
        );

        assert_eq!(
            grid.get_position((0, 0), Corner::TopLeft),
            Vec3::new(0.0, 1.0, 0.0)
        );
        assert_eq!(
            grid.get_position((1, 0), Corner::BottomRight),
            Vec3::new(2.0, 0.0, 1.0)
        );
        assert_eq!(
            grid.get_position((0, 1), Corner::BottomLeft),
            Vec3::new(0.0, 1.0, 2.0)
        );
    }
}
