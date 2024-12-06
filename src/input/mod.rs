use bevy::prelude::*;

use crate::{board::components::Position, player::Player, vectors::Vector2Int};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_position);
    }
}

const DIR_KEY_MAPPING: [(KeyCode, Vector2Int); 8] = [
    (KeyCode::KeyW, Vector2Int::UP),
    (KeyCode::KeyA, Vector2Int::LEFT),
    (KeyCode::KeyS, Vector2Int::DOWN),
    (KeyCode::KeyD, Vector2Int::RIGHT),
    (KeyCode::ArrowUp, Vector2Int::UP),
    (KeyCode::ArrowLeft, Vector2Int::LEFT),
    (KeyCode::ArrowDown, Vector2Int::DOWN),
    (KeyCode::ArrowRight, Vector2Int::RIGHT),
];

fn player_position(
    keys: ResMut<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Position, With<Player>>,
) {
    let Ok(mut position) = player_query.get_single_mut() else {
        return;
    };
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.just_pressed(key) {
            continue;
        }
        position.v += dir;
    }
}
