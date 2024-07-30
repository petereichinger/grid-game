use avian3d::spatial_query::{RayHitData, SpatialQuery, SpatialQueryFilter};
use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{camera::MainCamera, Terrain};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainRaycast>()
            .init_resource::<CurrentMousePos>()
            .add_systems(
                PreUpdate,
                (
                    update_current_mouse_pos,
                    raycast.after(update_current_mouse_pos),
                ),
            )
            .add_systems(Update, terrain_gizmo);
    }
}
#[derive(Debug, Resource, Default)]
pub struct CurrentMousePos {
    pub position: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct HitPoint {
    pub position: Vec3,
    pub normal: Vec3,
    pub entity: Entity,
}
#[derive(Debug, Resource, Default)]
pub struct TerrainRaycast {
    pub hit_point: Option<HitPoint>,
}

fn update_current_mouse_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut current_mouse_pos: ResMut<CurrentMousePos>,
) {
    if let Some(&CursorMoved { position, .. }) = cursor_moved_events.read().last() {
        current_mouse_pos.position = position;
    }
}

fn raycast(
    spatial_query: SpatialQuery,
    terrain: Query<(&Parent, &Terrain)>,
    main_camera: Query<(&GlobalTransform, &Camera), With<MainCamera>>,
    mouse_position: Res<CurrentMousePos>,
    mut terrain_raycast: ResMut<TerrainRaycast>,
) {
    let (t, camera) = main_camera
        .get_single()
        .expect("only one main camera allowed");
    let Ray3d { origin, direction } = camera
        .viewport_to_world(t, mouse_position.position)
        .unwrap_or(Ray3d::new(t.translation(), *t.forward()));

    terrain_raycast.hit_point = spatial_query
        .cast_ray_predicate(
            origin,
            direction,
            f32::MAX,
            true,
            SpatialQueryFilter::default(),
            &|entity| terrain.get(entity).is_ok(),
        )
        .map(
            |RayHitData {
                 time_of_impact,
                 normal,
                 entity,
             }| {
                let position = origin + time_of_impact * direction;

                let (ref parent, _) = terrain
                    .get(entity)
                    .expect(format!("terrain {} does not have parent", entity).as_str());

                HitPoint {
                    position,
                    normal,
                    entity: ***parent,
                }
            },
        );
}

fn terrain_gizmo(mut gizmos: Gizmos, terrain_raycast: Res<TerrainRaycast>) {
    if let Some(HitPoint {
        position, normal, ..
    }) = terrain_raycast.hit_point
    {
        gizmos.arrow(position, position + normal, WHITE);
    }
}
