use std::default;

use bevy::{prelude::*, render::extract_resource::ExtractResource};
use bevy_egui::EguiContexts;

use crate::{
    height_grid::{
        cell::{Coord, FlipAxis, FlipCorner},
        corner::{Corner, CORNERS},
        mesh_builder::RequiresMeshing,
        HeightGrid,
    },
    input::{HitPoint, TerrainRaycast},
};

pub struct TerrainEditorPlugin;

impl Plugin for TerrainEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditConfig {
            strength: 1,
            ..default()
        })
        .add_systems(Update, (edit, config_ui));
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
enum EditMode {
    #[default]
    Corner,
    Vertex,
    Cell,
}

#[derive(Resource, Debug, Default)]
struct EditConfig {
    strength: i32,
    mode: EditMode,
}
fn config_ui(mut contexts: EguiContexts, mut edit_config: ResMut<EditConfig>) {
    use bevy_egui::egui;

    egui::Window::new("Editor Config").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Strength");
            ui.add(egui::DragValue::new(&mut edit_config.strength).speed(1.0));
        });

        ui.label("Mode");

        ui.radio_value(&mut edit_config.mode, EditMode::Corner, "Corner");
        ui.radio_value(&mut edit_config.mode, EditMode::Vertex, "Vertex");
        ui.radio_value(&mut edit_config.mode, EditMode::Cell, "Cell");
    });
}

fn edit(
    mut commands: Commands,
    edit_config: Res<EditConfig>,
    hit_point: Res<TerrainRaycast>,
    mut height_grid_q: Query<&mut HeightGrid>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        return;
    }

    let inverse = mouse_button.just_pressed(MouseButton::Right);

    if let Some(HitPoint {
        position, entity, ..
    }) = hit_point.hit_point
    {
        let vec2 = position.xy();
        let vec2r = vec2.round();

        let Vec2 { x: rx, y: ry } = vec2r;
        let UVec2 { x, y } = vec2r.as_uvec2();

        let (coord, corner) = match vec2 {
            Vec2 { x: fx, y: fy } if fx < rx && fy < ry => ((x - 1, y - 1), Corner::TopRight),
            Vec2 { x: fx, .. } if fx < rx => ((x - 1, y), Corner::BottomRight),
            Vec2 { y: fy, .. } if fy < ry => ((x, y - 1), Corner::TopLeft),
            _ => ((x, y), Corner::BottomLeft),
        };

        let mut height_grid = height_grid_q
            .get_mut(entity)
            .expect("hit non existing terrain");

        modify_terrain(&mut height_grid, coord, corner, &edit_config, inverse);

        commands.entity(entity).insert(RequiresMeshing);
    }
}

fn modify_terrain(
    height_grid: &mut HeightGrid,
    coord: (u32, u32),
    corner: Corner,
    EditConfig { strength, mode }: &EditConfig,
    inverse: bool,
) {
    let delta = if inverse { -strength } else { *strength };
    match mode {
        EditMode::Corner => modify_corner(height_grid, coord, corner, delta),
        EditMode::Vertex => {
            modify_corner(height_grid, coord, corner, delta);
            if let Some((coord, corner)) = (coord, corner).flip(FlipAxis::Horizontal) {
                modify_corner(height_grid, coord, corner, delta);
            }
            if let Some((coord, corner)) = (coord, corner).flip(FlipAxis::Vertical) {
                modify_corner(height_grid, coord, corner, delta);
            }
            if let Some((coord, corner)) = (coord, corner).flip(FlipAxis::Diagonal) {
                modify_corner(height_grid, coord, corner, delta);
            }
        }
        EditMode::Cell => {
            for corner in CORNERS {
                modify_corner(height_grid, coord, corner, delta);
            }
        }
    }
}

fn modify_corner(height_grid: &mut HeightGrid, coord: (u32, u32), corner: Corner, delta: i32) {
    if !height_grid.valid_coord(coord) {
        return;
    }
    let cell = height_grid.get_cell_mut(coord);
    let new_height = cell.get_height(corner).saturating_add_signed(delta);
    cell.set_height(corner, new_height);
}

// fn edit(terrain_edit: Option<Res<TerrainEdit>>) {
//     if let Some(res) = terrain_edit {
//         info!("{:?} {:?}", res.coord, res.corner);
//     }
// }
// fn finish_edit(mut commands: Commands) {
//     commands.remove_resource::<TerrainEdit>();
// }
