use bevy::prelude::*;
use std::collections::HashMap;

use crate::{board::components::Exit, vectors::Vector2Int};

use super::{
    components::{Position, Tile},
    CurrentBoard,
};

pub fn spawn_map(mut commands: Commands, mut board: ResMut<CurrentBoard>) {
    info!("Spawning map");

    board.width = 5;
    board.height = 4;
    board.start = Vector2Int::ZERO;
    board.exit = Vector2Int::new(2, 1);
    board.tiles = HashMap::new();

    // Spawn a grid of tiles of arbitrary size for now
    for x in 0..board.width {
        for y in 0..board.height {
            let v = Vector2Int::new(x, y);
            let tile = commands.spawn((Position { v }, Tile)).id();
            if v == board.exit {
                commands.entity(tile).insert(Exit);
            }
            board.tiles.insert(v, tile);
        }
    }

    // Remove some tiles to make the board solvable
    let remove_vector = &Vector2Int::new(1, 2);
    let tile_entity = board.tiles.get(remove_vector).unwrap();
    commands.entity(*tile_entity).despawn();
    board.tiles.remove(remove_vector);

    let remove_vector = &Vector2Int::new(4, 3);
    let tile_entity = board.tiles.get(remove_vector).unwrap();
    commands.entity(*tile_entity).despawn();
    board.tiles.remove(remove_vector);
}
