use bevy::prelude::*;

use crate::{
    board::components::Position, pieces::components::Piece, states::MainState, vectors::Vector2Int,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position {
            v: Vector2Int { x: 0, y: 0 },
        },
    ));
}
