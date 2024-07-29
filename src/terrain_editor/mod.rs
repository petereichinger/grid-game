use bevy::{prelude::*, render::extract_resource::ExtractResource};

use crate::{
    height_grid::{cell::Coord, corner::Corner},
    input::{HitPoint, TerrainRaycast},
};

pub struct TerrainEditorPlugin;

impl Plugin for TerrainEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (init_edit, edit, finish_edit).chain());
    }
}

#[derive(Resource, Debug, Default)]
struct TerrainEdit {
    coord: Coord,
    corner: Corner,
}
fn init_edit(
    mut commands: Commands,
    terrain_hit_point: Res<TerrainRaycast>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let left_down = mouse_button.just_pressed(MouseButton::Left);

    if !left_down {
        return;
    }

    if let Some(HitPoint { position, .. }) = terrain_hit_point.hit_point {
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

        commands.insert_resource(TerrainEdit { coord, corner })
    }
}

fn edit(terrain_edit: Option<Res<TerrainEdit>>) {
    if let Some(res) = terrain_edit {
        info!("{:?} {:?}", res.coord, res.corner);
    }
}
fn finish_edit(mut commands: Commands) {
    commands.remove_resource::<TerrainEdit>();
}
