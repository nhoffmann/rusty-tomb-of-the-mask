use bevy::prelude::*;

use crate::board::systems::spawn_map;
use crate::states::MainState;
use assets::AsciiSpriteSheet;

pub const TILE_SIZE: f32 = 32.;

mod assets;
mod tiles;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsciiSpriteSheet>().add_systems(
            OnEnter(MainState::Game),
            tiles::spawn_tile_renderer.after(spawn_map),
        );
    }
}
