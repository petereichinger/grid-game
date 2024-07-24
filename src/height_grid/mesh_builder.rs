use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};

use super::{corner::Corner, HeightGrid};

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
        let mut positions: Vec<Vec3> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        for y in 0..self.height_grid.cells_count.1 {
            for x in 0..self.height_grid.cells_count.0 {
                let cell = (x, y);
                let grid = self.height_grid;
                let mesh_type = get_cell_type(grid, cell);

                let array_offset = positions.len() as u32;

                let mut data = match mesh_type {
                    CellMeshType::Shared => create_flat_cell(grid, array_offset, cell),
                    CellMeshType::Slash => create_split_cell(grid, array_offset, cell, true),
                    CellMeshType::Backslash => create_split_cell(grid, array_offset, cell, false),
                };

                positions.append(&mut data.positions);
                indices.append(&mut data.indices);
                normals.append(&mut data.normals);
                uvs.append(&mut data.uvs);
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

enum CellMeshType {
    Shared,
    Slash,
    Backslash,
}

struct CellMeshData {
    positions: Vec<Vec3>,
    indices: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
}

fn get_cell_type(height_grid: &HeightGrid, cell: (u32, u32)) -> CellMeshType {
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

fn create_grid_cell(
    height_grid: &HeightGrid,
    cell: (u32, u32),
    positions: &mut Vec<Vec3>,
    indices: &mut Vec<u32>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
) {
}

fn create_split_cell(
    height_grid: &HeightGrid,
    array_offset: u32,
    cell: (u32, u32),
    slash: bool,
) -> CellMeshData {
    let mut positions = vec![];
    let mut indices = vec![];
    let mut normals: Vec<[f32; 3]> = vec![];
    let mut uvs = vec![];

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

    CellMeshData {
        positions,
        indices,
        normals,
        uvs,
    }
}
fn create_flat_cell(height_grid: &HeightGrid, array_offset: u32, cell: (u32, u32)) -> CellMeshData {
    let mut positions = vec![];
    let mut indices = vec![];
    let mut normals = vec![];
    let mut uvs = vec![];

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

    CellMeshData {
        positions,
        indices,
        normals,
        uvs,
    }
}
