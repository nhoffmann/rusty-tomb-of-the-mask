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

pub fn spawn_player(mut commands: Commands, board: Res<CurrentBoard>) {
    commands.spawn((
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position { v: board.start },
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

    loop {
        let next_position = player_position.v + dir;

        if !board.tile_on_board(next_position) {
            break;
        }

        debug!("Moving player into direction {:?}", dir);
        player_position.v = next_position;

        // if new position is the exit tile, we win
        // TODO properly transition the game into a winning state
        if next_position == board.exit {
            info!("Player reached the exit!");
            break;
        }
    }
}
