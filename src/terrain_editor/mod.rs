use bevy::{prelude::*, render::extract_resource::ExtractResource};

use crate::{
    height_grid::{cell::Coord, corner::Corner, mesh_builder::RequiresMeshing, HeightGrid},
    input::{HitPoint, TerrainRaycast},
};

pub struct TerrainEditorPlugin;

impl Plugin for TerrainEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainEditConfig { strength: 1 })
            .add_systems(Update, init_edit);
    }
}

#[derive(Resource, Debug, Default)]
struct TerrainEditConfig {
    strength: i32,
}

fn init_edit(
    mut commands: Commands,
    terrain_edit_config: Res<TerrainEditConfig>,
    terrain_hit_point: Res<TerrainRaycast>,
    mut height_grid_q: Query<&mut HeightGrid>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(HitPoint {
        position, entity, ..
    }) = terrain_hit_point.hit_point
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

        let cell = height_grid.get_cell_mut(coord);
        let new_height = cell
            .get_height(corner)
            .saturating_add_signed(terrain_edit_config.strength);

        cell.set_height(corner, new_height);

        commands.entity(entity).insert(RequiresMeshing);
    }
}

// fn edit(terrain_edit: Option<Res<TerrainEdit>>) {
//     if let Some(res) = terrain_edit {
//         info!("{:?} {:?}", res.coord, res.corner);
//     }
// }
// fn finish_edit(mut commands: Commands) {
//     commands.remove_resource::<TerrainEdit>();
// }
