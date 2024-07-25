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
    let array_offset: u32 = mesh_data
        .positions
        .len()
        .try_into()
        .expect("must be a valid u32");

    let MeshData {
        positions,
        normals,
        uvs,
        indices,
    } = mesh_data;
    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);

    let bl_br = (br - bl).normalize_or(Vec3::Z);
    let br_tr = (tr - br).normalize_or(Vec3::Z);
    let tl_tr = (tr - tl).normalize_or(Vec3::Z);
    let bl_tl = (tl - bl).normalize_or(Vec3::Z);

    let bl_tr = (tr - bl).normalize_or(Vec3::Z);
    let br_tl = (tl - br).normalize_or(Vec3::Z);
    if slash {
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
    } else {
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
    };

    indices.push(array_offset);
    indices.push(array_offset + 1);
    indices.push(array_offset + 2);

    indices.push(array_offset + 3);
    indices.push(array_offset + 4);
    indices.push(array_offset + 5);
}
fn create_flat_cell(height_grid: &HeightGrid, mesh_data: &mut MeshData, cell: Coord) {
    let array_offset: u32 = mesh_data
        .positions
        .len()
        .try_into()
        .expect("must be a valid u32");

    let MeshData {
        positions,
        normals,
        uvs,
        indices,
    } = mesh_data;

    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);
    positions.push(tl);
    positions.push(tr);
    positions.push(bl);
    positions.push(br);

    let (o1, o2, o3, o4, o5, o6) = (0, 2, 1, 1, 2, 3);

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

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 0.0]);
}

fn create_cliffs(grid: &HeightGrid, mesh_data: &mut MeshData, cell: Coord) {
    use super::cell::FlipAxis::*;
    use super::Corner::*;
    create_cliff(grid, mesh_data, cell, (BottomLeft, BottomRight), Horizontal);
    create_cliff(grid, mesh_data, cell, (BottomRight, TopRight), Vertical);
    create_cliff(grid, mesh_data, cell, (TopRight, TopLeft), Horizontal);
    create_cliff(grid, mesh_data, cell, (TopLeft, BottomRight), Vertical);
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
