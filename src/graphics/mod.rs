use bevy::prelude::*;

use crate::board::components::Position;
use crate::board::systems::spawn_map;
use crate::player::spawn_player;
use crate::states::MainState;
use assets::AsciiSpriteSheet;

pub const TILE_SIZE: f32 = 32.;
pub const TILE_Z: f32 = 0.;
pub const PIECE_Z: f32 = 10.;
pub const PIECE_SPEED: f32 = 30.;
pub const POSITION_TOLERANCE: f32 = 0.1;

mod assets;
mod pieces;
mod tiles;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsciiSpriteSheet>()
            .add_systems(
                OnEnter(MainState::Game),
                (
                    tiles::spawn_tile_renderer.after(spawn_map),
                    tiles::spawn_exit_renderer.after(spawn_map),
                    pieces::spawn_piece_renderer.after(spawn_player),
                ),
            )
            .add_systems(FixedUpdate, pieces::update_piece_position);
    }
}

fn get_world_position(position: &Position, z: f32) -> Vec3 {
    Vec3::new(
        position.v.x as f32 * TILE_SIZE,
        position.v.y as f32 * TILE_SIZE,
        z,
    )
}
