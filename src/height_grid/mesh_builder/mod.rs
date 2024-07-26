mod mesh_data;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use mesh_data::MeshData;

use super::{
    cell::{Coord, FlipCorner},
    corner::Corner,
    HeightGrid,
};

pub struct Builder<'a> {
    pub height_grid: &'a HeightGrid,
}

impl<'a> Builder<'a> {
    pub fn new(height_grid: &'a HeightGrid) -> Self {
        Self { height_grid }
    }
}

impl MeshBuilder for Builder<'_> {
    fn build(&self) -> Mesh {
        let num_vertices =
            self.height_grid.cells_count.0 as usize * self.height_grid.cells_count.1 as usize * 4;
        let num_indices =
            self.height_grid.cells_count.0 as usize * self.height_grid.cells_count.1 as usize * 6;

        let mut mesh_data = MeshData {
            positions: Vec::with_capacity(num_vertices),
            normals: Vec::with_capacity(num_vertices),
            uvs: Vec::with_capacity(num_vertices),
            indices: Vec::with_capacity(num_indices),
        };

        for y in 0..self.height_grid.cells_count.1 {
            for x in 0..self.height_grid.cells_count.0 {
                let cell = (x, y);
                let grid = self.height_grid;
                let mesh_type = get_cell_type(grid, cell);
                {
                    match mesh_type {
                        CellMeshType::Shared => create_flat_cell(grid, &mut mesh_data, cell),
                        CellMeshType::Slash => create_split_cell(grid, &mut mesh_data, cell, true),
                        CellMeshType::Backslash => {
                            create_split_cell(grid, &mut mesh_data, cell, false)
                        }
                    };
                }
                {
                    create_cliffs(grid, &mut mesh_data, cell);
                }
            }
        }

        let MeshData {
            positions,
            normals,
            uvs,
            indices,
        } = mesh_data;

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

enum CellMeshType {
    Shared,
    Slash,
    Backslash,
}

fn get_cell_type(height_grid: &HeightGrid, cell: Coord) -> CellMeshType {
    let cell = height_grid.get_cell(cell);

    let tl = cell.get_height(Corner::TopLeft);
    let tr = cell.get_height(Corner::TopRight);
    let bl = cell.get_height(Corner::BottomLeft);
    let br = cell.get_height(Corner::BottomRight);

    let backslash_equal = tl == br;
    let slash_equal = bl == tr;

    if backslash_equal && slash_equal {
        use std::cmp::Ordering::*;
        match tl.cmp(&tr) {
            Less => CellMeshType::Slash,
            Equal => CellMeshType::Shared,
            Greater => CellMeshType::Backslash,
        }
    } else if backslash_equal {
        CellMeshType::Backslash
    } else {
        CellMeshType::Slash
    }
}

fn create_split_cell(height_grid: &HeightGrid, mesh_data: &mut MeshData, cell: Coord, slash: bool) {
    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);
    if slash {
        mesh_data.create_triangle(&[tl, bl, tr]);
        mesh_data.create_triangle(&[tr, bl, br]);
    } else {
        mesh_data.create_triangle(&[tl, bl, br]);
        mesh_data.create_triangle(&[tl, br, tr]);
    };
}
fn create_flat_cell(height_grid: &HeightGrid, mesh_data: &mut MeshData, cell: Coord) {
    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);
    mesh_data.create_quad(&[tl, tr, bl, br]);
}

fn create_cliffs(grid: &HeightGrid, mesh_data: &mut MeshData, cell: Coord) {
    use super::cell::FlipAxis::*;
    use super::Corner::*;
    create_cliff(grid, mesh_data, cell, (BottomLeft, BottomRight), Horizontal);
    create_cliff(grid, mesh_data, cell, (BottomRight, TopRight), Vertical);
    create_cliff(grid, mesh_data, cell, (TopRight, TopLeft), Horizontal);
    create_cliff(grid, mesh_data, cell, (TopLeft, BottomLeft), Vertical);
}

fn create_cliff(
    grid: &HeightGrid,
    mesh_data: &mut MeshData,
    cell: (u32, u32),
    (left, right): (Corner, Corner),
    axis: super::cell::FlipAxis,
) {
    let left_height = grid.get_cell(cell).get_height(left);
    let right_height = grid.get_cell(cell).get_height(right);

    let left_opp = (cell, left).flip(axis);
    let right_opp = (cell, right).flip(axis);

    if let (Some((opp_coord, opp_corner_l)), Some((_, opp_corner_r))) = (left_opp, right_opp) {
        if let Some(opp_cell) = grid.try_get_cell(opp_coord) {
            let left_opposite_height = opp_cell.get_height(opp_corner_l);

            let right_opposite_height = opp_cell.get_height(opp_corner_r);

            let l_pos = grid.get_position(cell, left);
            let r_pos = grid.get_position(cell, right);
            let ol_pos = grid.get_position(opp_coord, opp_corner_l);
            let or_pos = grid.get_position(opp_coord, opp_corner_r);
            if left_opposite_height < left_height {
                mesh_data.create_triangle(&[l_pos, ol_pos, or_pos].into());
            }
            if right_opposite_height < right_height {
                mesh_data.create_triangle(&[l_pos, or_pos, r_pos].into());
            }
        }
    }
}
