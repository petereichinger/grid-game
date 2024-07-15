use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshBuilder, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    fn get_corner_offset(&self) -> (f32, f32) {
        match self {
            Corner::TopLeft => (0.0, 0.0),
            Corner::TopRight => (1.0, 0.0),
            Corner::BottomLeft => (0.0, 1.0),
            Corner::BottomRight => (1.0, 1.0),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cell {
    heights: (u32, u32, u32, u32),
}

impl From<(u32, u32, u32, u32)> for Cell {
    fn from(value: (u32, u32, u32, u32)) -> Self {
        Self { heights: value }
    }
}

impl Cell {
    fn get_height(&self, corner: Corner) -> u32 {
        match corner {
            Corner::TopLeft => self.heights.0,
            Corner::TopRight => self.heights.1,
            Corner::BottomLeft => self.heights.2,
            Corner::BottomRight => self.heights.3,
        }
    }
}

/// A grid where each cell contains 4 height values, one for each of its corners.
/// Small example:
/// 0--1,4--5
/// |   |   |
/// |   |   |
/// 2--3,6--7
/// 8--9,C--D
/// |   |   |
/// |   |   |
/// A--B,E--F
pub struct HeightGrid {
    pub cells_count: (u32, u32),
    pub cells: Vec<Cell>,
}

impl HeightGrid {
    pub fn new(cells_count: (u32, u32), cells: Vec<Cell>) -> Self {
        let (cells_width, cells_depth) = cells_count;
        assert!(cells_width > 0);
        assert!(cells_depth > 0);
        assert!(cells.len() > 0);
        assert_eq!((cells_width * cells_depth) as usize, cells.len());

        Self { cells_count, cells }
    }

    fn get_cell_index(&self, cell: (u32, u32)) -> usize {
        let (cells_width, cells_depth) = self.cells_count;
        let (cell_x, cell_y) = cell;
        assert!(cell_x < cells_width);
        assert!(cell_y < cells_depth);

        (cells_width * cell_y + cell_x) as usize
    }

    fn get_position(&self, cell: (u32, u32), corner: Corner) -> Vec3 {
        let cell_index = self.get_cell_index(cell);

        let cell_data = self.cells.get(cell_index).expect("index out of bounds");

        let height = cell_data.get_height(corner);

        let (col_offset, row_offset) = corner.get_corner_offset();
        Vec3::new(
            cell.0 as f32 + col_offset,
            cell.1 as f32 + row_offset,
            height as f32,
        )
    }
}

impl MeshBuilder for HeightGrid {
    fn build(&self) -> Mesh {
        let num_vertices = self.cells_count.0 as usize * self.cells_count.1 as usize * 4;
        let num_indices = self.cells_count.0 as usize * self.cells_count.1 as usize * 6;
        let mut positions: Vec<Vec3> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        for y in 0..self.cells_count.1 {
            for x in 0..self.cells_count.0 {
                let array_offset = (self.cells_count.0 * y + x) * 4;
                dbg!(array_offset);
                let cell = (x, y);
                let p0 = self.get_position(cell, Corner::TopLeft);
                let p1 = self.get_position(cell, Corner::TopRight);
                let p2 = self.get_position(cell, Corner::BottomLeft);
                let p3 = self.get_position(cell, Corner::BottomRight);

                positions.push(p0);
                positions.push(p1);
                positions.push(p2);
                positions.push(p3);

                normals.push(Vec3::Z.into());
                normals.push(Vec3::Z.into());
                normals.push(Vec3::Z.into());
                normals.push(Vec3::Z.into());

                uvs.push([0.0, 0.0]);
                uvs.push([1.0, 0.0]);
                uvs.push([0.0, 1.0]);
                uvs.push([1.0, 1.0]);

                let i0 = array_offset;
                let i1 = array_offset + 1;
                let i2 = array_offset + 2;
                let i3 = array_offset + 3;

                indices.push(i0 as u32);
                indices.push(i1 as u32);
                indices.push(i3 as u32);

                indices.push(i0 as u32);
                indices.push(i3 as u32);
                indices.push(i2 as u32);
            }
        }

        dbg!(&positions);
        dbg!(&normals);
        dbg!(&uvs);
        dbg!(&indices);

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
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
