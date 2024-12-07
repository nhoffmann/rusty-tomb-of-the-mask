use bevy::prelude::*;

use crate::{
    actions::Actions,
    board::{components::Position, CurrentBoard},
    pieces::components::Piece,
    states::MainState,
    vectors::Vector2Int,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player)
            .add_systems(Update, move_player);
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

fn move_player(
    actions: Res<Actions>,
    board: Res<CurrentBoard>,
    mut player_position_query: Query<&mut Position, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }

    let mut player_position = player_position_query.get_single_mut().unwrap();
    let dir = actions.player_movement.unwrap_or_default();

    if dir == Vector2Int::ZERO {
        return;
    }

    let mut new_position = player_position.v + dir;
    debug!("Moving player into direction {:?}", dir);

    // the while loop is bad, we need to move the player in the direction until we hit a wall one step at a time
    // we can then check in a different system whether the player has reached the exit
    while board.tile_on_board(new_position) {
        new_position = new_position + dir;

        // if new position is an exit tile, we win
    }

    player_position.v = new_position - dir;
}
