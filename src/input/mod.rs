use bevy::prelude::*;

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentMousePos::default())
            .add_systems(PreUpdate, update_current_mouse_pos);
    }
}
#[derive(Resource, Default)]
pub struct CurrentMousePos {
    pub position: Vec2,
}

fn update_current_mouse_pos(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut current_mouse_pos: ResMut<CurrentMousePos>,
) {
    if let Some(&CursorMoved { position, .. }) = cursor_moved_events.read().last() {
        current_mouse_pos.position = position;
    }
}
