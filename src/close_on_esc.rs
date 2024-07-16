use bevy::prelude::*;
use bevy::window::{ClosingWindow, Window};

pub fn close_on_esc(
    mut commands: Commands,
    window: Query<(Entity, &Window)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let (entity, _) = window.single();
    if keys.just_pressed(KeyCode::Escape) {
        commands.entity(entity).insert(ClosingWindow);
    }
}
