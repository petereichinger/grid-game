mod mesh_data;

use avian3d::prelude::{ColliderConstructor, ColliderConstructorHierarchy};
use bevy::prelude::*;
use mesh_data::MeshData;

use crate::{Cliffs, Ground};

use super::flip::*;
use super::{corner::Corner, HeightGrid};

#[derive(Component, Debug)]
pub struct RequiresMeshing;

pub struct MeshBuilderPlugin;

impl Plugin for MeshBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, generate_meshes);
    }
}

fn generate_meshes(
    mut commands: Commands,
    requires_meshing_q: Query<(Entity, &HeightGrid, &Children), With<RequiresMeshing>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cliffs_q: Query<&mut Handle<Mesh>, (With<Cliffs>, Without<Ground>)>,
    mut ground_q: Query<&mut Handle<Mesh>, (With<Ground>, Without<Cliffs>)>,
) {
    for (entity, height_grid, children) in requires_meshing_q.iter() {
        info!("Remeshing");
        let HeightGridMeshes { ground, cliffs } = build(height_grid);

        let cliffs = meshes.add(cliffs);
        let ground = meshes.add(ground);
        for child in children {
            if let Ok(mut cliffs_handle) = cliffs_q.get_mut(*child) {
                *cliffs_handle = cliffs.clone();
            }
            if let Ok(mut ground_handle) = ground_q.get_mut(*child) {
                *ground_handle = ground.clone();
            }
        }
        commands
            .entity(entity)
            .remove::<RequiresMeshing>()
            .remove::<ColliderConstructorHierarchy>()
            .insert(ColliderConstructorHierarchy::new(Some(
                ColliderConstructor::TrimeshFromMesh,
            )));
    }
}

pub struct HeightGridMeshes {
    pub ground: Mesh,
    pub cliffs: Mesh,
}

pub fn build(height_grid: &HeightGrid) -> HeightGridMeshes {
    let mut ground_mesh_data = MeshData::default();
    let mut cliffs_mesh_data = MeshData::default();

    for y in 0..height_grid.cells_count.y {
        for x in 0..height_grid.cells_count.x {
            let cell = UVec2::new(x, y);
            let grid = height_grid;
            let mesh_type = get_cell_type(grid, cell);
            {
                match mesh_type {
                    CellMeshType::Shared => create_flat_cell(grid, &mut ground_mesh_data, cell),
                    CellMeshType::Slash => {
                        create_split_cell(grid, &mut ground_mesh_data, cell, true)
                    }
                    CellMeshType::Backslash => {
                        create_split_cell(grid, &mut ground_mesh_data, cell, false)
                    }
                };
            }
            create_cliffs(grid, &mut cliffs_mesh_data, cell);
        }
    }

    HeightGridMeshes {
        ground: ground_mesh_data.into(),
        cliffs: cliffs_mesh_data.into(),
    }
}

enum CellMeshType {
    Shared,
    Slash,
    Backslash,
}

fn get_cell_type(height_grid: &HeightGrid, cell: UVec2) -> CellMeshType {
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

fn create_split_cell(height_grid: &HeightGrid, mesh_data: &mut MeshData, cell: UVec2, slash: bool) {
    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);
    if slash {
        mesh_data.create_triangle(&[tl, bl, tr], &[[0.0, 1.0], [0.0, 0.0], [1.0, 1.0]]);
        mesh_data.create_triangle(&[tr, bl, br], &[[1.0, 1.0], [0.0, 0.0], [1.0, 0.0]]);
    } else {
        mesh_data.create_triangle(&[tl, bl, br], &[[0.0, 1.0], [0.0, 0.0], [1.0, 0.0]]);
        mesh_data.create_triangle(&[tl, br, tr], &[[0.0, 1.0], [1.0, 0.0], [1.0, 1.0]]);
    };
}
fn create_flat_cell(height_grid: &HeightGrid, mesh_data: &mut MeshData, cell: UVec2) {
    let tl = height_grid.get_position(cell, Corner::TopLeft);
    let tr = height_grid.get_position(cell, Corner::TopRight);
    let bl = height_grid.get_position(cell, Corner::BottomLeft);
    let br = height_grid.get_position(cell, Corner::BottomRight);
    mesh_data.create_quad(
        &[tl, tr, bl, br],
        &[[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
    );
}

fn create_cliffs(grid: &HeightGrid, mesh_data: &mut MeshData, cell: UVec2) {
    use super::corner::Corner::*;
    use super::flip::FlipAxis::*;
    create_cliff(grid, mesh_data, cell, (BottomLeft, BottomRight), Horizontal);
    create_cliff(grid, mesh_data, cell, (BottomRight, TopRight), Vertical);
    create_cliff(grid, mesh_data, cell, (TopRight, TopLeft), Horizontal);
    create_cliff(grid, mesh_data, cell, (TopLeft, BottomLeft), Vertical);
}

fn create_cliff(
    grid: &HeightGrid,
    mesh_data: &mut MeshData,
    cell: UVec2,
    (left, right): (Corner, Corner),
    axis: FlipAxis,
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
                mesh_data.create_triangle(
                    &[l_pos, ol_pos, or_pos],
                    &[
                        [0.0, left_height as f32],
                        [0.0, left_opposite_height as f32],
                        [1.0, right_opposite_height as f32],
                    ],
                );
            }
            if right_opposite_height < right_height {
                mesh_data.create_triangle(
                    &[l_pos, or_pos, r_pos],
                    &[
                        [0.0, left_height as f32],
                        [1.0, right_height as f32],
                        [1.0, right_opposite_height as f32],
                    ],
                );
            }
        }
    }
}
