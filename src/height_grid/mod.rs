pub mod cell;
pub mod corner;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, MeshBuilder, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use cell::Cell;
use corner::Corner;

enum CellSplit {
    Slash,
    Backslash,
}

enum CellVertexType {
    Shared,
    Split,
}
/// A grid where each cell contains 4 height values, one for each of its corners.
/// Small example:
/// A--B,E--F
/// |   |   |
/// |   |   |
/// 8--9,C--D
/// 2--3,6--7
/// |   |   |
/// |   |   |
/// 0--1,4--5
pub struct HeightGrid {
    pub cells_count: (u32, u32),
    pub cells: Vec<Cell>,
}

struct CellMeshData {
    positions: Vec<Vec3>,
    indices: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
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

    fn get_cell(&self, cell: (u32, u32)) -> &Cell {
        let cell_index = self.get_cell_index(cell);

        self.cells.get(cell_index).expect("index out of bounds")
    }

    fn get_position(&self, cell: (u32, u32), corner: Corner) -> Vec3 {
        let cell_data = self.get_cell(cell);
        let height = cell_data.get_height(corner);

        let (col_offset, row_offset) = corner.get_corner_offset();
        Vec3::new(
            cell.0 as f32 + col_offset,
            cell.1 as f32 + row_offset,
            height as f32,
        )
    }

    fn get_cell_type(&self, cell: (u32, u32)) -> (CellSplit, CellVertexType) {
        let cell = self.get_cell(cell);

        let tl = cell.get_height(Corner::TopLeft);
        let tr = cell.get_height(Corner::TopRight);
        let bl = cell.get_height(Corner::BottomLeft);
        let br = cell.get_height(Corner::BottomRight);

        let backslash_equal = tl == br;
        let slash_equal = bl == tr;

        if backslash_equal && slash_equal {
            use std::cmp::Ordering::*;
            match tl.cmp(&tr) {
                Less => (CellSplit::Slash, CellVertexType::Split),
                Equal => (CellSplit::Slash, CellVertexType::Shared),
                Greater => (CellSplit::Backslash, CellVertexType::Split),
            }
        } else {
            if backslash_equal {
                (CellSplit::Backslash, CellVertexType::Split)
            } else {
                (CellSplit::Slash, CellVertexType::Split)
            }
        }
    }

    fn create_grid_cell(
        &self,
        cell: (u32, u32),
        positions: &mut Vec<Vec3>,
        indices: &mut Vec<u32>,
        normals: &mut Vec<[f32; 3]>,
        uvs: &mut Vec<[f32; 2]>,
    ) {
        let (split, vertex_type) = self.get_cell_type(cell);

        let array_offset = positions.len() as u32;

        let mut data = match vertex_type {
            CellVertexType::Shared => self.create_flat_cell(array_offset, cell, split),
            CellVertexType::Split => self.create_split_cell(array_offset, cell, split),
        };

        positions.append(&mut data.positions);
        indices.append(&mut data.indices);
        normals.append(&mut data.normals);
        uvs.append(&mut data.uvs);
    }

    fn create_split_cell(
        &self,
        array_offset: u32,
        cell: (u32, u32),
        split: CellSplit,
    ) -> CellMeshData {
        let mut positions = vec![];
        let mut indices = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut uvs = vec![];

        let tl = self.get_position(cell, Corner::TopLeft);
        let tr = self.get_position(cell, Corner::TopRight);
        let bl = self.get_position(cell, Corner::BottomLeft);
        let br = self.get_position(cell, Corner::BottomRight);

        let bl_br = (br - bl).normalize_or(Vec3::Z);
        let br_tr = (tr - br).normalize_or(Vec3::Z);
        let tl_tr = (tr - tl).normalize_or(Vec3::Z);
        let bl_tl = (tl - bl).normalize_or(Vec3::Z);

        let bl_tr = (tr - bl).normalize_or(Vec3::Z);
        let br_tl = (tl - br).normalize_or(Vec3::Z);
        match split {
            CellSplit::Slash => {
                positions.push(tl);
                positions.push(bl);
                positions.push(tr);

                positions.push(tr);
                positions.push(bl);
                positions.push(br);

                normals.push((-bl_tl).cross(tl_tr).into());
                normals.push((bl_tr).cross(bl_tl).into());
                normals.push((-tl_tr).cross(-bl_tr).into());

                normals.push((-bl_tr).cross(-br_tr).into());
                normals.push(bl_br.cross(bl_tr).into());
                normals.push(br_tr.cross(-bl_br).into());
                uvs.push([0.0, 1.0]);
                uvs.push([0.0, 0.0]);
                uvs.push([1.0, 1.0]);

                uvs.push([1.0, 1.0]);
                uvs.push([0.0, 0.0]);
                uvs.push([1.0, 0.0]);
            }

            CellSplit::Backslash => {
                positions.push(tl);
                positions.push(bl);
                positions.push(br);

                positions.push(tl);
                positions.push(br);
                positions.push(tr);

                normals.push((-bl_tl).cross(tl_tr).into());
                normals.push((bl_br).cross(bl_tl).into());
                normals.push((br_tl).cross(-bl_br).into());

                normals.push((-br_tl).cross(tl_tr).into());
                normals.push(br_tr.cross(br_tl).into());
                normals.push((-tl_tr).cross(-br_tr).into());

                uvs.push([0.0, 1.0]);
                uvs.push([0.0, 0.0]);
                uvs.push([1.0, 0.0]);

                uvs.push([0.0, 1.0]);
                uvs.push([1.0, 0.0]);
                uvs.push([1.0, 1.0]);
            }
        };

        indices.push(array_offset + 0);
        indices.push(array_offset + 1);
        indices.push(array_offset + 2);

        indices.push(array_offset + 3);
        indices.push(array_offset + 4);
        indices.push(array_offset + 5);

        CellMeshData {
            positions,
            indices,
            normals,
            uvs,
        }
    }
    fn create_flat_cell(
        &self,
        array_offset: u32,
        cell: (u32, u32),
        split: CellSplit,
    ) -> CellMeshData {
        let mut positions = vec![];
        let mut indices = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];

        let tl = self.get_position(cell, Corner::TopLeft);
        let tr = self.get_position(cell, Corner::TopRight);
        let bl = self.get_position(cell, Corner::BottomLeft);
        let br = self.get_position(cell, Corner::BottomRight);
        positions.push(tl);
        positions.push(tr);
        positions.push(bl);
        positions.push(br);

        let (o1, o2, o3, o4, o5, o6) = match split {
            CellSplit::Slash => (0, 2, 1, 1, 2, 3),
            CellSplit::Backslash => (0, 3, 1, 0, 2, 3),
        };

        indices.push(array_offset + o1);
        indices.push(array_offset + o2);
        indices.push(array_offset + o3);

        indices.push(array_offset + o4);
        indices.push(array_offset + o5);
        indices.push(array_offset + o6);

        normals.push(Vec3::Z.into());
        normals.push(Vec3::Z.into());
        normals.push(Vec3::Z.into());
        normals.push(Vec3::Z.into());

        uvs.push([0.0, 1.0].into());
        uvs.push([1.0, 1.0].into());
        uvs.push([0.0, 0.0].into());
        uvs.push([1.0, 0.0].into());

        CellMeshData {
            positions,
            indices,
            normals,
            uvs,
        }
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
                self.create_grid_cell((x, y), &mut positions, &mut indices, &mut normals, &mut uvs);
            }
        }

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
