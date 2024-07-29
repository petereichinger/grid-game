pub mod cell;
mod component;
pub mod corner;
pub mod mesh_builder;

use bevy::prelude::*;
pub use component::HeightGrid;
use mesh_builder::MeshBuilderPlugin;

pub struct HeightGridPlugin;

impl Plugin for HeightGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshBuilderPlugin);
    }
}
